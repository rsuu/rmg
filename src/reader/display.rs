use crate::reader::mini::Canvas2;
use crate::{
    color::format::PixelFormat,
    config::rsconf::Config,
    img::size::MetaSize,
    math::arrmatrix::{Affine, ArrMatrix},
    reader::{
        buffer::{self, PageInfo},
        keymap::Map,
    },
    utils::types::{ArchiveType, MyResult},
};
use emeta::meta;
use minifb::{Key, Scale, ScaleMode, Window};
use std::{path::PathBuf, thread, time::Duration};

/// display images
pub fn cat_img(
    config: &Config,
    page_list: Vec<PageInfo>,
    meta_size: MetaSize<u32>,
    format: PixelFormat,
    metadata: &Option<meta::MetaData>,
    path: &str,
) -> MyResult {
    use crate::reader::mini;

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

    let mut buf = buffer::Buffer {
        bytes: vec![],
        max_bytes,

        start: 0,
        end: max_bytes,

        archive_path: PathBuf::from(path),
        archive_type: ArchiveType::Tar,

        block,           //
        format,          // image format
        mode: Map::Stop, // keymap
        page_end: page_list.len(),
        page_list,
        screen_size,
        step, //
        view: (0, 0),
        window_position: (0, 0),
        window_size,
    };

    let mut canvas = Canvas2::new(window_size.width as usize, window_size.height as usize);

    buf.init(&mut canvas);

    //canvas.window.set_position(900, 900);
    'l1: while canvas.window.is_open() && !canvas.window.is_key_down(Key::Escape) {
        match canvas.window.get_keys().iter().as_slice() {
            [Key::J] => {
                buf.move_down(&mut canvas);
            }

            [Key::K] => {
                buf.move_up(&mut canvas);
            }

            [Key::H] => {
                buf.move_left();
            }

            [Key::L] => {
                buf.move_right();
            }

            [Key::Escape] => {
                break 'l1;
            }

            _ => {}
        }

        buf.flush(&mut canvas);

        std::thread::sleep(std::time::Duration::from_millis(40));
    }

    //let mut event_pump = canvas.sdl_context.event_pump()?;
    //    // keymap
    //    'l1: loop {
    //        for event in event_pump.poll_iter() {
    //            // eprintln!("{:?}", input);
    //
    //            match keymap::match_event(&event, &keymaps) {
    //                Map::Down => {
    //                    buf.move_down(canvas);
    //                }
    //
    //                Map::Up => {
    //                    buf.move_up(canvas);
    //                }
    //
    //                /*
    //                Map::DisplayMeta => {
    //                    if let Some(meta) = metadata {
    //                        //eprintln!("{:?}", meta);
    //
    //                        use emeta::meta::TUpdateMeta;
    //                        use emeta::tags::*;
    //                        let mut meta = emeta::meta::MetaData::new();
    //
    //                        meta.artist(&Some(emeta::tags::TagArtist {
    //                            name: vec!["安慰大文档".to_string()],
    //                        }));
    //                        meta.group(&Some(emeta::tags::TagGroup {
    //                            name: vec!["a".to_string()],
    //                        }));
    //                        let text = format!("{:?}", meta);
    //                        canvas.display_text(text.as_str())?;
    //                    } else {
    //                        // doing nothing
    //                    }
    //                }
    //                */
    //                Map::Reset => {
    //                    // check if the position of the window as same as position of the buffer
    //                    if canvas.sdl_canvas.window().position() != buf.window_position {
    //                        canvas.reset_pos(buf.window_position.0, buf.window_position.1);
    //                    } else {
    //                    }
    //                }
    //
    //                Map::FullScreen => {
    //                    canvas.try_fullscreen()?;
    //                }
    //
    //                Map::Exit => {
    //                    break 'l1;
    //                }
    //
    //                /*
    //                Map::Left => {
    //                }
    //
    //                Map::Right => {
    //                }
    //                    */
    //                _ => {}
    //            }
    //        }
    //
    //        thread::sleep(duration);
    //    }

    println!("CLOSE");

    Ok(())
}
