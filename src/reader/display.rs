use crate::{
    color::format::PixelFormat,
    config::rsconf::Config,
    img::size::{MetaSize, TMetaSize},
    math::arrmatrix::Affine,
    reader::{
        buffer,
        canvas::Canvas,
        keymap::{self, Map},
    },
    utils::types::MyResult,
};

use std::{thread, time::Duration};

/// display images
pub async fn cat_img(
    config: &Config,
    file_list: &[&str],
    meta_size: MetaSize<u32>,
    format: PixelFormat,
) -> MyResult {
    let screen_size = meta_size.screen;
    let window_size = meta_size.window;

    let canvas = Box::leak(Box::new(Canvas::new(
        window_size,
        format,
        config.base.font.as_deref(),
    )?));
    let step = 8; // drop 1/n part of image once
    let block = (window_size.width as usize * (window_size.height as usize / step));

    let mut buf = buffer::Buffer {
        bytes: vec![],                                          // buffer
        start: 0,                                               // buffer
        end: (window_size.width * window_size.height) as usize, // width * height
        max_page: file_list.len(),
        format,          // image format
        step,            //
        block,           //
        mode: Map::Stop, // keymap
        temp_step: 0,    // load next image
        page: 1,         // load next image
        window_size,
        screen_size,
        window_position: canvas.sdl_canvas.window().position(), // use as reset pos
    };

    // try to load the first image
    // we just try to load data here
    // and we do not need to flush them
    unsafe {
        buf.lazy_load_imgs(&file_list[0..buf.page], format).await?;

        if buf.page + 1 < buf.max_page {
            buf.lazy_load_imgs(&file_list[buf.page..buf.page + 1], format)
                .await?;

            buf.page += 1;
        } else {
            // doing nothing
        }
    }

    // init
    // display image
    buf.move_down(canvas);
    buf.move_up(canvas);

    let keymaps = keymap::KeyMap::new(); // user input
    let duration = Duration::from_millis(10); // sleep

    let mut event_pump = canvas.sdl_context.event_pump()?;

    // keymap
    'l1: loop {
        for event in event_pump.poll_iter() {
            // eprintln!("{:?}", input);

            match keymap::match_event(&event, &keymaps) {
                Map::Down => {
                    buf.move_down(canvas);
                    buf.try_load_next(file_list).await?; // TODO: very slow
                }

                Map::Up => {
                    buf.move_up(canvas);
                }

                Map::DisplayMeta => {
                    canvas.display_text(
                        "\
                a:
                  1234
                b:
                  1234
                ",
                    );

                    canvas.flush();
                }

                Map::Reset => {
                    canvas.sdl_canvas.window_mut().set_position(
                        sdl2::video::WindowPos::Positioned(buf.window_position.0),
                        sdl2::video::WindowPos::Positioned(buf.window_position.1),
                    );
                    canvas.flush();
                }

                Map::FullScreen => {
                    if canvas.sdl_canvas.window_mut().fullscreen_state()
                        == sdl2::video::FullscreenType::True
                    {
                        canvas
                            .sdl_canvas
                            .window_mut()
                            .set_fullscreen(sdl2::video::FullscreenType::Off)?;
                    } else {
                        canvas
                            .sdl_canvas
                            .window_mut()
                            .set_fullscreen(sdl2::video::FullscreenType::True)?;
                    }

                    canvas.flush();
                }

                Map::Exit => {
                    break 'l1;
                }

                Map::Left => {
                    /*
                        if let Some(data) = ArrMatrix::new(
                        canvas.data.as_slice(),
                        canvas.window_size.width,
                        canvas.window_size.height,
                    )
                    .translate_x((canvas.window_size.width as usize) / 2, false)
                    {
                        canvas.data = data;

                        canvas.update();
                        canvas.flush();

                        buf.mode = Map::Left;
                    }

                       */
                }
                Map::Right => {}

                _ => {}
            }
        }

        thread::sleep(duration);
    }

    println!("CLOSE");

    Ok(())
}
