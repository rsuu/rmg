use sdl2::{pixels::PixelFormatEnum, render::Texture, render::TextureCreator};

use std::cell::RefCell;

use crate::{color::format::PixelFormat, img::size::Size};

type Error = Box<dyn std::error::Error>;

pub struct Canvas {
    pub sdl_context: sdl2::Sdl,
    pub sdl_canvas: sdl2::render::Canvas<sdl2::video::Window>,
    pub creator: TextureCreator<sdl2::video::WindowContext>,
    pub texture: RefCell<Texture<'static>>,

    pub data: Vec<u32>,

    pub width: u32,
    pub height: u32,

    pub screen_width: u32,
    pub screen_height: u32,
}

impl Canvas {
    pub fn new(size: Size<u32>, format: PixelFormat) -> Result<Self, Error> {
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
    pub fn update(&mut self) {
        let mut texture = self.texture.borrow_mut();

        if texture
            .update(None, self.data_raw(), self.data_max_len())
            .is_ok()
        {
            // Copy data
            self.sdl_canvas
                .copy(&texture, None, None)
                .unwrap_or_else(|e| panic!("{}", e));
        } else {
            todo!()
        }
    }

    #[inline(always)]
    pub fn draw_pixel(&mut self, x: u32, y: u32, color: u32) {
        let height = y * self.width;
        let pixel = height + x;

        self.data[pixel as usize] = color;
    }

    #[inline(always)]
    pub fn data_raw(&self) -> &[u8] {
        // ?
        unsafe { std::slice::from_raw_parts(self.data.as_ptr() as *const u8, 0) }
    }

    pub fn data_max_len(&self) -> usize {
        // If self.width = 800
        // Then the pixel = [0_u32;800]
        // Because the length of u32 is 4bytes
        // So the bytes of pixel = [0_u32;800 * 4] = self.data.maxlen()
        (self.width * 4) as usize
    }

    #[inline(always)]
    pub fn flush(&mut self) {
        self.sdl_canvas.present();
    }
}
