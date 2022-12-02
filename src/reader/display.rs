use crate::{
    archive::ArchiveType,
    config::rsconf::Config,
    img::size::MetaSize,
    reader::{
        keymap::{self, Map},
        scroll::{Scroll, State},
        view::{Buffer, Page},
        window::Canvas,
    },
    utils::err::Res,
};
//use emeta::meta;

use log::{debug, info};
use std::{
    path::PathBuf,
    sync::{Arc, RwLock},
};

/// display images
pub async fn cat_img(
    config: &Config,
    page_list: Vec<Page>,
    meta_size: MetaSize<u32>,
    //_metadata: &Option<meta::MetaData>,
    path: &str,
    archive_type: ArchiveType,
) -> Res<()> {
    let screen_size = meta_size.screen;
    let window_size = meta_size.window;
    let max_ram = window_size.width as usize * window_size.height as usize;
    let limit_next = max_ram * 8;
    let limit_prev = max_ram * 2;

    let step = config.base.step as usize;
    let filter = config.base.filter;
    let keymaps = keymap::KeyMap::new();

    let mut buf = Scroll {
        buffer: Buffer::new(), // buffer
        max_ram,

        head: 0,
        tail: 0,
        rng: 0,

        len: 0,

        archive_path: PathBuf::from(path),
        archive_type,

        mode: Map::Stop,                           // keymap
        page_end: page_list.len(),                 //
        page_list,                                 //
        screen_size,                               //
        y_step: max_ram / step,                    // drop 1/step part of image once
        x_step: window_size.width as usize / step, //
        window_position: (0, 0),                   //
        window_size,                               //

        page_load_list: Vec::new(),
        filter, //
    };
    let mut canvas = Canvas::new(window_size.width as usize, window_size.height as usize);

    for_minifb(config, &mut buf, &mut canvas, &keymaps).await;

    info!("---EXIT---");

    Ok(())
}

pub async fn for_minifb(
    config: &Config,
    buf: &mut Scroll,
    canvas: &mut Canvas,
    keymaps: &[keymap::KeyMap],
) {
    let arc_state = Arc::new(RwLock::new(State::Nothing));
    let arc_page: Arc<RwLock<Page>> = Arc::new(RwLock::new(Page::null()));

    buf.init();

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

                // BUG: Miss Key::Escape
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

        std::thread::sleep(std::time::Duration::from_millis(40));
    }
}
