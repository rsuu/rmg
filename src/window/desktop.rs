use crate::*;

use eyre::OptionExt;
use rgb::RGBA8;
use std::{
    num::NonZeroU32,
    rc::Rc,
    time::{Duration, Instant},
};
use winit::{
    dpi::{LogicalSize, PhysicalPosition},
    event::{Event, KeyEvent, MouseButton, MouseScrollDelta, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowBuilder},
};

pub fn main(config: Config) -> eyre::Result<()> {
    let record_gesture_name = config.once.record_gesture_name.clone();

    let (mut app, window, event_loop) = App::new(config)?;

    if let Some(name) = record_gesture_name {
        app.run_record_gesture(&window, event_loop)?;
    } else {
        app.run(&window, event_loop)?;
    }

    Ok(())
}

pub struct App {
    canvas: Canvas,
    surface: softbuffer::Surface<Rc<Window>, Rc<Window>>,

    ev: EnvVal,
    gestures: Gesture,
}

struct EnvVal {
    flag_flush: bool,
    flag_gesture: bool,
    flag_fullscreen: bool,

    loop_dur: Duration,
}

impl App {
    fn new(config: Config) -> eyre::Result<(Self, Rc<Window>, EventLoop<()>)> {
        let canvas = {
            let path = config.app.target.as_path();
            let file = DataType::new(path)?;

            Canvas::new(config, file)?
        };
        tracing::info!("Canvas");

        let event_loop = EventLoop::new()?;
        let window = {
            let size = canvas.size;
            let size = LogicalSize::new(size.width(), size.height());

            Rc::new(
                WindowBuilder::new()
                    .with_inner_size(size)
                    // .with_name(general, instance)
                    .with_max_inner_size(size)
                    .with_min_inner_size(size)
                    .build(&event_loop)?,
            )
        };
        tracing::info!("Winit");

        let context =
            softbuffer::Context::new(window.clone()).or_else(|e| Err(eyre::eyre!("{e:#?}")))?;
        let surface = softbuffer::Surface::new(&context, window.clone())
            .or_else(|e| Err(eyre::eyre!("{e:#?}")))?;
        tracing::info!("Softbuffer");

        let gestures = canvas.config.app.gestures_zip.clone();
        let gestures = Gesture::new(gestures)?;

        Ok((
            Self {
                ev: EnvVal::new(),
                canvas,
                surface,
                gestures,
            },
            window,
            event_loop,
        ))
    }

    fn run(&mut self, window: &Rc<Window>, event_loop: EventLoop<()>) -> eyre::Result<()> {
        // TODO: render thread
        // let thread_flush = thread::spawn(|| {});

        event_loop.run(move |event, elwt| {
            elwt.set_control_flow(ControlFlow::wait_duration(self.ev.loop_dur));
            // elwt.set_control_flow(ControlFlow::Poll);

            self.event_loop(window, event, elwt).unwrap();
        })?;

        Ok(())
    }

    fn run_record_gesture(
        &mut self,
        window: &Rc<Window>,
        event_loop: EventLoop<()>,
    ) -> eyre::Result<()> {
        event_loop.run(move |event, elwt| {
            elwt.set_control_flow(ControlFlow::wait_duration(self.ev.loop_dur));

            todo!();
        })?;

        Ok(())
    }

    fn event_loop(
        &mut self,
        window: &Rc<Window>,
        event: Event<()>,
        elwt: &EventLoopWindowTarget<()>,
    ) -> eyre::Result<()> {
        match event {
            Event::AboutToWait => {
                // if self.ev.flag_flush {
                // }
                window.request_redraw();
            }

            Event::WindowEvent { window_id, event } if window_id == window.id() => {
                // dbg!(&event);
                self.on_window_event(window, elwt, event)?;
            }

            _ => {
                self.ev.flag_flush = false;
            }
        }

        Ok(())
    }

    fn on_flush(&mut self, window: &Rc<Window>) -> eyre::Result<()> {
        // TODO: no resize
        let new_size = {
            let size = window.inner_size();
            Size::new(size.width as f32, size.height as f32)
        };

        if new_size != self.canvas.size {
            self.canvas.resize(new_size);

            let Size { width, height } = self.canvas.size;
            self.surface
                .resize(
                    NonZeroU32::new(width as u32).ok_or_eyre("NonZeroU32")?,
                    NonZeroU32::new(height as u32).ok_or_eyre("NonZeroU32")?,
                )
                .or_else(|e| Err(eyre::eyre!("{e:#?}")))?;
        }

        let mut buffer = self
            .surface
            .buffer_mut()
            .or_else(|e| Err(eyre::eyre!("{e:#?}")))?;

        self.canvas.draw()?;

        // buffer.swap_with_slice(&mut self.canvas.buffer);
        buffer.copy_from_slice(&self.canvas.buffer);
        buffer.present().or_else(|e| Err(eyre::eyre!("{e:#?}")))?;

        // dbg!(&(window.inner_size(), self.canvas.size, window.outer_size(),));
        // dbg!("flush");

        Ok(())
    }

