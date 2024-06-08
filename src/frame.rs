pub mod heic;
pub mod svg;

// feature
pub mod aseprite;
pub mod avif;

use crate::*;

use fir::ResizeAlg;
use image;
use std::{io::Cursor, mem, num::NonZeroU32};

#[derive(Clone)]
pub struct Frame {
    pub fmt: FrameFmt,
    pub data: FrameData,

    pub size: Size,
    pub vertex: Rect,
}

#[derive(Clone)]
pub enum FrameData {
    Single {
        data: Pixels,
    },

    Multi {
        data: Vec<Pixels>,

        index: usize,
        len: usize,

        pts: Vec<f32>,
        delay: f32,
    },

    DynSingle {
        data: Pixels,
    },
}

#[derive(Clone)]
pub enum FrameFmt {
    // single frame
    Jpeg,
    Png,
    // Apng, // no plans

    // AVIF and HEIC are both in HEIF container.
    // no animation support.
    Avif,
    Heic,

    // has animation support.
    Aseprite,
    Webp,
    Gif,
    // Jxl, // ?

    // no animation support.
    Svg,

    Unknown,
}

#[derive(Clone)]
pub enum Pixels {
    RGBA { inner: Vec<u32> },
    ARGB { inner: Vec<u32> },
}

impl Pixels {
    pub fn flip(&mut self, size: Size) {
        let Size { width, height } = size;

        match self {
            Self::RGBA { inner } | Self::ARGB { inner } => {
                for h in inner.chunks_mut(width as usize) {
                    h.reverse();
                }
            }
        }
    }

    pub fn free(&mut self) {
        match self {
            Self::RGBA { inner } | Self::ARGB { inner } => {
                inner.clear();
                inner.shrink_to(0);
            }
        }
    }

    pub fn from_rgba_bytes(bytes: Vec<u8>) -> Self {
        let mut inner = vec![0; bytes.len() / 4];
        for (index, rgba) in bytes.chunks(4).enumerate() {
            let [r, g, b, a] = [rgba[0], rgba[1], rgba[2], rgba[3]];

            mem::swap(&mut inner[index], &mut u32::from_be_bytes([r, g, b, a]));
        }

        Self::RGBA { inner }
    }

    pub fn as_bytes(&self) -> &[u32] {
        match self {
            Self::RGBA { inner } | Self::ARGB { inner } => inner.as_slice(),
        }
    }

    pub fn to_argb_bytes(&self) -> Vec<u32> {
        match self {
            Self::RGBA { inner } => rgba_to_argb_bytes(inner.as_slice()),
            Self::ARGB { inner } => inner.clone(),
        }
    }

    pub fn as_argb_bytes(&self) -> &[u32] {
        match self {
            Self::ARGB { inner } => inner.as_slice(),
            _ => unreachable!(),
        }
    }
}

impl Frame {
    pub fn flip(&mut self) {
        match &mut self.data {
            FrameData::Single { data } => data.flip(self.size),

            FrameData::Multi { data, .. } => {
                for frame in data.iter_mut() {
                    frame.flip(self.size);
                }
            }

            _ => unimplemented!(),
        }
    }

    pub fn next_frame(&mut self) -> &Pixels {
        self.data.next_frame()
    }

