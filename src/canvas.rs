// TODO: i32 vs f32

pub mod buffer;
pub mod draw;
pub mod layout;
pub mod page;
pub mod task;

use crate::*;

use esyn::EsynDe;
use std::sync::Arc;

pub struct Canvas {
    config: Config,
    pub size: Size,
    pub buffer: Buffer,

    pub pages: Pages,
    pub flag_get_all_frame_size: bool,
    pub page_max_width: f32,

    pub offset: Vec2,

    /// camera's step
    step: Vec2,
    mode: Mode,

    /// background `RGBA` color in `u32` format.
    bg: u32,
    /// background image.
    pub bg_img: Vec<u32>,

    /// limit cache size when scroll.
    cache_factor_up: f32,
    cache_factor_down: f32,

    pool: Pool,
    /// archive's info.
    data: Arc<DataType>,
    // TODO:
    // min_page_width: f32
    // min_page_height: f32
    pub top_line: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct CurView<T = RectVertex> {
    pub view: T,
    pub border: T,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, EsynDe)]
pub enum Mode {
    #[default]
    Manga,
    Comic,
}

impl Canvas {
    pub fn new(config: Config, data: DataType) -> eyre::Result<Self> {
        // dbg!(&config);

        let Size { width, height } = config.canvas_size();

        let bg = config.bg();
        let tmp = vec![bg; config.canvas_size().len()];
        let fname_padding = 10;
        let mut empty_pages = data.gen_empty_pages(fname_padding)?;

        let data = Arc::new(data);
        // {
        //     let pages = empty_pages.clone();
        //     let data = data.clone();
        //     thread::spawn(move || {
        //         thread_get_all_frame_size(pages, data);
        //         //
        //     });
        // }

        Ok(Self {
            size: config.canvas_size(),
            step: config.canvas_step(),
            buffer: tmp.clone(),
            bg_img: tmp,
            mode: Mode::default(),
            offset: Vec2::default(),
            cache_factor_up: 0.5,
            cache_factor_down: 1.0,

            flag_get_all_frame_size: false,
            top_line: 0.0,
            page_max_width: config.canvas_size().width(),
            pool: Pool::new(empty_pages.clone()),
            pages: empty_pages,

            data,
            bg,
            config,
        })
    }

    pub fn draw(&mut self) -> eyre::Result<()> {
        // // #[cold]
        // 's: {
        //     if !self.flag_get_all_frame_size {
        //         let Some(vec) = THREAD_GET_ALL_FRAME_SIZE.get() else {
        //             break 's;
        //         };
        //
        //         for (new, page) in vec.iter().zip(self.pages.iter_mut()) {
        //             page.frame.size = new.frame.size;
        //             page.vertex = new.vertex;
        //         }
        //
        //         self.flag_get_all_frame_size = true;
        //     }
        // }

        match self.config.layout() {
            Layout::Vertical => self.draw_vertical(),
            Layout::Double => self.draw_double(),
            Layout::Masonry { .. } => self.draw_masonry(),
            Layout::Single { .. } => self.draw_single(),
            _ => todo!(),
        }
    }

    // TODO: cache `page.dst_size` and `page.frame.size`
    // Block {
    //   page: Page
    // }
    pub fn draw_vertical(&mut self) -> eyre::Result<()> {
        self.reset();

        let cur_view = self.cur_view();
        let new_width = self.page_max_width;

        // let mut no_layout = false;

        // TODO: blocks -> Body
        let mut blocks: Vec<&mut Page> = Vec::with_capacity(10);
        // rebuild everytime
        let mut page_offset: Vec2<f32> = Vec2::default();

        'l: for page in self.pages.iter_mut() {
            // TODO: center `Page`
            // let padding_left = (self.size.width - page.dst_size.width()) / 2.0;
            // let padding_left = padding_left.clamp(0.0, self.size.width());
            let padding_left = 0.0;

            // 1. page.drag()
            let drag_offset =
                Vec2::new(self.offset.x + padding_left, self.offset.y + page_offset.y);
            page.drag(drag_offset);

            // 2. update
            let h = page.dst_size.height();
            page_offset.y += h;

            let flag = page.is_passed(&self.config, &cur_view);
            if !flag {
                continue 'l;
            }

            match page.state {
                // 4. drawing
                State::Done => {
                    blocks.push(page);
                }

                // 3. loading
                State::Waiting => {
                    let data = self.data.clone();

                    if page.frame.size.is_zero() {
                        self.pool.task_set_size(page, data);
                        page.dst_size = page.frame.size.resize_by_width(new_width);
                    } else {
                        self.pool.task_resize(page, data, &self.config);
                    }
                }

                _ => {}
            }
        }

        // 6. drawing
        // dbg!(blocks.len());
        for block in blocks {
            // dbg!(
            //     &block.index,
            //     &block.state,
            //     &block.dst_size,
            //     &block.cast_vertex
            // );
            block.draw(&mut self.buffer, self.size);
        }

        // dbg!(&dbg_cur_drawing);
        // dbg!(&self.offset);

