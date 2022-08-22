use crate::{
    archive,
    color::{format::PixelFormat, rgb::TransRgb},
    config::rsconf::Config,
    img::{
        resize,
        size::{MetaSize, Size},
    },
    math::arrmatrix::{Affine, ArrMatrix},
    reader::{
        buffer::{self, Buffer, PageInfo},
        keymap::{self, Map, TKeycode},
        mini::Canvas2,
    },
    utils::types::{ArchiveType, MyResult},
};
use emeta::meta;
use minifb::{Key, Scale, ScaleMode, Window};
use std::{
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
    thread,
    time::Duration,
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum State {
    Start,
    Ready,
    Done,
}

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
        view: (0, 0),
        window_position: (0, 0),
        window_size,
    };

    let mut canvas = Canvas2::new(window_size.width as usize, window_size.height as usize);
    let keymaps = keymap::KeyMap::new();

    buf.init();
    for_minifb(&mut buf, &mut canvas, &keymaps).await;

    println!("CLOSE");

    Ok(())
}

pub async fn for_minifb(buf: &mut Buffer, canvas: &mut Canvas2, keymaps: &[keymap::KeyMap]) {
    let next_color_buffer_arc: Arc<RwLock<(Vec<u32>, usize)>> =
        Arc::new(RwLock::new((Vec::new(), 0)));
    let next_state_arc = Arc::new(RwLock::new(State::Start));

    // let mut prev_color_buffer_arc: Arc<RwLock<Vec<u32>>> = Arc::new(RwLock::new(Vec::new()));
    // let prev_state_arc = Arc::new(RwLock::new(State::Start));

    'l1: while canvas.window.is_open() {
        match keymap::match_event(canvas.window.get_keys().iter().as_slice(), keymaps) {
            Map::Down => {
                // HACK: async version
                let cut_len = buf.page_list[buf.view.0].len;

                if (buf.at_tail() || buf.bytes.len() <= buf.max_bytes * 8)
                    && buf.not_tail()
                    && *next_state_arc.read().unwrap() == State::Start
                {
                    let mut color_buffer = next_color_buffer_arc.clone();
                    let mut state = next_state_arc.clone();
                    *state.write().unwrap() = State::Ready;

                    buf.goto_next();

                    let archive_type = buf.archive_type;
                    let archive_path = buf.archive_path.clone();
                    let page_path = buf.page_list[buf.view.1].path.clone();
                    let page_pos = buf.page_list[buf.view.1].pos;
                    let screen_size = buf.screen_size;
                    let window_size = buf.window_size;

                    tokio::spawn(async move {
                        let mut img_buffer = Vec::new();

                        get_rgb_buffer(
                            &mut img_buffer,
                            archive_type,
                            archive_path.as_path(),
                            page_path.as_path(),
                            page_pos,
                            screen_size,
                            window_size,
                        );

                        color_buffer
                            .write()
                            .unwrap()
                            .0
                            .extend_from_slice(img_buffer.as_slice());

                        color_buffer.write().unwrap().1 = img_buffer.len();

                        *state.write().unwrap() = State::Done;

                        log::debug!("len: {}", color_buffer.read().unwrap().0.len());
                        log::debug!("DONE");
                    });
                } else {
                }

                //println!("bytes_len == {}", buf.bytes.len());

                //println!("len: {}", color_buffer_arc.read().unwrap().0.len());
                if *next_state_arc.read().unwrap() == State::Done {
                    //println!("load_next");

                    buf.bytes
                        .extend_from_slice(next_color_buffer_arc.read().unwrap().0.as_slice());
                    buf.page_list[buf.view.1].len = next_color_buffer_arc.read().unwrap().1;

                    next_color_buffer_arc.write().unwrap().0.clear();

                    *next_state_arc.write().unwrap() = State::Start;

                    if buf.start >= cut_len && buf.bytes.len() > buf.max_bytes + cut_len {
                        buf.view.0 += 1;
                        free_head(&mut buf.bytes, cut_len);

                        buf.start -= cut_len;
                        buf.end -= cut_len;
                    } else {
                    }
                } else {
                }

                buf.move_down();
            }

            Map::Up => {
                buf.move_up(); // not async

                //  let cut_len = buf.page_list[buf.view.1].len;
                //
                //  if (buf.at_head() || buf.need_pad())
                //      && buf.not_head()
                //      && *prev_state_arc.read().unwrap() == State::Start
                //  {
                //      let mut color_buffer = prev_color_buffer_arc.clone();
                //      let mut state = prev_state_arc.clone();
                //      *state.write().unwrap() = State::Ready;
                //
                //      buf.goto_prev();
                //
                //      let archive_type = buf.archive_type;
                //      let archive_path = buf.archive_path.clone();
                //      let page_path = buf.page_list[buf.view.0].path.clone();
                //      let page_pos = buf.page_list[buf.view.0].pos;
                //      let screen_size = buf.screen_size;
                //      let window_size = buf.window_size;
                //
                //      tokio::spawn(async move {
                //          let mut img_buffer = Vec::new();
                //
                //          get_rgb_buffer(
                //              &mut img_buffer,
                //              archive_type,
                //              archive_path.as_path(),
                //              page_path.as_path(),
                //              page_pos,
                //              screen_size,
                //              window_size,
                //          );
                //
                //          color_buffer
                //              .write()
                //              .unwrap()
                //              .extend_from_slice(img_buffer.as_slice());
                //
                //          *state.write().unwrap() = State::Done;
                //
                //          log::debug!("len: {}", color_buffer.read().unwrap().len());
                //          log::debug!("DONE");
                //      });
                //  } else {
                //  }
                //
                //  if *prev_state_arc.read().unwrap() == State::Done {
                //      push_front(
                //          &mut buf.bytes,
                //          prev_color_buffer_arc.read().unwrap().as_slice(),
                //      );
                //      prev_color_buffer_arc.write().unwrap().clear();
                //      *prev_state_arc.write().unwrap() = State::Start;
                //
                //      if buf.bytes.len() > buf.max_bytes * 2 + cut_len
                //          && buf.view.1 - 1 >= 0
                //          && buf.bytes.len() > buf.end + cut_len
                //      {
                //          buf.view.1 -= 1;
                //          free_tail(&mut buf.bytes, cut_len);
                //
                //          log::debug!("le:: bytes.len: {:?}", cut_len);
                //      } else {
                //      }
                //  } else {
                //  }
                // buf.move_up();
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
            _ => {}
        }

        buf.flush(canvas);

        std::thread::sleep(std::time::Duration::from_millis(40));
    }
}

