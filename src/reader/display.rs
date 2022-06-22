use std::{
    cell::RefCell,
    convert::TryInto,
    fs,
    ops::Add,
    path::{Path, PathBuf},
    thread::sleep,
    time::Duration,
};

use sdl2::{
    event::Event, keyboard::Keycode, mouse::MouseWheelDirection, pixels::PixelFormatEnum,
    render::Texture, render::TextureCreator,
};

use image::{self, DynamicImage, EncodableLayout};

use crate::{
    archive::{tar, zip},
    color::{format::PixelFormat, rgb::TransRgb, rgba::TransRgba},
    files::list,
    img::size::{MetaSize, Size, TMetaSize},
    math::arrmatrix::{Affine, ArrMatrix},
};

use tempfile::TempDir;

type Error = Box<dyn std::error::Error>;

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

    mode: Move,
}

impl Buffer {
    fn move_up(&mut self, canvas: &mut Canvas) {
        if self.start >= self.speed {
            self.start -= self.speed;
            self.end -= self.speed;

            canvas.data = self.bytes[self.start..self.end].to_vec();

            self.mode = Move::Up;
        } else {
        }
    }

    fn move_down(&mut self, canvas: &mut Canvas) {
        if self.end + self.speed <= self.bytes.len() {
            self.start += self.speed;
            self.end += self.speed;

            canvas.data = self.bytes[self.start..self.end].to_vec();

            self.mode = Move::Down;
        } else {
        }
    }

    fn load_imgs(&mut self, imgs: &[&str], p: PixelFormat, img_size: Size<u32>) {
        self.bytes = match p {
            PixelFormat::Rgb => {
                let mut res: Vec<u32> = Vec::new();

                for file in imgs.into_iter() {
                    let bytes = handle_img_resize(file, img_size.width, img_size.height);

                    let chunks = bytes.as_rgb8().unwrap().as_bytes().chunks(3);
                    for f in chunks.into_iter() {
                        res.push(TransRgb::rgb_to_u32(f.try_into().unwrap()));
                    }

                    // TODO
                }

                res
            }

            PixelFormat::Rgba => {
                todo!()
            }
        };
    }
}

pub fn cat_rgb_img(file_list: &[&str], window_size: Size<u32>) -> Result<(), Error> {
    let duration = Duration::from_millis(100);
    let max_pixel = (window_size.width * window_size.height) as usize;
    let mut canvas = Canvas::new(window_size, PixelFormat::Rgb)?;

    let n = 4;

    let mut buf = Buffer {
        start: 0,
        end: max_pixel,
        bytes: vec![],
        speed: (window_size.width * (window_size.height / n)) as usize, // drop 1/n part of image once
        mode: Move::Stop,
    };

    //println!("{:?}", file_list);

    buf.load_imgs(file_list, PixelFormat::Rgb, window_size);

    // println!("{} = {}", buf.bytes.len(), window_size.0 * window_size.1 * 2);

    let mut event_pump = canvas.sdl_context.event_pump().unwrap();

    canvas.data = buf.bytes[buf.start..buf.end].to_vec();
    // println!("{}", canvas.data.len());

    let mut move_l_r_buffer = Vec::new();
    let mut lf_step = 0;

    canvas.update();
    canvas.flush();

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
                            // move down
                            buf.move_down(&mut canvas);
                            println!("{}", buf.start);
                            println!("{}", buf.end);

                            canvas.update();
                            canvas.flush();
                        } else if y > 0 {
                            // move up

                            buf.move_up(&mut canvas);
                            canvas.update();
                            canvas.flush();
                        }
                    }
                }

                Event::KeyDown {
                    keycode: Option::Some(Keycode::J),
                    repeat: false,
                    ..
                } => {
                    // j
                    // move down
                    // BUG

                    buf.move_down(&mut canvas);

                    canvas.update();
                    canvas.flush();
                }

                Event::KeyDown {
                    keycode: Option::Some(Keycode::K),
                    repeat: false,
                    ..
                } => {
                    // k
                    // move up

                    buf.move_up(&mut canvas);
                    canvas.update();
                    canvas.flush();
                }

                Event::KeyDown {
                    keycode: Option::Some(Keycode::H),
                    repeat: false,
                    ..
                } => {
                    // TODO
                    // h
                    // move left
                    if buf.mode != Move::Left && buf.mode != Move::Right {
                        move_l_r_buffer = canvas.data.clone();

                        buf.mode = Move::Left;

                        println!("debug");
                        println!("{:?}", buf.mode);
                    } else if buf.mode != Move::Left {
                        canvas.data = move_l_r_buffer.clone();

                        buf.mode = Move::Left;
                    } else if buf.mode == Move::Left {
                        canvas.data = canvas.data;
                    }

                    canvas.data =
                        ArrMatrix::new(canvas.data.as_slice(), canvas.width, canvas.height)
                            .translate_x((canvas.width as usize) / 2, false)
                            .unwrap();

                    canvas.update();
                    canvas.flush();
                }

                Event::KeyDown {
                    keycode: Option::Some(Keycode::L),
                    repeat: false,
                    ..
                } => {
                    // l
                    // move right

                    if buf.mode != Move::Left && buf.mode != Move::Right {
                        move_l_r_buffer = canvas.data.clone();

                        buf.mode = Move::Right;
                    } else if buf.mode == Move::Left {
                        canvas.data = move_l_r_buffer.clone();

                        buf.mode = Move::Right;
                    } else if buf.mode == Move::Right {
                        canvas.data = canvas.data;
                    }

                    canvas.data =
                        ArrMatrix::new(canvas.data.as_slice(), canvas.width, canvas.height)
                            .translate_x((canvas.width as usize) / 2, true)
                            .unwrap();

                    canvas.update();
                    canvas.flush();
                }

                _ => {}
            }
        }
        sleep(duration);
    }

    println!("DONE");

    Ok(())
}