    pub fn free(&mut self) {
        match &mut self.data {
            FrameData::Single { data } | FrameData::DynSingle { data } => data.free(),

            FrameData::Multi { data, .. } => {
                data.clear();
                data.shrink_to(0);
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FrameTy {
    Img,
    Anim,
}

impl Frame {
    pub fn vertex(&self) -> &Rect {
        &self.vertex
    }

    pub fn resize(blob: &[u8], dst_size: Size, algo: ResizeAlg) -> eyre::Result<Self> {
        use imagesize::{blob_size, image_type, ImageType};

        // FIXME(imagesize): avif will be detected as heic
        let ty = image_type(blob)?;
        let size = blob_size(blob)?;

        let fmt = {
            match ty {
                ImageType::Jpeg => FrameFmt::Jpeg,
                ImageType::Png => FrameFmt::Png,

                ImageType::Avif => FrameFmt::Avif,
                ImageType::Heif => FrameFmt::Heic,
                // TODO:
                // ImageType::Heic => FrameFmt::Heic,
                ImageType::Aseprite => FrameFmt::Aseprite,
                ImageType::Gif => FrameFmt::Gif,
                ImageType::Webp => FrameFmt::Webp,

                _ => unimplemented!(),
            }
        };

        let size = Size::new(size.width as f32, size.height as f32);

        let data = {
            match fmt {
                FrameFmt::Jpeg | FrameFmt::Png | FrameFmt::Avif => {
                    let img = image::load_from_memory(blob)?;
                    let mut data = img.to_rgba8().to_vec();
                    resize_rgba8(&mut data, size, dst_size, algo)?;

                    FrameData::Single {
                        data: Pixels::from_rgba_bytes(data.to_vec()),
                    }
                }
                FrameFmt::Heic => {
                    let (.., mut data) = heic::load_heic(&blob)?;
                    resize_rgba8(&mut data, size, dst_size, algo)?;

                    FrameData::Single {
                        data: Pixels::from_rgba_bytes(data.to_vec()),
                    }
                }

                // TODO: no resize
                FrameFmt::Gif | FrameFmt::Webp => {
                    use image::{
                        codecs::gif::GifDecoder, codecs::webp::WebPDecoder, AnimationDecoder,
                    };

                    let mut pts = Vec::with_capacity(30);
                    let mut len = 0;

                    FrameData::Multi {
                        data: {
                            let cur = Cursor::new(blob);

                            let frames = {
                                match fmt {
                                    FrameFmt::Gif => {
                                        GifDecoder::new(cur)?.into_frames().collect_frames()?
                                    }

                                    FrameFmt::Webp => {
                                        WebPDecoder::new(cur)?.into_frames().collect_frames()?
                                    }

                                    _ => unreachable!(),
                                }
                            };

                            len = frames.len();
                            let mut data = Vec::with_capacity(frames.len());
                            for frame in frames {
                                let delay = frame.delay().numer_denom_ms().0 as f32;
                                pts.push(delay);

                                let mut frame = frame.buffer().to_vec();

                                if size.width() < dst_size.width() {
                                    let (bgw, bgh) =
                                        (dst_size.width() as usize, size.height() as usize);
                                    let bg = &mut vec![0; bgw * 4 * bgh];
                                    let fg = &mut frame;
                                    let (fgw, fgh) = (size.width() as usize, bgh);

                                    center_img(bg, fg, bgw, fgw, fgh);
                                    bg.shrink_to_fit();

                                    fg.clear();
                                    fg.shrink_to(0);

                                    mem::swap(bg, fg);
                                } else if size.width() > dst_size.width() {
                                    resize_rgba8(&mut frame, size, dst_size, algo)?;
                                } else if size.width() == dst_size.width() {
                                    // doing nothing
                                }

                                data.push(Pixels::from_rgba_bytes(frame))
                            }

                            data
                        },

                        index: 0,
                        delay: 0.0,
                        len,
                        pts,
                    }
                }
                FrameFmt::Aseprite => {
                    let (.., pts, frames) = frame::aseprite::load_aseprite(&blob)?;

                    let mut data = Vec::with_capacity(pts.len());
                    for frame in frames.into_iter() {
                        let mut frame = frame;

                        if size.width() < dst_size.width() {
                            let (bgw, bgh) = (dst_size.width() as usize, size.height() as usize);
                            let bg = &mut vec![0; bgw * 4 * bgh];
                            let fg = &mut frame;
                            let (fgw, fgh) = (size.width() as usize, bgh);

                            center_img(bg, fg, bgw, fgw, fgh);
                            bg.shrink_to_fit();

                            fg.clear();
                            fg.shrink_to(0);

                            mem::swap(bg, fg);
                        } else if size.width() > dst_size.width() {
                            resize_rgba8(&mut frame, size, dst_size, algo)?;
                        } else if size.width() == dst_size.width() {
                            // doing nothing
                        }

                        data.push(Pixels::from_rgba_bytes(frame))
                    }

                    FrameData::Multi {
                        index: 0,
                        delay: 0.0,
                        len: pts.len(),
                        data,
                        pts,
                    }
                }

                FrameFmt::Svg => todo!(),

                _ => unimplemented!(),
            }
        };

        let vertex = Rect::new_at_zero(dst_size);

        Ok(Self {
            fmt,
            data,
            vertex,
            size: dst_size,
        })
    }

    pub fn ty(&self) -> FrameTy {
        match self.data {
            FrameData::Single { .. } => FrameTy::Img,
            FrameData::Multi { .. } => FrameTy::Anim,
            FrameData::DynSingle { .. } => FrameTy::Img,
        }
    }
}

impl FrameData {
    pub fn next_frame(&mut self) -> &Pixels {
        match self {
            Self::Single { ref data, .. } => &data,

            Self::Multi {
                ref data,
                ref mut index,
                ref len,
                ref pts,
                ref mut delay,
            } => {
                let pts = pts[*index];
                let t = *delay - pts;

                // timeout
                if t > 0.0 {
                    *delay = t;

                    if *index + 1 < *len {
                        *index += 1;
                    } else {
                        // reset
                        *index = 0;
                    }

                // unready
                } else {
                    *delay += 1000.0 / 24.0;
                }

                &data[*index]
            }

            Self::DynSingle { ref data, .. } => &data,
        }
    }
}

impl From<&str> for FrameFmt {
    fn from(value: &str) -> Self {
        match value {
            "jpg" | "jpeg" => Self::Jpeg,
            "png" => Self::Png,

            "heic" | "heif" => Self::Heic,
            "avif" => Self::Avif,

            "ase" | "aseprite" => Self::Aseprite,
            "webp" => Self::Webp,
            "gif" => Self::Gif,

            "svg" => Self::Svg,

            _ => Self::Unknown,
        }
    }
}

impl Default for Frame {
    fn default() -> Self {
        Self {
            fmt: FrameFmt::Unknown,
            data: FrameData::Single {
                data: Pixels::RGBA { inner: vec![] },
            },
            size: Size::default(),
            vertex: Rect::default(),
        }
    }
}

impl Frame {
    pub fn as_argb(&self) -> &[u32] {
        match &self.data {
            FrameData::Single { data } => data.as_argb_bytes(),
            FrameData::Multi { data, index, .. } => data[*index].as_argb_bytes(),
            _ => todo!(),
        }
    }

    // pub fn resize(&mut self, dst_size: &Size, algo: ResizeAlg) -> eyre::Result<()> {
    //     match &mut self.data {
    //         FrameData::Single { data } => {
    //             resize_rgba8(&mut data.inner, &self.size, dst_size, algo)?;
    //         }
    //
    //         FrameData::Multi { data, .. } => 's: {
    //             if self.size.width() < dst_size.width() {
    //                 for pixels in data.iter_mut() {
    //                     let (bgw, bgh) = (dst_size.width() as usize, self.size.height() as usize);
    //                     let bg = &mut vec![0; bgw * 4 * bgh];
    //                     let fg = &mut pixels.inner;
    //                     let (fgw, fgh) = (self.size.width() as usize, bgh);
    //
    //                     center_img(bg, fg, bgw, fgw, fgh);
    //                     bg.shrink_to_fit();
    //
    //                     fg.clear();
    //                     fg.shrink_to(0);
    //
    //                     mem::swap(bg, fg);
    //                 }
    //             } else if self.size.width() > dst_size.width() {
    //                 for pixels in data.iter_mut() {
    //                     resize_rgba8(&mut pixels.inner, &self.size, dst_size, algo)?;
    //                 }
    //             } else if self.size.width() == dst_size.width() {
    //                 // doing nothing
    //                 break 's;
    //             }
    //         }
    //
    //         _ => eyre::bail!("ERROR: Unknown Format"),
    //     }
    //
    //     Ok(())
    // }
}

// make sure `bg.height == fg.height`.
pub fn center_img(bg: &mut [u8], fg: &mut [u8], bgw: usize, fgw: usize, fgh: usize) {
    let x_offset = (bgw * 4 - fgw * 4) / 2;

    for y in 0..fgh {
        let fgi = y * fgw * 4;
        let bgi = y * bgw * 4 + x_offset;
        let w = fgw * 4;

        bg[bgi..(bgi + w)].copy_from_slice(&fg[fgi..(fgi + w)]);
    }
}

pub fn resize_rgba8(
    bytes: &mut Vec<u8>,
    src: Size,
    dst: Size,
    algo: ResizeAlg,
) -> eyre::Result<()> {
    // dbg!(&src, &dst);

    let (sw, sh) = (src.width() as u32, src.height() as u32);
    let (dw, dh) = (dst.width() as u32, dst.height() as u32);

    let mut src_image = fir::Image::from_slice_u8(
        NonZeroU32::new(sw).unwrap(),
        NonZeroU32::new(sh).unwrap(),
        bytes,
        fir::PixelType::U8x4,
    )?;
    let dw = NonZeroU32::new(dw).unwrap();
    let dh = NonZeroU32::new(dh).unwrap();

    let mut dst_image = fir::Image::new(dw, dh, src_image.pixel_type());
    let mut dst_view = dst_image.view_mut();
    let mut resizer = fir::Resizer::new(algo);

    unsafe {
        #[cfg(feature = "arch_sse4_1")]
        {
            resizer.set_cpu_extensions(fir::CpuExtensions::Sse4_1);
        }

        #[cfg(feature = "arch_avx2")]
        {
            resizer.set_cpu_extensions(fir::CpuExtensions::Avx2);
        }
    }

    resizer.resize(&src_image.view(), &mut dst_view)?;

    // RGBA
    let alpha_mul_div = fir::MulDiv::default();
    alpha_mul_div.multiply_alpha_inplace(&mut src_image.view_mut())?;
    alpha_mul_div.divide_alpha_inplace(&mut dst_view)?;

    *bytes = mem::take(&mut dst_image.buffer().to_owned());
    bytes.shrink_to_fit();

    Ok(())
}

fn rgba_to_argb_bytes(bytes: &[u32]) -> Vec<u32> {
    let mut res = vec![0; bytes.len()];

    for (index, rgba) in bytes.iter().enumerate() {
        let [r, g, b, a] = rgba.to_be_bytes();

        res[index] = u32::from_be_bytes([a, r, g, b]);
    }

    res
}
