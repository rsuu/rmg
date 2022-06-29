use crate::{
    color::{format::PixelFormat, rgb::TransRgb},
    img::size::{MetaSize, Size, TMetaSize},
    math::arrmatrix::{Affine, ArrMatrix},
    reader::canvas::Canvas,
    utils::types::{MyError, MyResult},
};
use fast_image_resize as fir;
use image::{self, io::Reader as ImageReader};
use sdl2::{event::Event, keyboard::Keycode, mouse::MouseWheelDirection};
use std::{convert::TryInto, num::NonZeroU32, thread::sleep, time::Duration};

pub async fn cat_rgb_img(file_list: &[&str], window_size: Size<u32>) -> MyResult {
    let duration = Duration::from_millis(100);
    let max_pixels = (window_size.width * window_size.height) as usize;
    let n = 8; // drop 1/n part of image once
               // :drop

    let mut canvas = Box::leak(Box::new(Canvas::new(window_size, PixelFormat::Rgb)?));

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
        window_size: Size {
            width: 900,
            height: 900,
        },
    };

    // TODO
    // if file_list.len() == 0 { panic }
    buf.lazy_load_imgs(&file_list[0..buf.page], PixelFormat::Rgb, window_size)
        .await?;

    let mut event_pump = canvas.sdl_context.event_pump()?;

    if buf.page + 1 < buf.max_page {
        buf.lazy_load_imgs(
            &file_list[buf.page..buf.page + 1],
            PixelFormat::Rgb,
            buf.window_size,
        )
        .await?;

        buf.page += 1;
    } else {
    }

    // init
    buf.move_down(&mut canvas);
    buf.move_up(&mut canvas);

    // match input
    'l1: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'l1,

                Event::MouseWheel {
                    direction: MouseWheelDirection::Normal,
                    ..
                } => {
                    // Mouse
                    if let Event::MouseWheel { y, .. } = event {
                        if y < 0 {
                            buf.move_down(&mut canvas);

                            buf.try_load_next(file_list).await?;

                            // move down
                            // Try to load next block of images
                        } else if y > 0 {
                            // move up

                            buf.move_up(&mut canvas);
                        }
                    }
                }

                Event::KeyDown {
                    keycode: Option::Some(Keycode::P),
                    repeat: false,
                    ..
                } => {
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

                Event::KeyDown {
                    keycode: Option::Some(Keycode::J),
                    repeat: false,
                    ..
                } => {
                    // j
                    // move down

                    buf.move_down(&mut canvas);
                    buf.try_load_next(file_list).await?;
                }

                Event::KeyDown {
                    keycode: Option::Some(Keycode::K),
                    repeat: false,
                    ..
                } => {
                    // k
                    // move up

                    buf.move_up(&mut canvas);
                }

                Event::KeyDown {
                    keycode: Option::Some(Keycode::H),
                    repeat: false,
                    ..
                } => {
                    // TODO
                    // h
                    // move left

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

                Event::KeyDown {
                    keycode: Option::Some(Keycode::L),
                    repeat: false,
                    ..
                } => {
                    // l
                    // move right

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
}

impl Buffer {
    fn get_block(&self) -> Vec<u32> {
        self.bytes[self.start..self.end].to_vec()
    }

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

            self.mode = Move::Down;
        } else {
        }

        self.temp_step += 1;
    }

    async fn try_load_next(&mut self, file_list: &[&str]) -> MyResult {
        // if time
        if self.temp_step == self.step {
            self.temp_step = 0;

            if self.page < self.max_page {
                self.lazy_load_imgs(
                    &file_list[self.page..self.page + 1],
                    PixelFormat::Rgb,
                    self.window_size,
                )
                .await?;
            }

            self.page += 1;
        } else {
        }

        Ok(())
    }

    async fn lazy_load_imgs(
        &mut self,
        imgs: &[&str],
        p: PixelFormat,
        window_size: Size<u32>,
    ) -> MyResult {
        let res = match p {
            PixelFormat::Rgb => {
                let mut res: Vec<u32> = Vec::new();
                let mut buffer = Vec::new();

                for filename in imgs.iter() {
                    resize(&mut buffer, filename, window_size.width, window_size.height).await?;

                    let chunks = buffer.as_slice().chunks(3);

                    for f in chunks {
                        res.push(TransRgb::rgb_to_u32(f.try_into().unwrap()));
                    }

                    buffer.clear();
                }

                res
            }

            PixelFormat::Rgba => {
                todo!()
            }
        };

        self.bytes.extend_from_slice(res.as_slice());
        Ok(())
    }
}

pub async fn resize(buffer: &mut Vec<u8>, path: &str, ww: u32, wh: u32) -> MyResult {
    let img = ImageReader::open(path)?.decode()?;
    // TODO
    let mut meta = MetaSize::<u32>::new(1440, 900, ww, wh, img.width(), img.height());

    meta.resize();

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

    Ok(())
}