struct Canvas {
    sdl_context: sdl2::Sdl,
    sdl_canvas: sdl2::render::Canvas<sdl2::video::Window>,
    creator: TextureCreator<sdl2::video::WindowContext>,
    texture: RefCell<Texture<'static>>,

    data: Vec<u32>,

    width: u32,
    height: u32,

    screen_width: u32,
    screen_height: u32,
}

impl Canvas {
    fn new(size: Size<u32>, format: PixelFormat) -> Result<Self, Error> {
        let (width, height) = (size.width, size.height);
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let window = video_subsystem
            .window("Window", width, height)
            .position_centered()
            .build()?;
        let sdl_canvas = window.into_canvas().build()?;
        let creator = sdl_canvas.texture_creator();

        let texture = match format {
            PixelFormat::Rgb => {
                creator.create_texture_target(PixelFormatEnum::RGB888, width, height)?
            }
            PixelFormat::Rgba => {
                creator.create_texture_target(PixelFormatEnum::RGBA8888, width, height)?
            }
        };

        let texture = unsafe { std::mem::transmute::<_, Texture<'static>>(texture) };
        let data_len = (size.width * size.height) as usize;
        let screen = &sdl_context.video().unwrap().current_display_mode(0)?;

        Ok(Canvas {
            sdl_canvas,
            sdl_context,
            creator,
            texture: RefCell::new(texture),

            data: vec![0; data_len],

            width,
            height,
            screen_width: screen.w as u32,
            screen_height: screen.h as u32,
        })
    }

    #[inline(always)]
    fn update(&mut self) {
        let mut texture = self.texture.borrow_mut();

        if let Ok(_) = texture.update(None, self.data_raw(), self.data_max_len()) {
            // Copy data
            self.sdl_canvas
                .copy(&texture, None, None)
                .unwrap_or_else(|e| panic!("{}", e));
        } else {
            todo!()
        }
    }

    #[inline(always)]
    fn draw_pixel(&mut self, x: u32, y: u32, color: u32) {
        let height = y * self.width;
        let pixel = height + x;

        self.data[pixel as usize] = color;
    }

    #[inline(always)]
    fn data_raw(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.data.as_ptr() as *const u8, self.data.len()) }
    }

    fn data_max_len(&self) -> usize {
        // If self.width = 800
        // Then the pixel = [0_u32;800]
        // Because the length of u32 is 4bytes
        // So the bytes of pixel = [0_u32;800 * 4] = self.date.maxlen()
        (self.width * 4) as usize
    }

    #[inline]
    fn wait(&self) -> Move {
        let duration = Duration::from_millis(100);

        let mut event_pump = self.sdl_context.event_pump().unwrap();

        'l1: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => std::process::exit(-1),

                    Event::KeyDown {
                        keycode: Option::Some(Keycode::J),
                        repeat: false,
                        ..
                    } => {
                        // down
                        return Move::Down;
                    }

                    Event::KeyDown {
                        keycode: Option::Some(Keycode::K),
                        repeat: false,
                        ..
                    } => {
                        // up
                        return Move::Up;
                    }
                    _ => return Move::Stop,
                }
            }

            sleep(duration);
        }
    }

    #[inline(always)]
    fn flush(&mut self) {
        self.sdl_canvas.present();
    }
}

pub fn handle_img(path: &str, width: u32, height: u32) -> DynamicImage {
    if let Ok(img) = image::open(path) {
        img
    } else {
        todo!()
    }
}

pub fn handle_img_resize(path: &str, width: u32, height: u32) -> DynamicImage {
    if let Ok(img) = image::open(path) {
        // !!! important
        // if width = 3 height = 4
        // do width = (width/2) * 2    = (3/2) * 2 = 1 * 2 = 2
        //    height = (height/2) * 2  = (4/2) * 2 = 2 * 2 = 4
        // We will get a bug if miss it
        // (╯°Д°)╯︵ ┻━┻
        // TODO
        let mut meta = MetaSize::<u32>::new(1440, 900, 900, 900, img.width(), img.height());
        meta.resize();
        img.resize_exact(
            meta.fix.width,
            meta.fix.height,
            image::imageops::FilterType::Nearest,
        )
    } else {
        todo!()
    }
}

pub fn get_wh(path: &str) -> (u32, u32) {
    if let Ok(img) = image::open(path) {
        (img.width(), img.height())
        //img
    } else {
        todo!()
    }
}

/*

pub fn cat_rgba(file: &str) -> Result<(), Error> {
    let wh = get_wh(file);

    let img = handle_img(file, wh.0, wh.1);
    let mut bytes = img.as_rgba8().unwrap().as_bytes().chunks(4);
    let mut canvas = Canvas::new(wh.0, wh.1, PixelFormat::Rgba)?;

    // 1. left to right
    for h in 0..wh.1 {
        // 2. up to down
        for w in 0..wh.0 {
            canvas.draw_pixel(
                w,
                h,
                rgba_to_u32(bytes.next().unwrap().as_ref().try_into().unwrap()),
            );
        }
    }

    canvas.update();
    canvas.flush();
    sleep(Duration::from_millis(1000));
    canvas.wait(); // user input

    println!("DONE");

    Ok(())
}

*/
