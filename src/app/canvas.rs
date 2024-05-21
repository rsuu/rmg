// TODO: App { canvas, }

use crate::*;

use eyre::OptionExt;
use std::sync::Arc;

pub type Elems = Vec<Page>; // TODO

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
    pub config: Config, // TODO: rm
    pub buffer: Buffer,

    pub elems: Elems, // TODO: mv to App.elems
    pub page_max_width: f32,
    // pub flag_get_all_frame_size: bool,
    /// view's offset
    pub offset: Vec2, // TODO: mv to App

    /// mouse step
    pub step: Vec2, // TODO: mv to App
    pub page_dire: PageDirection, // TODO: mv to App
    pub action: Action,           // TODO: mv to App
    /// background `RGBA` color in `u32` format.
    pub bg: u32,
    /// background image.
    pub bg_img: Vec<u32>,

    /// limit cache size when scroll.
    cache_factor_up: f32,
    cache_factor_down: f32,

    pool: Pool, // TODO: mv to App
    /// archive's info.
    data: Arc<DataType>, // TODO: mv to App
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

        let bg = config.bg();
        let tmp = vec![bg; size.len()];
        let fname_padding = 10;
        let empty_pages = data.gen_empty_pages(fname_padding)?;
        let data = Arc::new(data);

        Ok(Self {
            step: config.canvas_step(),
            buffer: Buffer::new(tmp.clone(), size),
            // TODO:from config
            //      from winit
            // init_size:
            bg_img: tmp,
            dire: PageDirection::default(),
            action: Action::default(),
            offset: Vec2::default(),
            cache_factor_up: 0.5,
            cache_factor_down: 1.0,

            // flag_get_all_frame_size: false,
            top_line: 0.0,
            page_max_width: size.width(),
            pool: Pool::new(empty_pages.clone()),
            elems: empty_pages,
            data,
            bg,
            config,
        })
    }

    pub fn max_w(&self) -> f32 {
        self.size().width()
    }

    pub fn min_w(&self, nums: f32) -> f32 {
        self.max_w() / nums
    }

    pub fn max_h(&self) -> f32 {
        self.size().height()
    }

    pub fn min_h(&self, nums: f32) -> f32 {
        self.size().height() / nums
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

    pub fn size(&self) -> Size {
        self.buffer.size
    }

    pub fn drag(&mut self, offset: Vec2) {
        self.offset += offset;
    }

    pub fn push(&mut self, page: Page) {
        self.elems.push(page);
    }

    pub fn reset(&mut self) {
        self.buffer.vec.copy_from_slice(&self.bg_img);
    }

    pub fn move_up(&mut self) {
        self.offset.y += self.step.y;
        self.cache_factor_up = 1.0;
        self.cache_factor_down = 0.5;
    }

    pub fn move_down(&mut self) {
        self.offset.y -= self.step.y;
        self.cache_factor_up = 0.5;
        self.cache_factor_down = 1.0; // TODO: factor -> page nums
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
        self.bg_img = vec![self.bg; new.len()];
        self.buffer = Buffer::new(self.bg_img.clone(), new);
    }

    pub fn center_point(&self) -> Vec2 {
        Vec2::new(self.size().width() / 2.0, self.size().height() / 2.0)
    }
}