pub fn get_rgb_buffer(
    buffer: &mut Vec<u32>,
    archive_type: ArchiveType,
    archive_path: &Path,
    page_path: &Path,
    page_pos: usize,
    screen_size: Size<u32>,
    window_size: Size<u32>,
) {
    let mut img = Vec::new();

    resize_img(
        &mut img,
        archive_type,
        archive_path,
        page_path,
        page_pos,
        screen_size,
        window_size,
    );

    for f in (0..img.len()).step_by(3) {
        buffer.push(TransRgb::rgb_to_u32(&img[f..f + 3].try_into().unwrap()));
    }
}

pub fn resize_img(
    buffer: &mut Vec<u8>,
    archive_type: ArchiveType,
    archive_path: &Path,
    page_path: &Path,
    page_pos: usize,
    screen_size: Size<u32>,
    window_size: Size<u32>,
) {
    log::debug!("archive_type == {:?}", archive_type);

    let bytes = match archive_type {
        ArchiveType::Tar => {
            log::debug!("ex_tar()");

            archive::tar::load_file(archive_path, page_path).unwrap()
        }

        ArchiveType::Zip => {
            log::debug!("ex_zip()");

            archive::zip::load_file(archive_path, page_pos).unwrap()
        }

        _ => {
            todo!()
        }
    };

    resize::resize_bytes(bytes.as_slice(), buffer, screen_size, window_size);
}

pub fn free_head<T>(buffer: &mut Vec<T>, range: usize)
where
    T: Sized + Clone,
{
    buffer.drain(..range);
}

pub fn free_tail<T>(buffer: &mut Vec<T>, range: usize)
where
    T: Sized,
{
    buffer.truncate(buffer.len() - range);
}

pub fn push_front<T>(vec: &mut Vec<T>, slice: &[T])
where
    T: Copy,
{
    unsafe {
        let len = vec.len();
        let amt = slice.len();

        vec.reserve(amt);

        std::ptr::copy(vec.as_ptr(), vec.as_mut_ptr().offset((amt) as isize), len);
        std::ptr::copy(slice.as_ptr(), vec.as_mut_ptr(), amt);

        vec.set_len(len + amt);
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
