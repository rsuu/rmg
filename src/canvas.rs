pub mod buffer;
pub mod draw;
pub mod gesture;
pub mod layout;
pub mod page;
pub mod state;
pub mod task;

use eyre::OptionExt;

use crate::*;

use std::sync::Arc;

// 1. LocalSpace(Frame)
// 2. WorldSpace(Page)
// 3. ViewSpace(Page)
// 4. ClipSpace(Canvas)
// 5. ScreenSpace(Window)
//
// screen coordinate system:
//
//              -N
//              |
//              |
//              |
//-N -----------+----------- +N
//              |
//              |
//              |
//              +N
//
// min: (0,0)
// max: (w,h)
pub struct Canvas {
    pub config: Config,
    pub size: Size,
    pub buffer: Buffer,

    pub pages: Pages,
    pub page_max_width: f32,
    // pub flag_get_all_frame_size: bool,
    /// view's offset
    pub offset: Vec2,

    /// mouse step
    pub step: Vec2,
    pub mode: Mode,
    pub action: Action,
    /// background `RGBA` color in `u32` format.
    pub bg: u32,
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
pub struct ViewArea<T = Rect> {
    pub view: T,
    pub border: T,
}

impl Canvas {
    pub fn new(config: Config, data: DataType) -> eyre::Result<Self> {
        // dbg!(&config);

        let size = config.canvas.size;
        let Size { width, height } = size;

        let bg = config.bg();
        let tmp = vec![bg; size.len()];
        let fname_padding = 10;
        let empty_pages = data.gen_empty_pages(fname_padding)?;
        let data = Arc::new(data);

        Ok(Self {
            step: config.canvas_step(),
            buffer: tmp.clone(),
            bg_img: tmp,
            mode: Mode::default(),
            action: Action::default(),
            offset: Vec2::default(),
            cache_factor_up: 0.5,
            cache_factor_down: 1.0,

            // flag_get_all_frame_size: false,
            top_line: 0.0,
            page_max_width: size.width(),
            pool: Pool::new(empty_pages.clone()),
            pages: empty_pages,

            size,
            data,
            bg,
            config,
        })
    }

    pub fn draw(&mut self) -> eyre::Result<()> {
        match &self.action {
            Action::Gesture { .. } => {
                self.draw_gesture_path();

                return Ok(());
            }

            _ => {}
        }

        match self.config.layout() {
            Layout::Vertical { .. } => self.vertical_draw()?,
            Layout::Horizontal { .. } => self.horizontal_draw()?,
            Layout::Double { .. } => self.double_draw()?,
            Layout::Multi { .. } => self.multi_draw()?,
            Layout::Masonry { .. } => self.masonry_draw()?,
            Layout::Single { .. } => self.single_draw()?,

            _ => todo!(),
        }

        Ok(())
    }

    pub fn drag(&mut self, offset: Vec2) {
        self.offset += offset;
    }

    pub fn push(&mut self, page: Page) {
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

    pub fn center_point(&self) -> Vec2 {
        Vec2::new(self.size.width() / 2.0, self.size.height() / 2.0)
    }
}

impl Canvas {
    // TODO: cache `page.dst_size` and `page.frame.size`
    // Block {
    //   page: Page
    // }
    pub fn vertical_draw(&mut self) -> eyre::Result<()> {
        self.reset();

        let layout = &self.config.canvas.layout;
        let Layout::Vertical { align } = layout else {
            unreachable!()
        };

        let view_area = self.view_area(layout);
        let max_width = self.page_max_width;

        let cw = self.size.width();
        let cw_half = cw / 2.0;

        let mut elems: Vec<&mut Page> = Vec::with_capacity(10);
        let mut page_offset: Vec2<f32> = Vec2::default();

        'l: for page in self.pages.iter_mut() {
            // no async
            if page.frame.size.is_zero() {
                page.load(&self.data, false)?;
                page.dst_size = page.frame.size.resize_by_width(max_width);
                page.cast_vertex = Rect::new_at_zero(page.dst_size);

                return Ok(());
            }

            // dbg!(&page.index, &page.cast_vertex);
            // FIXME:
            let ew = page.dst_size.width();
            let padding_left = {
                match align {
                    Align::Center => (cw - ew) / 2.0,
                    Align::Left => 0.0,
                    Align::Right => cw - ew,
                }
            }
            .abs();

            // 1. drag
            let drag_offset =
                Vec2::new(self.offset.x + padding_left, self.offset.y + page_offset.y);
            page.drag(drag_offset);

            // 2. update
            let h = page.dst_size.height();
            page_offset.y += h;

            let flag = page.is_passed(&self.config, &view_area);
            if !flag {
                // dbg!("skip", page.index);

                continue 'l;
            }

            match page.state {
                // 4. drawing
                State::Done => {
                    elems.push(page);
                }

                // 3. loading
                State::Waiting => {
                    let data = self.data.clone();

                    if page.frame.size.is_zero() {
                        self.pool.task_load(page, data);
                        page.dst_size = page.frame.size.resize_by_width(max_width);
                    } else {
                        self.pool.task_resize(page, data, &self.config);
                    }
                }

                _ => {}
            }
        }

