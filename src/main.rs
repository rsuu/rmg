use std::{cell::RefCell, convert::TryInto, thread::sleep, time::Duration};

use image::{self, DynamicImage, EncodableLayout};
use ndarray::arr2;
use rgb;
use sdl2::{
    event::Event, keyboard::Keycode, mouse::MouseWheelDirection, pixels::PixelFormatEnum,
    render::Texture, render::TextureCreator,
};

type Error = Box<dyn std::error::Error>;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() >= 2 {
        let format = args[1].as_str();
        let file = args[2].as_str();

        match format {
            "img" => {
                cat_rgb_img(file).unwrap();
            }
            "rgb" => {
                cat_rgb(file).unwrap();
            }
            "rgba" => {
                cat_rgba(file).unwrap();
            }
            "rgba1" => {
                let width = args[3].parse::<u32>().unwrap();
                let height = args[4].parse::<u32>().unwrap();

                cat_rgba_resize(file, width, height).unwrap();
            }
            _ => {}
        }
    }

    let mut m: Vec<u32> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    // run_coo(&mut m);
}

#[derive(Debug, Clone, Copy)]
pub struct MetaSize {
    pub win_width: u32,
    pub win_height: u32,
    pub img_width: u32,
    pub img_heigth: u32,
    pub fix_width: u32,
    pub fix_heigth: u32,
}

#[derive(Debug, Clone, Copy)]
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
    pos: usize,
    speed: usize,
}

impl Buffer {
    fn move_down(&mut self, canvas: &mut Canvas) {
        if self.end + self.speed <= self.bytes.len() {
            self.start += self.speed;
            self.end += self.speed;

            canvas.data = self.bytes[self.start..self.end].to_vec();
        } else {
        }
    }

    fn move_up(&mut self, canvas: &mut Canvas) {
        if self.start >= self.speed {
            self.start -= self.speed;
            self.end -= self.speed;

            canvas.data = self.bytes[self.start..self.end].to_vec();
        } else {
        }
    }

    fn load_imgs(&mut self, imgs: &[&str], p: PixelFormat, img_size: (u32, u32)) {
        self.bytes = match p {
            Rgb => {
                let mut res: Vec<u32> = Vec::new();

                for file in imgs.into_iter() {
                    let bytes = handle_img_resize(file, img_size.0, img_size.1);

                    let chunks = bytes.as_rgb8().unwrap().as_bytes().chunks(3);
                    for f in chunks.into_iter() {
                        res.push(rgb_to_u32(f.try_into().unwrap()));
                    }

                    // TODO
                }

                res
            }

            Rgba => {
                todo!()
            }
        };
    }
}

fn cat_rgb_img(file: &str) -> Result<(), Error> {
    let duration = Duration::from_millis(100);
    let wh = (900, 900); // 1080, 1500
    let max_pixel = (wh.0 * wh.1) as usize;
    let mut canvas = Canvas::new(wh.0, wh.1, PixelFormat::Rgb)?;

    let n = 4;

    let mut buf = Buffer {
        start: 0,
        end: max_pixel,
        pos: max_pixel,
        bytes: vec![],
        speed: (wh.0 * (wh.1 / n)) as usize, // drop 1/n part of image once
    };

    let imgs = ["tests/files/rgb.jpg", file, file];
    buf.load_imgs(imgs.as_slice(), PixelFormat::Rgb, wh);

    // println!("{} = {}", buf.bytes.len(), wh.0 * wh.1 * 2);

    let mut event_pump = canvas.sdl_context.event_pump().unwrap();

    canvas.data = buf.bytes[buf.start..buf.end].to_vec();
    // println!("{}", canvas.data.len());

    canvas.update();
    canvas.flush();

    'l1: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => std::process::exit(-1),

                Event::MouseWheel {
                    direction: MouseWheelDirection::Normal,
                    ..
                } => {
                    if let Event::MouseWheel { y, .. } = event {
                        if y < 0 {
                            // move down

                            buf.move_down(&mut canvas);

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
                    // down
                    //Move::Down;
                    // canvas.data.extend_from_slice(&null_array);
                    //
                }

                Event::KeyDown {
                    keycode: Option::Some(Keycode::K),
                    repeat: false,
                    ..
                } => {}
                _ => {}
            }
        }
        sleep(duration);
    }

    println!("DONE");

    Ok(())
}

fn cat_rgb(file: &str) -> Result<(), Error> {
    let wh = get_wh(file);

    let img = handle_img(file, wh.0, wh.1);
    let mut bytes = img.as_rgb8().unwrap().as_bytes().chunks(3);
    let mut canvas = Canvas::new(wh.0, wh.1, PixelFormat::Rgb)?;

    for h in 0..canvas.height {
        for w in 0..canvas.width {
            canvas.draw_pixel(
                w,
                h,
                rgb_to_u32(bytes.next().unwrap().as_ref().try_into().unwrap()),
            );
        }
    }

    canvas.update(); // display
    canvas.flush();
    sleep(Duration::from_millis(1000));
    canvas.wait(); // user input

    println!("DONE");

    Ok(())
}

