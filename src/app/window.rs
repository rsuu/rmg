use crate::*;

use eyre::OptionExt;
use rgb::RGBA8;
use softbuffer::{Context, Surface};
use std::{
    collections::VecDeque,
    num::NonZeroU32,
    rc::Rc,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
use winit::{
    dpi::{LogicalSize, PhysicalPosition},
    event::{Event, KeyEvent, MouseButton, MouseScrollDelta, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
    keyboard::{KeyCode, PhysicalKey},
    monitor::{MonitorHandle, VideoMode},
    window::{Window, WindowBuilder},
};

pub struct App {
    pub init: Init,
    pub env: EnvData,
    pub ext: Ext,

    pub elems: Vec<Page>, // TODO: world
    pub world: World,

    pub canvas: Canvas,
    pub layout: Layout,
    pub config: Config,
    pub action: Action,
    pub gestures: Gesture,

    pub event_info: EventInfo,
    pub surface: Surface<Rc<Window>, Rc<Window>>,
    pub monitor: Option<MonitorHandle>,
}

pub struct Ext {
    pub pool: Pool,
    pub data: Arc<DataType>, // TODO: mv to App
}

#[derive(Debug, Default)]
pub struct Init {
    pub page_size: Size,
}

pub struct EnvData {
    flag_gesture: bool,
    flag_fullscreen: bool,

    loop_dur: Duration,
}

pub struct EventInfo {
    mouse_pos: Vec2,
}

impl App {
    pub fn start(config: Config) -> eyre::Result<()> {
        let (mut app, event_loop) = Self::new(config)?;

        app.window().set_ime_allowed(true);
        if let Some(name) = app.config.once.record_gesture_name.as_ref() {
            app.run_record_gesture(event_loop)?;
        } else {
            app.run(event_loop)?;
        }

        Ok(())
    }

    fn window(&self) -> &Rc<Window> {
        self.surface.window()
    }

    fn new(config: Config) -> eyre::Result<(Self, EventLoop<()>)> {
        let gestures = Gesture::new(config.gestures.data_path.as_str())?;

        let (data, pool, elems);
        let canvas = {
            let path = config.app.target.as_path();
            data = DataType::new(path)?;
            let empty_pages = data.gen_empty_pages(config.misc.padding_filename as usize)?;

            pool = Pool::new(empty_pages.clone());
            elems = empty_pages;

            Canvas::new(&config)?
        };
        tracing::info!("Canvas");

        let event_loop = EventLoop::new()?;
        let window = {
            let size = canvas.size();
            let size = LogicalSize::new(size.width(), size.height());

            Rc::new(WindowBuilder::new().with_title("rmg").build(&event_loop)?)
        };
        tracing::info!("Winit");

        let context = Context::new(window.clone()).or_else(|e| Err(eyre::eyre!("{e:#?}")))?;
        let surface =
            Surface::new(&context, window.clone()).or_else(|e| Err(eyre::eyre!("{e:#?}")))?;
        tracing::info!("Softbuffer");

        Ok((
            Self {
                init: Default::default(),
                action: Default::default(),
                env: EnvData::new(),
                event_info: EventInfo::new(),
                world: World::new(elems.clone()),
                elems, // TODO: rm
                ext: Ext {
                    pool,
                    data: Arc::new(data),
                },
                monitor: window.current_monitor(),
                layout: config.canvas.layout.clone(),
                canvas,
                surface,
                config,
                gestures,
            },
            event_loop,
        ))
    }

    fn run(&mut self, event_loop: EventLoop<()>) -> eyre::Result<()> {
        event_loop.run(move |event, elwt| {
            elwt.set_control_flow(ControlFlow::wait_duration(self.env.loop_dur));

            self.event_loop(event, elwt).unwrap();
        })?;

        Ok(())
    }

    fn run_record_gesture(&mut self, event_loop: EventLoop<()>) -> eyre::Result<()> {
        let gest_name = self.config.once.record_gesture_name.as_ref().unwrap();

        event_loop.run(move |event, elwt| {
            elwt.set_control_flow(ControlFlow::wait_duration(self.env.loop_dur));

            todo!();
        })?;

        Ok(())
    }

    fn event_loop(
        &mut self,
        event: Event<()>,
        elwt: &EventLoopWindowTarget<()>,
    ) -> eyre::Result<()> {
        let window = self.window();

        match event {
            Event::AboutToWait => {
                window.request_redraw();
            }

            Event::WindowEvent { window_id, event } if window_id == window.id() => {
                // dbg!(&event);
                self.on_window_event(elwt, event)?;
            }

            _ => {}
        }

        Ok(())
    }

    fn flush(&mut self) -> eyre::Result<()> {
        let new_size = {
            let size = self.window().inner_size();
            Size::new(size.width as f32, size.height as f32)
        };

        if new_size != self.canvas.size() {
            // #[cold]
            if self.init.page_size.is_zero() {
                // ???
                // let screen = window.current_monitor().unwrap();
                // screen.size();
                //
                // let percent_w = 0.6;
                // let percent_h = 1.0;

                self.init.page_size = new_size;
            }

            self.canvas.resize(new_size);

            let Size { width, height } = self.canvas.size();
            self.surface
                .resize(
                    NonZeroU32::new(width as u32).ok_or_eyre("NonZeroU32")?,
                    NonZeroU32::new(height as u32).ok_or_eyre("NonZeroU32")?,
                )
                .or_else(|e| Err(eyre::eyre!("{e:#?}")))?;
        }

        self.render()?;

        let mut buffer = self
            .surface
            .buffer_mut()
            .or_else(|e| Err(eyre::eyre!("{e:#?}")))?;

        // buffer.swap_with_slice(&mut self.canvas.buffer);
        buffer.copy_from_slice(self.canvas.buffer.vec.as_slice());
        buffer.present().or_else(|e| Err(eyre::eyre!("{e:#?}")))?;

        // dbg!(&(window.inner_size(), self.canvas.size, window.outer_size(),));
        // dbg!("flush");

        Ok(())
    }

    fn on_window_event(
        &mut self,
        elwt: &EventLoopWindowTarget<()>,
        e: WindowEvent,
    ) -> eyre::Result<()> {
        // TODO:
        // [ ](window) full/mini/float screen
        // [ ](mouse) pick img
        match e {
            WindowEvent::RedrawRequested => self.flush()?,

            // ===== Mouse =====
            WindowEvent::CursorMoved { position, .. } => 's: {
                let sf = self.window().scale_factor();
                tracing::trace!(dpi.scale = sf);

                self.on_cursor_moved(sf, position)?;
            }

            WindowEvent::MouseWheel { delta, .. } => self.on_mousewheel(delta)?,

            e @ WindowEvent::MouseInput { .. } => self.on_mouse(e)?,

            // ===== Keyboard =====
            WindowEvent::KeyboardInput { event, .. } => {
                self.on_keyboard(event, elwt)?;
            }

            WindowEvent::ModifiersChanged(new) => {}

            WindowEvent::Resized(new_size) => {
                // size = new_size.to_logical(1.0);
            }

            WindowEvent::CloseRequested => self.on_exit(elwt)?,

            _ => {}
        }

        Ok(())
    }

    fn on_cursor_moved(
        &mut self,
        sf: f64,
        PhysicalPosition { x, y }: PhysicalPosition<f64>,
    ) -> eyre::Result<()> {
        if !self.env.flag_gesture {
            return Ok(());
        }

        let sf = sf as f32;
        let origin = Vec2::new(x as f32, y as f32).scale(sf, sf);

        self.event_info.mouse_pos = origin;

        // dbg!(y, window.inner_size().height);
        match self.action {
            Action::Gesture { ref mut path, .. } => {
                path.push(origin);
            }

            Action::View => {
                let red = RGBA8::new(255, 0, 0, 255);
                self.action = Action::Gesture {
                    fill: red,
                    path: vec![],
                    stroke_width: 4.0,
                };
            }

            _ => {}
        }

        Ok(())
    }

    // Single-Left-Click: Pick frame
    // Single-Left-Drag : Drag frame
    // Double-Left-Drag : ???
    // Double-Left-Drag : ???
    //
    // Single-Right-Click: ???
    // Single-Right-Drag : ???
    // Double-Right-Drag : ???
    // Double-Right-Drag : ???
    fn on_mouse(&mut self, e: WindowEvent) -> eyre::Result<()> {
        let WindowEvent::MouseInput {
            device_id,
            state,
            button,
        } = e
        else {
            return Ok(());
        };

        match button {
            MouseButton::Left => {}
            _ => {}
        }

        Ok(())
    }

    fn on_mousewheel(&mut self, e: MouseScrollDelta) -> eyre::Result<()> {
        let MouseScrollDelta::LineDelta(.., y) = e else {
            return Ok(());
        };

        // tracing::trace!(y = y);

        match self.action {
            Action::View => {}

            _ => return Ok(()),
        }

        match &mut self.layout {
            Layout::Single {
                mouse_pos,
                flag_scroll,
                dire,
                cur_zoom,
                ref max_zoom,
                ref min_zoom,
                ..
            } => 's: {
                *mouse_pos = self.event_info.mouse_pos;
                *flag_scroll = true;

                if y < 0.0 && *cur_zoom < *max_zoom {
                    *dire = 1.0;
                    *cur_zoom += 1;
                } else if y > 0.0 && *cur_zoom > *min_zoom {
                    *dire = -1.0;
                    *cur_zoom -= 1;
                } else {
                    *flag_scroll = false;
                }
            }

            Layout::Vertical { .. } => {
                if y < 0.0 {
                    self.canvas.move_down();
                } else if y > 0.0 {
                    self.canvas.move_up();
                }
            }

            _ => {}
        }

        Ok(())
    }

    fn on_keyboard(
        &mut self,
        KeyEvent {
            physical_key,
            state,
            ..
        }: KeyEvent,
        elwt: &EventLoopWindowTarget<()>,
    ) -> eyre::Result<()> {
        if !state.is_pressed() {
            return Ok(());
        }

        match physical_key {
            PhysicalKey::Code(KeyCode::KeyJ) | PhysicalKey::Code(KeyCode::ArrowDown) => {
                // dbg!("move_down");
                self.canvas.move_down();
            }

            PhysicalKey::Code(KeyCode::KeyK) | PhysicalKey::Code(KeyCode::ArrowUp) => {
                // dbg!("move_up");
                self.canvas.move_up();
            }

            PhysicalKey::Code(KeyCode::KeyH) | PhysicalKey::Code(KeyCode::ArrowLeft) => {
                // dbg!("move_left");
                self.canvas.move_left();
            }

            PhysicalKey::Code(KeyCode::KeyL) | PhysicalKey::Code(KeyCode::ArrowRight) => {
                // dbg!("move_right");
                self.canvas.move_right();
            }

            PhysicalKey::Code(KeyCode::KeyF) => self.on_keyboard_fullscreen()?,

            PhysicalKey::Code(KeyCode::KeyG) => self.on_keyboard_gesture()?,

            PhysicalKey::Code(KeyCode::KeyQ) | PhysicalKey::Code(KeyCode::Escape) => {
                self.on_exit(elwt)?
            }

            _ => {}
        }

        self.window().reset_dead_keys();

        Ok(())
    }

    fn on_exit(&mut self, elwt: &EventLoopWindowTarget<()>) -> eyre::Result<()> {
        tracing::info!("exit");

        self.gestures.save()?;

        elwt.exit();

        Ok(())
    }

    fn on_keyboard_gesture(&mut self) -> eyre::Result<()> {
        tracing::trace!(flag_gesture = self.env.flag_gesture);

        if !self.env.flag_gesture {
            self.env.flag_gesture = true;

            return Ok(());
        } else {
            self.env.flag_gesture = false;
        }

        let Action::Gesture { path, .. } = &self.action else {
            return Ok(());
        };

        // TODO: record
        let mut new_temp = Vec::with_capacity(path.len() * 2 * 4);
        for p in path.iter() {
            // let p = p.normalized();
            let x = p.x.to_be_bytes();
            let y = p.y.to_be_bytes();

            new_temp.extend_from_slice(&x);
            new_temp.extend_from_slice(&y);
        }
        // let mut f = File::create("ring.gest")?;
        // f.write_all(&new_temp)?;

        if let Ok(name) = self.gestures.matches(&path, self.config.gestures.min_score) {
            // TODO: name -> action
            dbg!(&name);
        }

        self.action = Action::View;

        Ok(())
    }

    fn on_keyboard_fullscreen(&mut self) -> eyre::Result<()> {
        // if wasm { return }

        // REFS: https://github.com/rust-windowing/winit/issues/717
        if self.env.flag_fullscreen {
            self.window().set_fullscreen(None);

            self.env.flag_fullscreen = false;
        } else {
            let mut modes = self.monitor.as_ref().unwrap().video_modes();
            let first = modes.nth(0).unwrap();

            tracing::debug!("VideoMode = {:?}", &first);

            // window.set_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));
            self.window()
                .set_fullscreen(Some(winit::window::Fullscreen::Exclusive(first)));

            self.env.flag_fullscreen = true;
        }

        tracing::debug!(self.env.flag_fullscreen);

        Ok(())
    }
}

impl EnvData {
    fn new() -> Self {
        Self {
            flag_gesture: false,
            flag_fullscreen: false,

            // 90FPS
            loop_dur: Duration::from_millis(1000 / 90),
        }
    }
}

impl EventInfo {
    pub fn new() -> Self {
        Self {
            mouse_pos: Vec2::default(),
        }
    }
}
