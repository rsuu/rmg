use crate::{
    archive::ArchiveType,
    config::rsconf::Config,
    img::size::MetaSize,
    reader::{
        keymap::{self, Map},
        scroll::{Render, State},
        view::{Buffer, Page, ViewMode},
        window::Canvas,
    },
    utils::err::Res,
    FPS,
};
use log::{debug, info};
use std::{
    ops::Sub,
    path::PathBuf,
    sync::{Arc, RwLock},
};

use super::scroll::{self, ExtData};

/// display images
pub fn cat_img(
    config: &Config,
    page_list: Vec<Page>,
    meta_size: MetaSize<u32>,
    //_metadata: &Option<meta::MetaData>,
    path: &str,
    archive_type: ArchiveType,
) -> Res<()> {
    let screen_size = meta_size.screen;
    let window_size = meta_size.window;
    let buffer_max = window_size.width as usize * window_size.height as usize;

    let step = config.base.step as usize;
    let filter = config.base.filter;
    let keymaps = keymap::KeyMap::new();

    let mut buf = Render::new(
        page_list,
        archive_type,
        path,
        buffer_max,
        step,
        screen_size,
        window_size,
        config.base.view_mode,
        filter,
    );
    let mut canvas = Canvas::new(window_size.width as usize, window_size.height as usize);

    buf.init(); // init

    match buf.view_mode {
        // Bit
        ViewMode::Scroll => {
            for_minifb_scroll(config, &mut buf, &mut canvas, &keymaps);
        }

        // Bit OR Anim
        ViewMode::Image => {
            for_minifb_image(config, &mut buf, &mut canvas, &keymaps);
        }

        // Bit OR Anim
        ViewMode::Page => {
            for_minifb_page(config, &mut buf, &mut canvas, &keymaps);
        }
    }

    info!("*** EXIT ***");

    Ok(())
}

pub fn for_minifb_page(
    _config: &Config,
    _buf: &mut Render,
    _canvas: &mut Canvas,
    _keymaps: &[keymap::KeyMap],
) {
    todo!()
}

pub fn for_minifb_image(
    _config: &Config,
    buf: &mut Render,
    canvas: &mut Canvas,
    keymaps: &[keymap::KeyMap],
) {
    let mut time_start = std::time::Instant::now();
    let mut sleep = FPS;

    let end = buf.page_list[0].size();

    buf.buffer.data = vec![0; buf.buffer_max];

    'l1: while canvas.window.is_open() {
        match keymap::match_event(canvas.window.get_keys().iter().as_slice(), keymaps) {
            Map::Down => {
                // scrolling
                if buf.rng + buf.y_step <= end {
                    buf.rng += buf.y_step;
                } else if buf.rng <= end {
                    buf.rng = end - buf.rng;
                } else {
                    unreachable!()
                }
            }

            Map::Up => {
                if buf.rng >= buf.y_step {
                    buf.rng -= buf.y_step;
                } else if buf.rng >= 0 {
                    buf.rng -= buf.rng;
                } else {
                    unreachable!()
                };
            }

            Map::Exit => {
                println!("EXIT");

                // BUG: Miss Key::Escape
                break 'l1;
            }

            _ => {}
        }

        buf.buffer.data = vec![0; buf.buffer_max];
        buf.buffer.data.copy_from_slice(buf.page_list[0].data());
        buf.page_list[0].to_next_frame();

        canvas.flush(&buf.buffer.data[buf.rng..buf.rng + buf.buffer_max]);

        let now = std::time::Instant::now();
        let count = (now - time_start).as_millis() as u64;

        time_start = now;
        sleep = FPS.checked_sub(count / 6).unwrap_or(10);

        std::thread::sleep(std::time::Duration::from_millis(sleep));
    }
}