impl Canvas {
    // TODO: cache `page.dst_size` and `page.frame.size`
    // Block {
    //   page: Page
    // }
    pub fn vertical_draw(&mut self) -> eyre::Result<()> {
        self.clamp_offset();
        self.reset();

        let layout = &self.config.canvas.layout;
        let Layout::Vertical { align } = layout else {
            unreachable!()
        };

        let view_area = self.view_area(layout);
        let max_w = self.page_max_width;

        let cw = self.size().width();
        let cw_half = cw / 2.0;

        let mut elems: Vec<&mut Page> = Vec::with_capacity(10);
        let mut page_offset: Vec2<f32> = Vec2::default();

        'l: for page in self.elems.iter_mut() {
            // no async
            if page.frame.size.is_zero() {
                page.load(&self.data, false)?;
                page.dst_size = page.frame.size.resize_by_width(max_w);

                return Ok(());
            }

            // dbg!(&page.index, &page.cast_vertex);

            // TODO: flag_key
            // TODO: rewrite align
            // TODO: if fullscreen { padding } else { skip }
            // 1. drag
            let padding_left = center_x(cw, page.dst_size.width());
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
                // 4. draw
                State::Done => {
                    elems.push(page);
                }

                // 3. loading
                State::Waiting => {
                    let data = self.data.clone();

                    if page.frame.size.is_zero() {
                        self.pool.task_load(page, data);
                    } else {
                        page.dst_size = page.frame.size.resize_by_width(max_w);

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

        // 6. draw
        // dbg!(elems.len());
        for elem in elems {
            // dbg!(
            //     &elem.index,
            //     &elem.state,
            //     &elem.dst_size,
            //     &elem.cast_vertex
            // );

            elem.draw(&mut self.buffer);
        }

        Ok(())
    }

    // FIXME: bug in fullscreen
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
                let view = Rect::new(origin, self.size());

                let h = self.size().height();
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
            Layout::Double { .. } => {
                // FIXME:
                let origin = Vec2::new(0.0, 0.0);
                let view = Rect::new(origin, self.size());

                let h = self.size().height();
                let limit = self.config.canvas.cache_limit as f32;
                let up = h * self.cache_factor_up * limit;
                let down = h * self.cache_factor_down * limit;
                let mut border = view.clone();
                border.min.y -= up;
                border.max.y += down;

                ViewArea { view, border }
            }

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

            circle.draw(&mut self.buffer, *fill);
        }
    }
}

