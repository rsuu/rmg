use crate::{Canvas, Config, DataType, Layout, PathBuf, Size, FPS};

use std::{num::NonZeroU32, rc::Rc, thread, time::Duration};
use winit::{
    dpi::LogicalSize,
    event::{Event, KeyEvent, MouseScrollDelta, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::WindowBuilder,
};

pub fn main(config: Config) -> eyre::Result<()> {
    let mut canvas = {
        let path = PathBuf::from(config.path());
        let file = DataType::new(path.as_path()).unwrap();

        Canvas::new(config, file)?
    };
    dbg!("INFO: canvas");

    let event_loop = EventLoop::new().unwrap();
    let window = {
        let size = canvas.size;
        let size = LogicalSize::new(size.width(), size.height());

        Rc::new(
            WindowBuilder::new()
                .with_inner_size(size)
                // TODO: resize
                .with_max_inner_size(size)
                .with_min_inner_size(size)
                .build(&event_loop)
                .unwrap(),
        )
    };
    dbg!("INFO: winit");

    let context = softbuffer::Context::new(window.clone()).unwrap();
    let mut surface = softbuffer::Surface::new(&context, window.clone()).unwrap();
    dbg!("INFO: softbuffer");

    // TODO: floating window
    let mut is_fullscreen = false;
    event_loop
        .run(move |event, elwt| {
            elwt.set_control_flow(ControlFlow::Wait);

            // TODO:
            // [ ](window) fullscreen/mini
            // [ ](mouse)  pick img
            match event {
                Event::WindowEvent {
                    window_id,
                    event: WindowEvent::RedrawRequested,
                } if window_id == window.id() => {
                    let new_size = {
                        let size = window.inner_size();
                        Size::new(size.width as f32, size.height as f32)
                    };

                    if new_size != canvas.size {
                        canvas.resize(new_size);

                        let Size { width, height } = canvas.size;
                        surface
                            .resize(
                                NonZeroU32::new(width as u32).unwrap(),
                                NonZeroU32::new(height as u32).unwrap(),
                            )
                            .unwrap();
                    }
                    let mut buffer = surface.buffer_mut().unwrap();

                    canvas.draw().unwrap();
                    buffer.swap_with_slice(&mut canvas.buffer);
                    buffer.present().unwrap();

                    // dbg!("flush");
                }

                Event::WindowEvent {
                    event: WindowEvent::MouseWheel { delta, .. },
                    ..
                } => match delta {
                    MouseScrollDelta::LineDelta(.., y) => {
                        if y == -1.0 {
                            canvas.move_down();
                        } else if y == 1.0 {
                            canvas.move_up();
                        }

                        // dbg!(&(x, y));
                    }
                    _ => {}
                },

                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::KeyboardInput {
                        event: KeyEvent { physical_key, .. },
                        ..
                    } => match physical_key {
                        PhysicalKey::Code(KeyCode::KeyJ)
                        | PhysicalKey::Code(KeyCode::ArrowDown) => {
                            // dbg!("move_down");
                            canvas.move_down();
                        }

                        PhysicalKey::Code(KeyCode::KeyK) | PhysicalKey::Code(KeyCode::ArrowUp) => {
                            // dbg!("move_up");
                            canvas.move_up();
                        }

                        PhysicalKey::Code(KeyCode::KeyH)
                        | PhysicalKey::Code(KeyCode::ArrowLeft) => {
                            // dbg!("move_left");
                            canvas.move_left();
                        }

                        PhysicalKey::Code(KeyCode::KeyL)
                        | PhysicalKey::Code(KeyCode::ArrowRight) => {
                            // dbg!("move_right");
                            canvas.move_right();
                        }

                        PhysicalKey::Code(KeyCode::KeyF) => {
                            if !is_fullscreen {
                                // dbg!("fullscreen");
                                window.set_fullscreen(Some(winit::window::Fullscreen::Borderless(
                                    None,
                                )));
                            } else {
                                // dbg!("unfullscreen");

                                window.set_fullscreen(None);
                            }
                        }

                        PhysicalKey::Code(KeyCode::KeyQ) | PhysicalKey::Code(KeyCode::Escape) => {
                            dbg!("exit");
                            elwt.exit();
                        }
                        _ => {}
                    },

                    WindowEvent::ModifiersChanged(new) => {}

                    WindowEvent::Resized(new_size) => {
                        // size = new_size.to_logical(1.0);
                    }

                    WindowEvent::CloseRequested => {
                        elwt.exit();
                    }

                    _ => {}
                },
                _ => {}
            }

            window.request_redraw();

            let dur = Duration::from_millis((1000.0 / FPS) as u64);
            thread::sleep(dur);
        })
        .unwrap();

    Ok(())
}
