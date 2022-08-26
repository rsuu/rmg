use crate::{
    color::format::PixelFormat,
    config::rsconf::Config,
    img::{
        resize,
        size::{MetaSize, Size},
    },
    reader::{
        buffer::{Buffer, PageInfo, State},
        keymap::{self, Map},
        mini::Canvas2,
    },
    utils::types::{ArchiveType, MyResult},
};
use emeta::meta;
use log::log_enabled;
use std::{
    fmt::format,
    path::PathBuf,
    sync::{Arc, RwLock},
};
use tokio::sync::Notify;

/// display images
pub async fn cat_img(
    config: &Config,
    page_list: Vec<PageInfo>,
    meta_size: MetaSize<u32>,
    format: PixelFormat,
    metadata: &Option<meta::MetaData>,
    path: &str,
    archive_type: ArchiveType,
) -> MyResult {
    let screen_size = meta_size.screen;
    let window_size = meta_size.window;

    //    let canvas = Box::leak(Box::new(Canvas::new(
    //        window_size,
    //        format,
    //        config.base.font.as_deref(),
    //    )?));
    let step = 10; // drop 1/n part of image once
    let max_bytes = (window_size.width as usize * window_size.height as usize);
    let block = max_bytes / step;

    let mut buf = Buffer {
        bytes: vec![],
        max_bytes,

        start: 0,
        end: max_bytes,

        archive_path: PathBuf::from(path),
        archive_type,

        block,           //
        format,          // image format
        mode: Map::Stop, // keymap
        page_end: page_list.len(),
        page_list,
        screen_size,
        step, //
        window_position: (0, 0),
        window_size,

        range_start: 0,
        range_end: 0,
    };

    let mut canvas = Canvas2::new(window_size.width as usize, window_size.height as usize);
    let keymaps = keymap::KeyMap::new();

    for_minifb(&mut buf, &mut canvas, &keymaps).await;

    println!("CLOSE");

    Ok(())
}

pub async fn for_minifb(buf: &mut Buffer, canvas: &mut Canvas2, keymaps: &[keymap::KeyMap]) {
    let color_buffer_arc: Arc<RwLock<Vec<u32>>> = Arc::new(RwLock::new(Vec::new()));
    let state_arc = Arc::new(RwLock::new(State::NextLoad));

    buf.init();

    'l1: while canvas.window.is_open() {
        // input from keymap
        match keymap::match_event(canvas.window.get_keys().iter().as_slice(), keymaps) {
            Map::Down => {
                buf.move_down(&color_buffer_arc, &state_arc);
            }

            Map::Up => {
                buf.move_up(&color_buffer_arc, &state_arc);
                //buf.move_up();
            }

            Map::DisplayMeta => {
                todo!()
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
                break 'l1;
            }

            _ => {
                // input from mouse
                if let Some((x, y)) = canvas.window.get_scroll_wheel() {
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

        buf.flush(canvas);

        std::thread::sleep(std::time::Duration::from_millis(40));
    }
}

// should we use SDL2 as a feature?
// pub fn for_sdl2() {
//     let mut event_pump = canvas.sdl_context.event_pump()?;
//     // keymap
//     'l1: loop {
//         for event in event_pump.poll_iter() {
//             // eprintln!("{:?}", input);
//
//             match keymap::match_event(&event, &keymaps) {
//                 Map::Down => {
//                     buf.move_down(canvas);
//                 }
//
//                 Map::Up => {
//                     buf.move_up(canvas);
//                 }
//
//                 /*
//                 Map::DisplayMeta => {
//                     if let Some(meta) = metadata {
//                         //eprintln!("{:?}", meta);
//
//                         use emeta::meta::TUpdateMeta;
//                         use emeta::tags::*;
//                         let mut meta = emeta::meta::MetaData::new();
//
//                         meta.artist(&Some(emeta::tags::TagArtist {
//                             name: vec!["安慰大文档".to_string()],
//                         }));
//                         meta.group(&Some(emeta::tags::TagGroup {
//                             name: vec!["a".to_string()],
//                         }));
//                         let text = format!("{:?}", meta);
//                         canvas.display_text(text.as_str())?;
//                     } else {
//                         // doing nothing
//                     }
//                 }
//                 */
//                 Map::Reset => {
//                     // check if the position of the window as same as position of the buffer
//                     if canvas.sdl_canvas.window().position() != buf.window_position {
//                         canvas.reset_pos(buf.window_position.0, buf.window_position.1);
//                     } else {
//                     }
//                 }
//
//                 Map::FullScreen => {
//                     canvas.try_fullscreen()?;
//                 }
//
//                 Map::Exit => {
//                     break 'l1;
//                 }
//
//                 /*
//                 Map::Left => {
//                 }
//
//                 Map::Right => {
//                 }
//                     */
//                 _ => {}
//             }
//         }
//
//         thread::sleep(duration);
//     }
// }