fn cat_rgba(file: &str) -> Result<(), Error> {
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

fn cat_rgba_resize(file: &str, width: u32, height: u32) -> Result<(), Error> {
    let wh = (width, height);

    let img = handle_img_resize(file, wh.0, wh.1);
    let mut bytes = img.as_rgba8().unwrap().as_bytes().chunks(4);
    let mut canvas = Canvas::new(wh.0, wh.1, PixelFormat::Rgba)?;

    for h in 0..wh.1 {
        for w in 0..wh.0 {
            canvas.draw_pixel(
                w,
                h,
                rgba_to_u32(bytes.next().unwrap().as_ref().try_into().unwrap()),
            );
        }
    }

    canvas.update(); // display
    canvas.flush();
    sleep(Duration::from_millis(1000));
    canvas.wait(); // user input

    println!("DONE");

    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub enum PixelFormat {
    Rgb,
    Rgba,
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
    fn new(width: u32, height: u32, format: PixelFormat) -> Result<Self, Error> {
        // std::env::set_var("DBUS_FATAL_WARNINGS","0");
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let window = video_subsystem
            .window("Window", width, height)
            .position_centered()
            .build()?;
        let sdl_canvas = window.into_canvas().build()?;
        let creator = sdl_canvas.texture_creator();

        let texture = match format {
            Rgb => creator.create_texture_target(PixelFormatEnum::RGB888, width, height)?,
            Rgba => creator.create_texture_target(PixelFormatEnum::RGBA8888, width, height)?,
        };

        let texture = unsafe { std::mem::transmute::<_, Texture<'static>>(texture) };
        let data_len = (width * height) as usize;
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

    fn flush(&mut self) {
        self.sdl_canvas.present();
    }
}

fn handle_img(path: &str, width: u32, height: u32) -> DynamicImage {
    if let Ok(img) = image::open(path) {
        img
    } else {
        todo!()
    }
}

fn handle_img_resize(path: &str, width: u32, height: u32) -> DynamicImage {
    if let Ok(img) = image::open(path) {
        img.resize_exact(width, height, image::imageops::FilterType::Nearest)
    } else {
        todo!()
    }
}

fn get_wh(path: &str) -> (u32, u32) {
    if let Ok(img) = image::open(path) {
        (img.width(), img.height())
        //img
    } else {
        todo!()
    }
}

fn rgb_to_u32(rgb: &[u8; 3]) -> u32 {
    let r = (rgb[0] as u32) << 16;
    let g = (rgb[1] as u32) << 8;
    let b = (rgb[2] as u32);

    r + g + b
}

fn rgb_from_u32(rgb: u32) -> [u8; 3] {
    let r = (rgb >> 16) & 0x0ff;
    let g = (rgb >> 8) & 0x0ff;
    let b = rgb & 0x0ff;

    [r as u8, g as u8, b as u8]
}

fn rgba_to_u32(rgba: &[u8; 4]) -> u32 {
    let r = (rgba[0] as u32) << 24;
    let g = (rgba[1] as u32) << 16;
    let b = (rgba[2] as u32) << 8;
    let a = (rgba[3] as u32);

    r + g + b + a
}

fn rgba_from_u32(rgba: u32) -> [u8; 4] {
    let r = (rgba >> 24) & 0x0ff;
    let g = (rgba >> 16) & 0x0ff;
    let b = (rgba >> 8) & 0x0ff;
    let a = rgba & 0x0ff;

    [r as u8, g as u8, b as u8, a as u8]
}

#[derive(Debug)]
struct Coo2 {
    val: u32,
    x: u32,
    y: u32,
}

impl Coo2 {
    fn new(data: &mut [u32], width: u32) -> Vec<Self> {
        let mut res: Vec<Coo2> = Vec::new();
        let mut x: u32 = 0;
        let mut y: u32 = 0;

        for w in data.chunks(width as usize).into_iter() {
            for pixel in w.iter() {
                res.push(Coo2 { val: *pixel, x, y });
                x += 1;
            }

            y += 1;
        }

        res
    }

    fn translate(v_coo: &mut Vec<Self>, nx: u32, ny: u32) {
        for coo in v_coo.iter_mut() {
            coo.x = coo.x + nx;
            coo.y = coo.y + ny;
        }
    }

    fn scale(v_coo: &mut Vec<Self>, sx: u32, sy: u32) {
        for coo in v_coo.iter_mut() {
            coo.x = coo.x * sx;
            coo.y = coo.y * sy;
        }
    }
}

fn run_coo(array: &mut [u32]) {
    let mut coo = Coo2::new(array, 4);

    Coo2::translate(&mut coo, 100, 0);
    //Coo2::scale(&mut coo, 10, 10);
    println!("{:#?}", coo);
}
