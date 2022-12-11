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

    let mut buf = Render {
        buffer: Buffer::new(),
        buffer_max,
        mem_limit: buffer_max * 5,

        head: 0,
        tail: 0,
        rng: 0,

        len: 0,

        archive_path: PathBuf::from(path),
        archive_type,

        mode: Map::Stop,
        page_end: page_list.len(),
        page_list,
        screen_size,
        y_step: buffer_max / step, // drop 1/step part of image once
        x_step: window_size.width as usize / step,
        window_position: (0, 0),
        window_size,

        page_load_list: Vec::new(),
        filter,
        view_mode: config.base.view_mode,

        page_number: 0,
        page_loading: vec![0; buffer_max * 20],
    };
    let mut canvas = Canvas::new(window_size.width as usize, window_size.height as usize);

    buf.init(); // init

    match buf.view_mode {
        ViewMode::Scroll => {
            for_minifb_scroll(config, &mut buf, &mut canvas, &keymaps);
        }
        ViewMode::Image => {
            for_minifb_image(config, &mut buf, &mut canvas, &keymaps);
        }

        ViewMode::Manga | ViewMode::Comic => {
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
}

pub fn for_minifb_image(
    _config: &Config,
    buf: &mut Render,
    canvas: &mut Canvas,
    keymaps: &[keymap::KeyMap],
) {
    'l1: while canvas.window.is_open() {
        match keymap::match_event(canvas.window.get_keys().iter().as_slice(), keymaps) {
            Map::Exit => {
                println!("EXIT");

                // BUG: Miss Key::Escape
                break 'l1;
            }

            _ => {}
        }

        canvas.flush(&buf.buffer.data[buf.rng..buf.rng + buf.buffer_max]);

        // TODO: gif
        std::thread::sleep(std::time::Duration::from_millis(40));
    }
}

pub fn for_minifb_scroll(
    config: &Config,
    buf: &mut Render,
    canvas: &mut Canvas,
    keymaps: &[keymap::KeyMap],
) {
    let arc_state = Arc::new(RwLock::new(State::Nothing));
    let arc_page: Arc<RwLock<Page>> = Arc::new(RwLock::new(Page::null()));

    let mut time_start = std::time::Instant::now();
    let mut sleep = FPS;

    'l1: while canvas.window.is_open() {
        match keymap::match_event(canvas.window.get_keys().iter().as_slice(), keymaps) {
            Map::Down => {
                buf.move_down(&arc_page, &arc_state);
            }

            Map::Up => {
                buf.move_up(&arc_page, &arc_state);
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
                            buf.move_up(&arc_page, &arc_state);
                        } else if y < 0.0 {
                            buf.move_down(&arc_page, &arc_state);
                        } else {
                        }

                        debug!("mouse_y == {}", y);
                    }
                } else if let Some((_x, y)) = canvas.window.get_scroll_wheel() {
                    if y > 0.0 {
                        buf.move_down(&arc_page, &arc_state);
                    } else if y < 0.0 {
                        buf.move_up(&arc_page, &arc_state);
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