        // TODO:
        // if let Some(nav) = self.ui_nav_bar {
        //     elems.push(nav);
        // }

        // TODO: case 2
        // for elem in self.ui_elems.iter() {
        //     elem.draw();
        // }

        // 6. drawing
        // dbg!(elems.len());
        for elem in elems {
            // dbg!(
            //     &elem.index,
            //     &elem.state,
            //     &elem.dst_size,
            //     &elem.cast_vertex
            // );

            elem.draw(&mut self.buffer, self.size);
        }

        // dbg!(&dbg_cur_drawing);
        // dbg!(&self.offset);

        Ok(())
    }

    pub fn view_area(&self, layout: &Layout) -> ViewArea {
        // screen coordinate system:
        match layout {
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
            Layout::Vertical { .. } => {
                let origin = Vec2::new(0.0, 0.0);
                let view = Rect::new(origin, self.size);

                let h = self.size.height();
                let limit = self.config.canvas.cache_limit as f32;
                let up = h * self.cache_factor_up * limit;
                let down = h * self.cache_factor_down * limit;
                let mut border = view.clone();
                border.min.y -= up;
                border.max.y += down;

                ViewArea { view, border }
            }

            //
            //              -1
            //              |
            //              |
            //              |
            //-1 --+--------+--------+--------+--- +1
            //     |        |        |        |
            //     |   PL   |  VIEW  |   PR   |
            //     |        |        |        |
            //     +--------+--------+--------+
            //              |
            //              |
            //              |
            //              +1
            //
            // * canvas.size == window.size
            //
            // Layout::Horizontal { .. }=>{},
            //
            // Layout::Double { .. } => {}
            //
            _ => todo!(),
        }
    }