    fn on_window_event(
        &mut self,
        window: &Rc<Window>,
        elwt: &EventLoopWindowTarget<()>,
        e: WindowEvent,
    ) -> eyre::Result<()> {
        // TODO:
        // [ ](window) fullscreen/mini/floating
        // [ ](mouse) pick img
        // [ ](mouse) gesture
        match e {
            WindowEvent::RedrawRequested => self.on_flush(window)?,

            // ===== Mouse =====
            WindowEvent::CursorMoved { position, .. } => 's: {
                if !self.ev.flag_gesture {
                    break 's;
                }

                let sf = window.scale_factor();
                tracing::trace!(dpi.scale = sf);

                self.on_cursor_moved(sf, position)?;
            }

            WindowEvent::MouseWheel { delta, .. } => self.on_mousewheel(delta)?,

            e @ WindowEvent::MouseInput { .. } => self.on_mouse(e)?,

            // ===== Keyboard =====
            WindowEvent::KeyboardInput { event, .. } => {
                self.on_keyboard(&window, event, elwt)?;

                window.reset_dead_keys();
            }

            WindowEvent::ModifiersChanged(new) => {}

            WindowEvent::Resized(new_size) => {
                // size = new_size.to_logical(1.0);
            }

            WindowEvent::CloseRequested => self.on_exit(elwt)?,

            _ => {
                self.ev.flag_flush = false;
            }
        }

        Ok(())
    }

    fn on_cursor_moved(
        &mut self,
        sf: f64,
        PhysicalPosition { x, y }: PhysicalPosition<f64>,
    ) -> eyre::Result<()> {
        let x = (x / sf) as f32;
        let y = (y / sf) as f32;
        let x = x.clamp(0.0, self.canvas.size.width() - 1.0);
        let y = y.clamp(0.0, self.canvas.size.height() - 1.0);

        let origin = Vec2::new(x, y);

        // dbg!(y, window.inner_size().height);
        match self.canvas.action {
            Action::Gesture { ref mut path, .. } => {
                path.push(origin);
            }

            Action::View => {
                let red = RGBA8::new(255, 0, 0, 255);
                self.canvas.action = Action::Gesture {
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

        match self.canvas.action {
            Action::View => {}

            _ => return Ok(()),
        }

        match &mut self.canvas.config.canvas.layout {
            // FIXME: get mouse pos
            Layout::Single { zoom, mouse_pos } => {
                // zoom at mouse position
                if y < 0.0 && *zoom < 10.0 {
                    *zoom += 0.1;
                } else if y > 0.0 && *zoom > 0.2 {
                    *zoom -= 0.1;
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
        window: &Rc<Window>,
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

            PhysicalKey::Code(KeyCode::KeyF) => self.on_keyboard_fullscreen(window)?,

            PhysicalKey::Code(KeyCode::KeyG) => self.on_keyboard_gesture()?,

            PhysicalKey::Code(KeyCode::KeyQ) | PhysicalKey::Code(KeyCode::Escape) => {
                self.on_exit(elwt)?
            }

            _ => {}
        }

        Ok(())
    }

    fn on_exit(&mut self, elwt: &EventLoopWindowTarget<()>) -> eyre::Result<()> {
        tracing::info!("exit");

        self.gestures.save()?;

        elwt.exit();

        Ok(())
    }

    fn on_keyboard_gesture(&mut self) -> eyre::Result<()> {
        tracing::trace!(flag_gesture = self.ev.flag_gesture);

        if !self.ev.flag_gesture {
            self.ev.flag_gesture = true;

            return Ok(());
        } else {
            self.ev.flag_gesture = false;
        }

        let Action::Gesture { path, .. } = &self.canvas.action else {
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

        if let Ok(name) = self
            .gestures
            .matches(&path, self.canvas.config.app.min_gesture_score)
        {
            // TODO: name -> action
            dbg!(&name);
        }

        self.canvas.action = Action::View;

        Ok(())
    }

    fn on_keyboard_fullscreen(&mut self, window: &Rc<Window>) -> eyre::Result<()> {
        if self.ev.flag_fullscreen {
            tracing::debug!("unfullscreen");
            window.set_fullscreen(None);

            self.ev.flag_fullscreen = false;
        } else {
            tracing::debug!("fullscreen");
            window.set_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));

            self.ev.flag_fullscreen = true;
        }

        Ok(())
    }
}

impl EnvVal {
    fn new() -> Self {
        Self {
            flag_flush: true,
            flag_gesture: false,
            flag_fullscreen: false,

            loop_dur: Duration::from_millis(30),
        }
    }
}
