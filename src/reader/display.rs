use crate::{
    config::rsconf::Config,
    img::size::MetaSize,
    reader::{
        buffer::{Buffer, PageInfo, State},
        keymap::{self, Map},
        window::Canvas,
    },
    utils::{err::Res, types::ArchiveType},
};
//use emeta::meta;

use std::{
    path::PathBuf,
    sync::{Arc, RwLock},
};

/// display images
pub async fn cat_img(
    config: &Config,
    page_list: Vec<PageInfo>,
    meta_size: MetaSize<u32>,
    //_metadata: &Option<meta::MetaData>,
    path: &str,
    archive_type: ArchiveType,
) -> Res<()> {
    let screen_size = meta_size.screen;
    let window_size = meta_size.window;
    let max_bytes = window_size.width as usize * window_size.height as usize;

    let step = config.base.step as usize;
    let filter = config.base.filter;
    let keymaps = keymap::KeyMap::new();

    let mut buf = Buffer {
        bytes: Vec::with_capacity(max_bytes * 4), // buffer
        max_bytes,

        head: 0,
        tail: 0,
        start: 0,
        end: max_bytes,

        archive_path: PathBuf::from(path),
        archive_type,

        mode: Map::Stop,                           // keymap
        page_end: page_list.len(),                 //
        page_list,                                 //
        screen_size,                               //
        y_step: max_bytes / step,                  // drop 1/step part of image once
        x_step: window_size.width as usize / step, //
        window_position: (0, 0),                   //
        window_size,                               //

        filter, //
    };
    let mut canvas = Canvas::new(window_size.width as usize, window_size.height as usize);

    for_minifb(config, &mut buf, &mut canvas, &keymaps).await;

    println!("CLOSE");

    Ok(())
}

pub async fn for_minifb(
    config: &Config,
    buf: &mut Buffer,
    canvas: &mut Canvas,
    keymaps: &[keymap::KeyMap],
) {
    let state_arc = Arc::new(RwLock::new(State::Nothing));
    let color_buffer_arc: Arc<RwLock<Vec<u32>>> = Arc::new(RwLock::new(Vec::new()));

    buf.init();

    'l1: while canvas.window.is_open() {
        // input from keymap
        match keymap::match_event(canvas.window.get_keys().iter().as_slice(), keymaps) {
            Map::Down => {
                buf.move_down(&color_buffer_arc, &state_arc);
            }

            Map::Up => {
                buf.move_up(&color_buffer_arc, &state_arc);
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
                // BUG: Miss Key::Escape
                break 'l1;
            }

            _ => {
                // input from mouse
                if config.base.invert_mouse {
                    if let Some((_x, y)) = canvas.window.get_scroll_wheel() {
                        if y > 0.0 {
                            buf.move_up(&color_buffer_arc, &state_arc);
                        } else if y < 0.0 {
                            buf.move_down(&color_buffer_arc, &state_arc);
                        } else {
                        }

                        log::debug!("mouse_y == {}", y);
                    }
                } else if let Some((_x, y)) = canvas.window.get_scroll_wheel() {
                    if y > 0.0 {
                        buf.move_down(&color_buffer_arc, &state_arc);
                    } else if y < 0.0 {
                        buf.move_up(&color_buffer_arc, &state_arc);
                    } else {
                    }

                    log::debug!("mouse_y == {}", y);
                }
            }
        }

        //log::debug!("Key: {:?}", canvas.window.get_keys().iter().as_slice());
        buf.flush(canvas);

        std::thread::sleep(std::time::Duration::from_millis(40));
    }
}