pub fn for_minifb_scroll(
    config: &Config,
    buf: &mut Render,
    canvas: &mut Canvas,
    keymaps: &[keymap::KeyMap],
) {
    let arc_state = Arc::new(RwLock::new(State::Nothing));
    let arc_extdata = Arc::new(RwLock::new(ExtData::new(
        buf.archive_type.clone(),
        buf.archive_path.clone(),
        buf.screen_size.clone(),
        buf.window_size.clone(),
        buf.filter.clone(),
    ))); // use for resize image

    let arc_state_main = arc_state.clone();
    let arc_extdata_main = arc_extdata.clone();

    let arc_state_thread = arc_state.clone();
    let arc_extdata_thread = arc_extdata.clone();

    // new thread for resize image
    let _thread_for_resize_image = new_thread(arc_state_thread, arc_extdata_thread);

    let mut time_start = std::time::Instant::now();
    let mut sleep = FPS;

    'l1: while canvas.window.is_open() {
        match keymap::match_event(canvas.window.get_keys().iter().as_slice(), keymaps) {
            Map::Down => {
                buf.move_down(&arc_state_main, &arc_extdata_main);
            }

            Map::Up => {
                buf.move_up(&arc_state_main, &arc_extdata_main);
            }

            Map::Reset => {
                todo!()
            }

            Map::FullScreen => {
                todo!()
            }

            Map::Left => {
                buf.move_left();
            }

            Map::Right => {
                buf.move_right();
            }

            Map::Exit => {
                println!("EXIT");

                // TODO: Key::Escape
                break 'l1;
            }

            _ => {
                // input from mouse
                if config.base.invert_mouse {
                    if let Some((_x, y)) = canvas.window.get_scroll_wheel() {
                        if y > 0.0 {
                            buf.move_up(&arc_state_main, &arc_extdata_main);
                        } else if y < 0.0 {
                            buf.move_down(&arc_state_main, &arc_extdata_main);
                        } else {
                        }

                        debug!("mouse_y == {}", y);
                    }
                } else if let Some((_x, y)) = canvas.window.get_scroll_wheel() {
                    if y > 0.0 {
                        buf.move_down(&arc_state_main, &arc_extdata_main);
                    } else if y < 0.0 {
                        buf.move_up(&arc_state_main, &arc_extdata_main);
                    } else {
                    }

                    debug!("mouse_y == {}", y);
                }
            }
        }

        buf.flush(canvas, &arc_state);

        let now = std::time::Instant::now();
        let count = (now - time_start).as_millis() as u64;

        time_start = now;
        sleep = FPS.checked_sub(count / 6).unwrap_or(10);

        std::thread::sleep(std::time::Duration::from_millis(sleep));
    }
}

pub fn new_thread(arc_state: Arc<RwLock<State>>, arc_extdata: Arc<RwLock<ExtData>>) {
    std::thread::spawn(move || {
        loop {
            if let Ok(mut arc_state) = arc_state.try_write() {
                match *arc_state {
                    State::LoadNext | State::LoadPrev => {
                        if let Ok(mut arc_extdata) = arc_extdata.try_write() {
                            let (ty, mut buffer, format) = scroll::load_file(
                                arc_extdata.archive_type.clone(),
                                arc_extdata.path.clone().as_path(),
                                arc_extdata.pos,
                            )
                            .unwrap();

                            let (meta, pts) = scroll::load_img(
                                format,
                                &mut buffer,
                                arc_extdata.screen_size,
                                arc_extdata.window_size,
                            )
                            .unwrap();

                            arc_extdata.page.ty = ty;
                            arc_extdata.page.resize = meta.fix;
                            arc_extdata.page.pts = pts;

                            let f = arc_extdata.filter.clone();

                            scroll::resize_page(&mut arc_extdata.page, &mut buffer, &meta, &f);

                            *arc_state = match *arc_state {
                                State::LoadPrev => State::DonePrev,
                                State::LoadNext => State::DoneNext,
                                _ => {
                                    unreachable!()
                                }
                            }
                        } else {
                            // wait
                        }
                    }
                    _ => {}
                }
            } else {
                // wait
            }

            // limit CPU usage
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    });
}
