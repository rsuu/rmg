use crate::{
    color::{format::PixelFormat, rgb::TransRgb, rgba::TransRgba},
    img::{
        resize,
        size::{Size, TMetaSize},
    },
    math::arrmatrix::{Affine},
    reader::{canvas::Canvas, keymap::Map},
    utils::types::MyResult,
};



use std::{convert::TryInto};

/// buffer
pub static mut RES_BUFFER: Vec<u32> = Vec::new();

/// buffer
pub static mut COLOR_BUFFER: Vec<u8> = Vec::new();

pub struct Buffer {
    pub bytes: Vec<u32>,

    pub start: usize,
    pub end: usize,
    pub block: usize,
    pub page: usize,
    pub max_page: usize,
    pub step: usize,
    pub temp_step: usize,

    pub mode: Map,
    pub format: PixelFormat,

    pub window_size: Size<u32>,
    pub screen_size: Size<u32>,
    pub window_position: (i32, i32),
}

impl Buffer {
    /// return a block of image
    pub fn get_block(&self) -> Vec<u32> {
        self.bytes[self.start..self.end].to_vec()
    }

    /// display prev page
    // Sliding Window
    pub fn move_up(&mut self, canvas: &mut Canvas) {
        if self.start >= self.block {
            self.start -= self.block;
            self.end -= self.block;
        } else {
        }

        canvas.data.clear();
        canvas
            .data
            .extend_from_slice(&self.bytes[self.start..self.end]);
        canvas.update();
        canvas.flush();

        self.mode = Map::Up;
    }

    /// display next page
    // Sliding Window
    pub fn move_down(&mut self, canvas: &mut Canvas) {
        if self.end + self.block <= self.bytes.len() {
            self.start += self.block;
            self.end += self.block;
        } else {
        }

        //eprintln!("start: {}, end: {}", self.start, self.end);

        canvas.data.clear();
        canvas
            .data
            .extend_from_slice(&self.bytes[self.start..self.end]);
        canvas.update();
        canvas.flush();

        self.mode = Map::Down;

        self.temp_step += 1; // use as load next image
    }

    /// load next page
    pub async fn try_load_next(&mut self, file_list: &[&str]) -> MyResult {
        // TODO: if time
        if self.temp_step == self.step {
            self.temp_step = 0;

            if self.page < self.max_page {
                unsafe {
                    self.lazy_load_imgs(&file_list[self.page..self.page + 1], self.format)
                        .await?;
                }
            }

            self.page += 1;
        } else {
        }

        Ok(())
    }

    //
    pub async unsafe fn lazy_load_imgs(&mut self, imgs: &[&str], format: PixelFormat) -> MyResult {
        match format {
            PixelFormat::Rgb8 => {
                for filename in imgs.iter() {
                    resize::resize(
                        &mut COLOR_BUFFER,
                        filename,
                        self.screen_size,
                        self.window_size,
                        PixelFormat::Rgb8,
                    )
                    .await?;

                    for f in COLOR_BUFFER.as_slice().chunks(3) {
                        RES_BUFFER.push(TransRgb::rgb_to_u32(f.try_into()?));
                    }
                }
            }

            PixelFormat::Rgba8 => {
                for filename in imgs.iter() {
                    resize::resize(
                        &mut COLOR_BUFFER,
                        filename,
                        self.screen_size,
                        self.window_size,
                        PixelFormat::Rgba8,
                    )
                    .await?;

                    for f in COLOR_BUFFER.as_slice().chunks(4) {
                        RES_BUFFER.push(TransRgba::rgba_to_u32(f.try_into()?));
                    }
                }
            }
        };

        self.bytes.extend_from_slice(RES_BUFFER.as_slice());
        RES_BUFFER.clear();
        COLOR_BUFFER.clear();

        Ok(())
    }
}
