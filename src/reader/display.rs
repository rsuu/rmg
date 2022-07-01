use crate::{
    color::{format::PixelFormat, rgb::TransRgb},
    config::rsconf::Config,
    img::size::{MetaSize, Size, TMetaSize},
    math::arrmatrix::{Affine, ArrMatrix},
    reader::canvas::Canvas,
    utils::types::MyResult,
};
use fast_image_resize as fir;
use image::{self, io::Reader as ImageReader};
use sdl2::{event::Event, keyboard::Keycode, mouse::MouseWheelDirection};
use std::{convert::TryInto, num::NonZeroU32, thread::sleep, time::Duration};

/// buffer
pub static mut RES_BUFFER: Vec<u32> = Vec::new();

/// buffer
pub static mut RGB_BUFFER: Vec<u8> = Vec::new();

/// display images
pub async fn cat_rgb_img(
    config: &Config,
    file_list: &[&str],
    meta_size: MetaSize<u32>,
) -> MyResult {
    let screen_size = meta_size.screen;
    let window_size = meta_size.window;

    let mut canvas = Box::leak(Box::new(Canvas::new(
        window_size,
        PixelFormat::Rgb,
        config.base.font.as_deref(),
    )?));

    let window_position = canvas.sdl_canvas.window().position();
    let max_pixels = (window_size.width * window_size.height) as usize;
    let n = 8; // drop 1/n part of image once
               // :drop
    let duration = Duration::from_millis(100);

    let metadata = {};
    let mut buf = Buffer {
        start: 0,
        end: max_pixels,
        bytes: vec![],

        step: 8,
        temp_step: 0,

        speed: (window_size.width * (window_size.height / n)) as usize, // ::drop
        mode: Move::Stop,
        page: 1,
        max_page: file_list.len(),

        window_size,
        screen_size,
        window_position,
    };

    let mut event_pump = canvas.sdl_context.event_pump()?;

    // TODO
    // if file_list.len() == 0 { panic }
    unsafe {
        buf.lazy_load_imgs(&file_list[0..buf.page], PixelFormat::Rgb)
            .await?;

        if buf.page + 1 < buf.max_page {
            buf.lazy_load_imgs(&file_list[buf.page..buf.page + 1], PixelFormat::Rgb)
                .await?;

            buf.page += 1;
        } else {
        }
    }

    // init
    buf.move_down(canvas);
    buf.move_up(canvas);

    // match input
    'l1: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'l1,

                // up: move up
                // down: move down
                Event::MouseWheel {
                    direction: MouseWheelDirection::Normal,
                    ..
                } => {
                    // Mouse
                    if let Event::MouseWheel { y, .. } = event {
                        if y < 0 {
                            buf.move_down(canvas);

                            buf.try_load_next(file_list).await?;

                            // move down
                            // Try to load next block of images
                        } else if y > 0 {
                            // move up

                            buf.move_up(canvas);
                        }
                    }
                }

                // p: display metadata
                // TODO
                Event::KeyDown {
                    keycode: Option::Some(Keycode::P),
                    repeat: false,
                    ..
                } => {
                    // TODO
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

                // j: move down
                Event::KeyDown {
                    keycode: Option::Some(Keycode::J),
                    repeat: true,
                    ..
                } => {
                    buf.move_down(canvas);
                    buf.try_load_next(file_list).await?;
                }

                // j: move down
                Event::KeyUp {
                    keycode: Option::Some(Keycode::J),
                    repeat: false,
                    ..
                } => {
                    buf.move_down(canvas);
                    buf.try_load_next(file_list).await?;
                }

                // k: move up
                Event::KeyDown {
                    keycode: Option::Some(Keycode::K),
                    repeat: true,
                    ..
                } => {
                    // k

                    buf.move_up(canvas);
                }

                // k: move up
                Event::KeyUp {
                    keycode: Option::Some(Keycode::K),
                    repeat: false,
                    ..
                } => {
                    // k

                    buf.move_up(canvas);
                }

                // h: move to left
                Event::KeyDown {
                    keycode: Option::Some(Keycode::H),
                    repeat: false,
                    ..
                } => {
                    // TODO

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

                        buf.mode = Move::Left;
                    }
                }

                // l: move to right
                // TODO
                Event::KeyDown {
                    keycode: Option::Some(Keycode::L),
                    repeat: false,
                    ..
                } => {
                    if let Some(data) = ArrMatrix::new(
                        canvas.data.as_slice(),
                        canvas.window_size.width,
                        canvas.window_size.height,
                    )
                    .translate_x((canvas.window_size.width as usize) / 2, true)
                    {
                        canvas.data = data;

                        canvas.update();
                        canvas.flush();

                        buf.mode = Move::Right;
                    }
                }

                // r: reset position
                Event::KeyDown {
                    keycode: Option::Some(Keycode::R),
                    repeat: false,
                    ..
                } => {
                    canvas.sdl_canvas.window_mut().set_position(
                        sdl2::video::WindowPos::Positioned(buf.window_position.0),
                        sdl2::video::WindowPos::Positioned(buf.window_position.1),
                    );
                    canvas.flush();
                }

                // f:
                // Not good
                Event::KeyDown {
                    keycode: Option::Some(Keycode::F),
                    repeat: false,
                    ..
                } => {
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

                _ => {}
            }
        }

        sleep(duration)
    }

    println!("DONE");

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Move {
    Up,
    Down,

    Left,
    Right,

    Stop,
}