        Ok(())
    }

    // Block {
    //   page: Page,
    //   space: Space,
    // }
    //
    // page.draw();
    // space.draw();
    pub fn draw_double(&mut self) -> eyre::Result<()> {
        todo!();
        // self.clamp_offset();
        // self.reset();
        //
        // let cur_view = self.cur_view();
        //
        // let mut page_offset = Vec2::default();
        // // let mut dbg_cur_drawing = 0;
        //
        // let mut cur_col = 0;
        // let mut prev_height = 0;
        // let mut page_dst_size = self.size;
        // let cw = self.size.width();
        // let cw_half = cw / 2.0;
        //
        // for page in self.pages.iter_mut() {
        //     page.offset = page_offset;
        //     page_offset.x += self.offset.x;
        //
        //     let mut drag_offset =
        //         Vec2::new(self.offset.x, self.offset.y + page.offset.y);
        //
        //     if !page.frame.size.is_zero() {
        //         let page.dst_size = Size::new(page.dst_size.width(), page.dst_size.height());
        //
        //         if self.config.canvas.mode == Mode::Manga {
        //             if page.dst_size.ratio() >= 1.333 {
        //                 page_dst_size = Size::new(cw, page.dst_size.height());
        //                 page_offset.y += page.dst_size.height();
        //             } else {
        //                 // case 2
        //                 if cur_col == 0 {
        //                     cur_col = 1;
        //                     prev_height = page.dst_size.height() as u32;
        //                     drag_offset.x += cw_half;
        //
        //                     page_dst_size = Size::new(cw_half, page.dst_size.height());
        //                     page_offset.y += page.dst_size.height();
        //                 } else {
        //                     cur_col = 0;
        //                     page_dst_size = Size::new(cw_half, prev_height as f32);
        //
        //                     drag_offset.x += 0.0;
        //                 }
        //             }
        //         } else {
        //             // case 1
        //             if page.dst_size.ratio() >= 1.333 {
        //                 page_dst_size = Size::new(cw, page.dst_size.height());
        //                 page_offset.y += page.dst_size.height();
        //             // TODO: Leave blank always
        //             } else {
        //                 // case 2
        //                 if cur_col == 0 {
        //                     cur_col = 1;
        //                     prev_height = page.dst_size.height() as u32;
        //
        //                     page_dst_size = Size::new(cw_half, page.dst_size.height());
        //                     page_offset.y += page.dst_size.height();
        //                 } else {
        //                     cur_col = 0;
        //                     page_dst_size = Size::new(cw_half, prev_height as f32);
        //
        //                     drag_offset.x += cw_half;
        //                 }
        //             }
        //         }
        //     }
        //
        //     if page.is_passed(&self.config, &self.pool, self.data.clone(), &cur_view) {
        //         page.drag(drag_offset);
        //         page.draw(&mut self.buffer, self.size);
        //
        //         // dbg_cur_drawing += 1;
        //     }
        //
        //     // dbg_cur_drawing += 1;
        // }
        //
        // // dbg!(&dbg_cur_drawing);
        // // dbg!(&self.offset);
        //
        // Ok(())
    }

    //
    // linebreak:
    // Block {
    //   page: None,
    //   space: Space,
    // }
    //
    // ?FlexBlock
    // P: page
    // S: space
    // +-------------+
    // |S| P |S| P |S|
    // +-------------+
    // |  linebreak  |
    // +-------------+
    // |S| P |S| P |S|
    // +-------------+
    // Block {
    //   Vec<((Page, Space), LineBreak)>
    // }
    //
    // page.draw();
    // space.draw();
    pub fn draw_masonry(&mut self) -> eyre::Result<()> {
        todo!();

        // self.clamp_offset();
        // self.reset();
        //
        // let cur_view = self.cur_view();
        //
        // let mut page_offset = Vec2::default();
        // // let mut dbg_cur_drawing = 0;
        //
        // let mut cur_col = 0;
        // let mut prev_height = 0;
        // let mut page_dst_size = self.size;
        // let cw = self.size.width();
        // let cw_half = cw / 2.0;
        //
        // let Layout::Masonry { cols, gap_x, gap_y } = self.config.layout() else {
        //     unreachable!()
        // };
        //
        // let min_w = cw as u32 / cols;
        // let mut prev_width = 0;
        //
        // for page in self.pages.iter_mut() {
        //     page.offset = page_offset;
        //     page_offset.x += self.offset.x;
        //
        //     let max_w = (cw as u32) - (cur_col * min_w);
        //
        //     let mut drag_offset =
        //         Vec2::new(self.offset.x, self.offset.y + page.offset.y);
        //
        //     // case 1
        //     let page.dst_size = Size::new(page.dst_size.width(), page.dst_size.height());
        //
        //     let ratio = page.dst_size.ratio();
        //     let multiple = {
        //         if ratio >= 2.0 {
        //             5
        //         } else if ratio >= 1.777 {
        //             4
        //         } else if ratio >= 1.555 {
        //             3
        //         } else if ratio >= 1.333 {
        //             2
        //         } else {
        //             1
        //         }
        //     };
        //     let mut iw = min_w * multiple;
        //     let remain = (cw as u32) - (min_w * cur_col);
        //     if iw >= remain {
        //         iw = remain;
        //     }
        //
        //     if cur_col == 0 {
        //         prev_width = 0;
        //         drag_offset.x += 0.0;
        //     } else if cur_col <= *cols {
        //         prev_width += iw + gap_x;
        //         drag_offset.x += iw as f32;
        //
        //         if cur_col + multiple >= *cols {
        //             cur_col = 0;
        //         } else {
        //             cur_col += multiple;
        //         }
        //     }
        //
        //     if page.is_passed(&self.config, &self.pool, self.data.clone(), &cur_view) {
        //         page.drag(drag_offset);
        //         page.draw(&mut self.buffer, self.size);
        //
        //         // dbg_cur_drawing += 1;
        //     }
        // }
        //
        // // dbg!(&dbg_cur_drawing);
        // // dbg!(&self.offset);
        //
        // Ok(())
    }

    pub fn draw_single(&mut self) -> eyre::Result<()> {
        todo!();

        // let page = &mut self.pages[0];
        // let drag_offset = Vec2::new(self.offset.x, self.offset.y + page.offset.y);
        // let mut page_dst_size = self.size;
        //
        // // TODO:
        // // pollster::block_on(async {
        // //     page.set_size(self.data).await.unwrap();
        // // });
        //
        // page_dst_size = page.dst_size.resize_by_width(self.size.width());
        //
        // let (page_dst_size, filter) = {
        //     if page.frame.ty() == FrameTy::Image {
        //         (page_dst_size, self.config.page_image_resize_algo())
        //     } else {
        //         (page_dst_size, self.config.page_anime_resize_algo())
        //     }
        // };
        //
        // pollster::block_on(async {
        //     page.set_size(&self.data, true).await.unwrap();
        // });
        // page.resize(&self.data, filter)?;
        //
        // page.drag(drag_offset);
        // page.draw(&mut self.buffer, self.size);
        //
        // Ok(())
    }

    pub fn drag(&mut self, offset: Vec2) {
        self.offset += offset;
    }

    pub fn add_page(&mut self, page: Page) {
        self.pages.push(page);
    }

    pub fn reset(&mut self) {
        self.clamp_offset();

        self.buffer.copy_from_slice(&self.bg_img);
    }

    pub fn move_up(&mut self) {
        self.offset.y += self.step.y;
        self.cache_factor_up = 1.0;
        self.cache_factor_down = 0.5;
    }

    pub fn move_down(&mut self) {
        self.offset.y -= self.step.y;
        self.cache_factor_up = 0.5;
        self.cache_factor_down = 1.0;
    }

    pub fn move_left(&mut self) {
        self.offset.x += self.step.x;
    }

    pub fn move_right(&mut self) {
        self.offset.x -= self.step.x;
    }

    pub fn clamp_offset(&mut self) {
        self.offset.x = {
            let min = 0.0 - self.page_max_width;
            let max = min.abs() * 2.0;

            self.offset.x.clamp(min, max)
        };
        self.offset.y = {
            if self.offset.y > 0.0 {
                0.0
            } else {
                self.offset.y
            }
        };

        // crate::dbg!(self.offset);
    }

    pub fn resize(&mut self, new: Size) {
        self.size = new;
        self.bg_img = vec![self.bg; new.len()];
        self.buffer = self.bg_img.clone();
    }

    // TODO: mut cur_view
    //
    // move down
    //   self.top += step.y
    //   self.btn += step.y
    // move up
    //   self.top -= step.y
    //   self.btn -= step.y
    // move left
    //   self.left  -= step.y
    //   self.right -= step.y
    // move right
    //   self.left  += step.y
    //   self.right += step.y
    pub fn cur_view(&self) -> CurView {
        //
        // screen coordinate system:
        //
        //              -1
        //              |
        //              |
        //              +--------+
        //              |        |
        //              |  PTOP  |
        //              |        |
        //-1 -----------+--------+--- +1
        //              |        |
        //              |  VIEW  |
        //              |        |
        //              +--------+
        //              |        |
        //              |  PBTN  |
        //              |        |
        //              +--------+
        //              |
        //              |
        //              +1
        //

        let origin = Vec2::new(0.0, 0.0);
        let view = RectVertex::new(origin, self.size);

        // view's height.
        let h = self.size.height();
        let cache = h + (h * self.config.canvas_cache_limit());
        let cache_up = cache * self.cache_factor_up;
        let cache_down = cache * self.cache_factor_down;

        let h = cache + cache_up + cache_down;
        let origin = Vec2::new(0.0, 0.0 - h);
        let border = RectVertex::new(origin, self.size);

        CurView { view, border }
    }
}

impl CurView {
    pub fn is_page_inside(&self, page: &Page) -> (bool, bool) {
        let Self { view, border } = *self;
        let min = page.cast_vertex.min();
        let max = page.cast_vertex.max();

        let is_page_at_border = { (border.is_include(min) || border.is_include(max)) };
        let is_page_at_view = { view.is_include(min) || view.is_include(max) };

        // a
        (is_page_at_border, is_page_at_view)
    }
}
