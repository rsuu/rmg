use crate::{color::format::PixelFormat, img::size::Size, utils::types::SelfResult};
use sdl2::{pixels::PixelFormatEnum, render::Texture, render::TextureCreator};
use std::cell::RefCell;

pub struct Canvas {
    pub sdl_context: sdl2::Sdl,
    pub sdl_canvas: sdl2::render::Canvas<sdl2::video::Window>,
    pub creator: TextureCreator<sdl2::video::WindowContext>,
    pub texture: RefCell<Texture<'static>>,
    pub data: Vec<u32>,
    pub ttf: Option<sdl2::ttf::Font<'static, 'static>>,
    pub window_size: Size<u32>,
    pub screen_size: Size<u32>,
}

impl Canvas {
    pub fn new<_Str>(
        size: Size<u32>,
        format: PixelFormat,
        font_path: Option<&_Str>,
    ) -> SelfResult<Self>
    where
        _Str: AsRef<str> + ?Sized,
    {
        let (width, height) = (size.width, size.height);
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let window = video_subsystem
            .window("rmg", width, height)
            .borderless()
            //.opengl()
            .position_centered()
            .build()?;
        let sdl_canvas = window.into_canvas().build()?;
        let creator = sdl_canvas.texture_creator();

        let texture = match format {
            PixelFormat::Rgb8 => {
                creator.create_texture_target(PixelFormatEnum::RGB888, width, height)?
            }

            PixelFormat::Rgba8 => {
                creator.create_texture_target(PixelFormatEnum::RGBA8888, width, height)?
            }
        };

        let texture = unsafe { std::mem::transmute::<_, Texture<'static>>(texture) };
        let data_len = (size.width * size.height) as usize;
        let screen = &sdl_context.video()?.current_display_mode(0)?;

        let font = if let Some(ttf) = font_path {
            let ttf_context = Box::leak(Box::new(sdl2::ttf::init().map_err(|e| e.to_string())?));

            let mut font = ttf_context.load_font(ttf.as_ref(), 128)?;

            font.set_style(sdl2::ttf::FontStyle::BOLD);

            Some(font)
        } else {
            None
        };

        Ok(Canvas {
            sdl_canvas,
            sdl_context,
            creator,
            texture: RefCell::new(texture),

            data: vec![0; data_len],

            ttf: font,

            window_size: Size { width, height },
            screen_size: Size {
                width: screen.w as u32,
                height: screen.h as u32,
            },
        })
    }

    #[inline(always)]
    pub fn update(&mut self) -> SelfResult<()> {
        let mut texture = self.texture.borrow_mut();

        if texture
            .update(None, self.data_raw(), self.data_max_len())
            .is_ok()
        {
            // Copy data
            self.sdl_canvas.copy(&texture, None, None)?;
        } else {
            // panic
            todo!()
        }

        Ok(())
    }

    #[inline(always)]
    pub fn draw_pixel(&mut self, x: u32, y: u32, color: u32) {
        let height = y * self.window_size.width;
        let pixel = height + x;

        self.data[pixel as usize] = color;
    }

    #[inline(always)]
    pub fn data_raw(&self) -> &[u8] {
        // https://docs.rs/sdl2/latest/src/sdl2/render.rs.html#1998
        // return data.ptr()
        // data -> u32_ptr -> u8_ptr
        unsafe { std::slice::from_raw_parts(self.data.as_ptr() as *const u8, 0) }
    }

    #[inline(always)]
    pub fn data_max_len(&self) -> usize {
        // If self.width = 800
        // Then the pixel = [0_u32; 800]
        //   Because the length of u32 is 4bytes
        // So the bytes of pixel = [0_u32;800 * 4] = self.data.maxlen()
        (self.window_size.width * 4) as usize
    }

    #[inline(always)]
    pub fn flush(&mut self) {
        self.sdl_canvas.present();
    }

    pub fn reset_pos(&mut self, x: i32, y: i32) {
        self.sdl_canvas.window_mut().set_position(
            sdl2::video::WindowPos::Positioned(x),
            sdl2::video::WindowPos::Positioned(y),
        );
        self.sdl_canvas.present();
    }

    pub fn try_fullscreen(&mut self) -> SelfResult<()> {
        if self.sdl_canvas.window_mut().fullscreen_state() == sdl2::video::FullscreenType::True {
            self.sdl_canvas
                .window_mut()
                .set_fullscreen(sdl2::video::FullscreenType::Off)?;
        } else {
            self.sdl_canvas
                .window_mut()
                .set_fullscreen(sdl2::video::FullscreenType::True)?;
        }
        self.sdl_canvas.present();

        Ok(())
    }

    pub fn display_text(&mut self, text: &str) -> SelfResult<()> {
        // TODO

        if self.ttf.is_some()
            && self
                .texture
                .borrow_mut()
                .update(None, self.data_raw(), self.data_max_len())
                .is_ok()
        {
            let surface = self
                .ttf
                .as_ref()
                .unwrap()
                .render(text)
                .blended_wrapped(sdl2::pixels::Color::RGBA(255, 0, 0, 255), 128 * 5)?;

            let ttf_texture = self.creator.create_texture_from_surface(&surface)?;

            self.sdl_canvas.copy(&ttf_texture, None, None)?;

            self.sdl_canvas.present();
        } else {
        }

        Ok(())
    }
}

// REF
// https://github.com/Rust-SDL2/rust-sdl2/blob/master/examples/ttf-demo.rs