struct Buffer {
    bytes: Vec<u32>,

    start: usize,
    end: usize,
    speed: usize,
    page: usize,
    max_page: usize,
    step: usize,
    temp_step: usize,

    mode: Move,

    window_size: Size<u32>,
    screen_size: Size<u32>,
    window_position: (i32, i32),
}

impl Buffer {
    ///
    fn get_block(&self) -> Vec<u32> {
        self.bytes[self.start..self.end].to_vec()
    }

    /// display prev page
    fn move_up(&mut self, canvas: &mut Canvas) {
        if self.start >= self.speed {
            self.start -= self.speed;
            self.end -= self.speed;

            canvas.data.clear();
            canvas
                .data
                .extend_from_slice(&self.bytes[self.start..self.end]);

            self.mode = Move::Up;

            canvas.update();
            canvas.flush();
        } else {
        }
    }

    /// display next page
    fn move_down(&mut self, canvas: &mut Canvas) {
        if self.end + self.speed <= self.bytes.len() {
            self.start += self.speed;
            self.end += self.speed;

            canvas.data.clear();
            canvas
                .data
                .extend_from_slice(&self.bytes[self.start..self.end]);

            canvas.update();
            canvas.flush();

            self.mode = Move::Down;
        } else if self.end <= self.bytes.len() {
            // load last part
            let s = self.bytes.len() - self.end;
            self.start += s;
            self.end += s;

            canvas.data.clear();
            canvas
                .data
                .extend_from_slice(&self.bytes[self.start..self.end]);

            canvas.update();
            canvas.flush();

            self.mode = Move::Down;
        } else {
        }

        self.temp_step += 1;
    }

    /// load next page
    async fn try_load_next(&mut self, file_list: &[&str]) -> MyResult {
        // if time
        if self.temp_step == self.step {
            self.temp_step = 0;

            if self.page < self.max_page {
                unsafe {
                    self.lazy_load_imgs(&file_list[self.page..self.page + 1], PixelFormat::Rgb)
                }
                .await?;
            }

            self.page += 1;
        } else {
        }

        Ok(())
    }

    //
    async unsafe fn lazy_load_imgs(&mut self, imgs: &[&str], p: PixelFormat) -> MyResult {
        match p {
            PixelFormat::Rgb => {
                for filename in imgs.iter() {
                    resize(
                        &mut RGB_BUFFER,
                        filename,
                        self.screen_size,
                        self.window_size,
                        PixelFormat::Rgb,
                    )
                    .await?;

                    for f in RGB_BUFFER.as_slice().chunks(3) {
                        RES_BUFFER.push(TransRgb::rgb_to_u32(f.try_into().unwrap()));
                    }
                }
            }

            PixelFormat::Rgba => {
                for filename in imgs.iter() {
                    resize(
                        &mut RGB_BUFFER,
                        filename,
                        self.screen_size,
                        self.window_size,
                        PixelFormat::Rgba,
                    )
                    .await?;

                    for _f in RES_BUFFER.as_slice().chunks(4) {
                        // TODO
                        // RES_BUFFER.push(TransRgba::rgba_to_u32(f.try_into().unwrap()));
                    }
                }
            }
        };

        self.bytes.extend_from_slice(RES_BUFFER.as_slice());
        RES_BUFFER.clear();
        RGB_BUFFER.clear();
        Ok(())
    }
}

pub async fn resize(
    buffer: &mut Vec<u8>,
    path: &str,
    screen_size: Size<u32>,
    window_size: Size<u32>,
    ty: PixelFormat,
) -> MyResult {
    let img = ImageReader::open(path)?.decode()?;
    // TODO
    let mut meta = MetaSize::<u32>::new(
        screen_size.width,
        screen_size.height,
        window_size.width,
        window_size.height,
        img.width(),
        img.height(),
    );

    meta.resize();

    if ty == PixelFormat::Rgb {
        let src_image = fir::Image::from_vec_u8(
            NonZeroU32::new(meta.image.width).expect(""),
            NonZeroU32::new(meta.image.height).expect(""),
            img.to_rgb8().into_raw(),
            fir::PixelType::U8x3,
        )?;
        let dst_width = NonZeroU32::new(meta.fix.width).unwrap();
        let dst_height = NonZeroU32::new(meta.fix.height).unwrap();

        let mut dst_image = fir::Image::new(dst_width, dst_height, src_image.pixel_type());
        let mut dst_view = dst_image.view_mut();
        let mut resizer = fir::Resizer::new(fir::ResizeAlg::Convolution(fir::FilterType::Box));

        resizer.resize(&src_image.view(), &mut dst_view)?;

        (*buffer).extend_from_slice(dst_image.buffer());
    } else if ty == PixelFormat::Rgba {
        todo!()
    };

    Ok(())
}