    pub fn draw_gesture_path(&mut self) {
        let Action::Gesture {
            fill,
            path,
            stroke_width,
        } = &self.action
        else {
            unreachable!()
        };

        let diameter = stroke_width;

        for origin in path.iter() {
            let r = diameter / 2.0;
            let circle = Circle::new(*origin, r);

            circle.draw(&mut self.buffer, self.size, *fill);
        }
    }
}

impl Canvas {
    pub fn single_draw(&mut self) -> eyre::Result<()> {
        self.reset();

        let layout = &self.config.canvas.layout;
        let Layout::Single { zoom, mouse_pos } = layout else {
            unreachable!()
        };

        // dbg!(&zoom);

        let max_width = self.page_max_width;
        let page = self.pages.first_mut().ok_or_eyre("None")?;

        // load file
        if page.frame.size.is_zero() {
            let algo = fir::ResizeAlg::Nearest;

            page.load(&self.data, true)?;

            page.dst_size = page.frame.size.resize_by_width(max_width);

            let frame = Frame::resize(page.tmp_blob.as_slice(), page.dst_size, algo)?;

            page.frame = frame;
            page.cast_vertex = page.frame.vertex;
            page.state = State::Done;
        }

        let zoom = *zoom;
        let zoom_size = page.dst_size * zoom;

        // TODO: zoom at mouse pos
        {
            let algo = fir::ResizeAlg::Nearest;

            let frame = Frame::resize(page.tmp_blob.as_slice(), zoom_size, algo)?;

            page.frame = frame;
            page.cast_vertex = page.frame.vertex;

            // no
            // page.dst_size = zoom_size;
        }

        // TODO: align
        let drag_offset = Vec2::new(
            self.offset.x * zoom + self.size.width() * zoom / 2.0,
            // self.offset.y * zoom + self.size.height() * zoom / 2.0,
            self.offset.y * zoom,
        );
        page.drag(drag_offset);

        page.draw(&mut self.buffer, self.size);

        Ok(())
    }
}

impl Canvas {
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
    // |    break    |
    // +-------------+
    // |S| P |S| P |S|
    // +-------------+
    // Block {
    //   Vec<((Page, Space), LineBreak)>
    // }
    //
    // page.draw();
    // space.draw();
    pub fn masonry_draw(&mut self) -> eyre::Result<()> {
        todo!();
    }
}

impl Canvas {
    pub fn horizontal_draw(&mut self) -> eyre::Result<()> {
        // self.page_max_width = f32::INFINITY;

        Ok(())
    }
}

impl Canvas {
    // TODO:
    // REFS: https://kittenyang.com/layout-algorithm/
    pub fn multi_draw(&mut self) -> eyre::Result<()> {
        let Layout::Multi { nums } = self.config.layout() else {
            eyre::bail!("")
        };

        let nums = nums.clone();

        let r43 = 4.0 / 3.0;
        let y_heights = vec![0.0; nums];
        let mut cur_idx = 0;

        for page in self.pages.iter_mut() {
            if page.dst_size.ratio() >= r43 {
                if cur_idx == 0 {
                    todo!("?resize frame")
                } else {
                    let range = &y_heights[0..cur_idx];
                    let mut sum = 0.0;
                    for f in range {
                        sum += f;
                    }

                    let frame_width = self.size.width() - sum;

                    todo!("?resize")
                }

                // breaking
                cur_idx = 0;
            } else {
                if cur_idx < nums {
                    cur_idx += 1;
                } else {
                    cur_idx = 0;
                }
            }
        }

        Ok(())
    }
}

impl Canvas {
    // Block {
    //   page: Page,
    //   space: Space,
    // }
    //
    // page.draw();
    // space.draw();
    pub fn double_draw(&mut self) -> eyre::Result<()> {
        self.reset();

        let layout = &self.config.canvas.layout;
        let Layout::Double { align, gap } = layout else {
            unreachable!()
        };

        let view_area = self.view_area(layout);
        let max_width = self.page_max_width;

        let cw = self.size.width();
        let cw_half = cw / 2.0;

        // let mut no_layout = false;

        let elems: Vec<&mut Page> = Vec::with_capacity(10);
        let page_offset: Vec2<f32> = Vec2::default();

        // {0, 1}
        let cur_col = 0;

        // 'l: for page in self.pages.iter_mut() {
        //     let mut cur_line = vec![];
        //
        //     if cur_col == 0 {
        //         cur_line.push(&mut page_left);
        //
        //         cur_col = 1;
        //     } else {
        //         if page.dst_size.ratio() > 1.333 {
        //             // TODO: push-empty-rect + line-break
        //         } else {
        //             cur_line.push(&mut page_right);
        //         }
        //
        //         cur_col = 0;
        //     }
        //
        //     // Align
        // }

        todo!();
    }
}

impl ViewArea {
    pub fn is_page_hover(&self, page: &Page) -> (bool, bool) {
        // dbg!(&self, page.cast_vertex);

        let Self { view, border } = *self;
        let page = &page.cast_vertex;

        let is_hover_edge = {
            if border.is_include(page.min()) || border.is_include(page.max()) {
                true
            } else {
                false
            }
        };
        let is_hover_view = {
            // if page.max().y >= view.min().y && page.min().y <= view.max().y {
            if view.is_include(page.min()) || view.is_include(page.max()) {
                true
            } else {
                false
            }
        };

        // dbg!((page, border, view));

        (is_hover_edge, is_hover_view)
    }
}

fn center_xy(canvas: Size, img: Size) -> Vec2 {
    let canvas_center_x = canvas.width() / 2.0;
    let canvas_center_y = canvas.height() / 2.0;

    let img_center_x = img.width() / 2.0;
    let img_center_y = img.height() / 2.0;

    Vec2::new(
        canvas_center_x - img_center_x,
        canvas_center_y - img_center_y,
    )
}

// REFS: https://www.codeandweb.com/texturepacker
// REFS: https://www.david-colson.com/2020/03/10/exploring-rect-packing.html
// REFS: https://codeincomplete.com/articles/bin-packing/
// REFS: https://www.joshwcomeau.com/css/understanding-layout-algorithms/
// REFS: https://blog.vjeux.com/2012/image/
// REFS: https://carmencincotti.com/2022-05-02/homogeneous-coordinates-clip-space-ndc/