impl Canvas {
    // TODO: center
    pub fn single_draw(&mut self) -> eyre::Result<()> {
        self.reset();

        let max_w = self.page_max_width;
        let size = self.size();
        let page = self.elems.first_mut().ok_or_eyre("None")?;

        let Layout::Single {
            flag_scroll,
            ref dire,
            ref mouse_pos,
            ..
        } = &mut self.config.canvas.layout
        else {
            unreachable!()
        };

        // load img
        if page.frame.size.is_zero() {
            let algo = fir::ResizeAlg::Nearest;

            page.load(&self.data, true)?;

            page.dst_size = page.frame.size.resize_by_width(max_w);

            let frame = Frame::resize(page.tmp_blob.as_slice(), page.dst_size, algo)?;

            // align
            let drag_offset = center_xy(size, page.dst_size);

            page.frame = frame;
            page.frame.vertex = page.frame.vertex.translate(drag_offset.x, drag_offset.y);
            page.cast_vertex = page.frame.vertex;
            page.state = State::Done;
        }

        // TODO: flag_key
        let Vec2 { x, y } = self.offset;
        page.cast_vertex = page.cast_vertex.translate(x, y);
        self.offset = Default::default();

        if *flag_scroll {
            // TODO: how align

            let algo = fir::ResizeAlg::Nearest;
            let scale = 1.1_f32;
            let factor = scale.powf(*dire);
            let dst_size = page.dst_size * factor;

            let frame = Frame::resize(page.tmp_blob.as_slice(), dst_size, algo)?;
            page.frame = frame;
            page.dst_size = dst_size;

            page.zoom_at(*mouse_pos, factor);

            *flag_scroll = false;
        }

        page.draw(&mut self.buffer);

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
        let Layout::Multi { cols_nums } = self.config.layout() else {
            eyre::bail!("")
        };

        let cols_nums = cols_nums.clone();
        let r43 = 4.0 / 3.0;
        let max_w = self.size().width();
        let min_w = max_w / (cols_nums as f32);

        let mut cur_cols = 0;
        let mut cur_total_w = 0.0;
        let mut progress = 0;
        let mut head_h = 0.0;
        let mut elem_offset = Vec2::new(0.0, 0.0);

        let mut draw_elems = Vec::with_capacity(10);

        for elem in self.elems.iter_mut() {
            let need_laying = { elem.index == progress };
            let is_head_elem = { cur_cols == 0 };

            // laying
            if need_laying {
                if is_head_elem {
                    head_h = elem.dst_size.height();
                }

                // resize
                elem.dst_size.resize_by_height(head_h);
                cur_total_w += elem.dst_size.width();

                // drag and align
                let padding_left = 0.0;
                let mut drag_offset =
                    Vec2::new(self.offset.x + padding_left, self.offset.y + elem_offset.y);
                elem.drag(drag_offset);

                // try breaking
                let need_break = { cur_cols == cols_nums || cur_total_w > max_w };

                if need_break {
                    elem_offset.y += head_h;

                    cur_cols = 0;
                } else {
                    cur_cols += 1;
                }
            }

            // TODO: state
            draw_elems.push(elem);
        }

        for elem in draw_elems.iter_mut() {
            elem.draw(&mut self.buffer);
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
        self.clamp_offset();
        self.reset();

        // TODO: align
        // TODO: gap
        // TODO: page_dire
        let layout = &self.config.canvas.layout;
        let Layout::Double { align, gap } = layout else {
            unreachable!()
        };

        let view_area = self.view_area(layout);
        let max_w = self.page_max_width;

        let max_page_nums = 2.0;
        let max_w = self.size().width();
        let min_w = max_w / max_page_nums;

        //
        // +-----+-----+  case 1
        // |     |     |
        // |     |     |
        // |     |     |
        // +-----+-----+
        //
        // +-----------+  case 2
        // |           |
        // |           |
        // |           |
        // +-----------+
        //
        // +-----+-----+  case 3
        // |     |.....|  `.` is mean `empty`
        // |     |.....|
        // |     |.....|
        // +-----+-----+
        // |           |
        // |           |
        // |           |
        // +-----------+
        //

        let r43 = 4.0 / 3.0;

        let mut draw_elems: Vec<&mut Page> = Vec::with_capacity(10);
        let mut elem_offset: Vec2<f32> = Vec2::default();
        let mut elem_rank = 0;
        let mut head_size = Default::default();

        'l: for elem in self.elems.iter_mut() {
            if elem.frame.size.is_zero() {
                elem.load(&self.data, false)?;
                elem.dst_size = elem.frame.size.resize_by_width(max_w);
                elem.cast_vertex = Rect::new_at_zero(elem.dst_size);

                return Ok(());
            }

            let flag = elem.is_passed(&self.config, &view_area);
            if !flag {
                // dbg!("skip", elem.index);

                continue 'l;
            }

            // TODO: Align
            let padding_left = 0.0;
            let mut drag_offset = Vec2::new(self.offset.x, self.offset.y + elem_offset.y);

            // lhs
            if elem_rank == 0 {
                // case 2
                if elem.dst_size.ratio() > r43 {
                    elem.dst_size.resize_by_width(max_w);

                    // next elem is at lhs
                    elem_rank = 0;

                    elem_offset.y += elem.dst_size.height();

                // maybe case 1
                } else {
                    elem.dst_size = elem.dst_size.resize_by_width(min_w);

                    elem_rank = 1;
                }

                head_size = elem.dst_size;

            // rhs
            } else {
                // case 3
                if elem.dst_size.ratio() > r43 {
                    elem.dst_size.resize_by_width(max_w);
                    // elem.offset.y += head_size.h
                    // push empty elem

                    todo!("")

                // case 1
                } else {
                    // dbg!("case 1");

                    drag_offset.x += head_size.width() + gap.x;
                    elem_offset.y += head_size.height() + gap.y;
                }

                // if elem_rank + 1 > max {
                //    elem_rank = 0;
                // }
                elem_rank = 0;
            }

            elem.drag(drag_offset);

            match elem.state {
                State::Done => {
                    draw_elems.push(elem);
                }

                State::Waiting => {
                    let data = self.data.clone();

                    if elem.frame.size.is_zero() {
                        self.pool.task_load(elem, data);
                    } else {
                        let w = {
                            if elem.frame.size.ratio() > r43 {
                                max_w
                            } else {
                                min_w
                            }
                        };

                        elem.dst_size = elem.frame.size.resize_by_width(w);

                        self.pool.task_resize(elem, data, &self.config);
                    }
                }

                _ => {}
            }
        }

        for elem in draw_elems.iter_mut() {
            elem.draw(&mut self.buffer);
        }

        Ok(())
        // todo!();
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

// REFS: https://www.codeandweb.com/texturepacker
// REFS: https://www.david-colson.com/2020/03/10/exploring-rect-packing.html
// REFS: https://codeincomplete.com/articles/bin-packing/
// REFS: https://www.joshwcomeau.com/css/understanding-layout-algorithms/
// REFS: https://blog.vjeux.com/2012/image/
// REFS: https://carmencincotti.com/2022-05-02/homogeneous-coordinates-clip-space-ndc/
