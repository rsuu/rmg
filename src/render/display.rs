use crate::{
    archive::ArchiveType,
    config::rsconf::Config,
    img::size::MetaSize,
    render::{
        keymap::KeyMap,
        once::Once,
        scroll::{self, Scroll},
        turn::Turn,
        view::{ArcTmpBuffer, Data, ImgType, Page, PageList, State, ViewMode},
        window::Canvas,
    },
    utils::err::Res,
};
use tracing::{debug, info};
use std::{
    path::PathBuf,
    sync::{Arc, RwLock},
};

/// display images
pub fn cat_img(
    config: &Config,
    page_list: &mut Vec<Page>,
    meta_size: MetaSize<u32>,
    path: &str,
    archive_type: ArchiveType,
) -> Res<()> {
    let screen_size = meta_size.screen;
    let window_size = meta_size.window;
    let buffer_max = window_size.width as usize * window_size.height as usize;

    let keymaps = KeyMap::new();
    let mut mode = config.base.view_mode;

    page_list.sort_by(|a, b| a.name.partial_cmp(&b.name).unwrap());

    let mut page_list = PageList::new(page_list.to_owned());

    for (idx, page) in page_list.list.iter_mut().enumerate() {
        page.number = idx;
    }

    let data = Data::new(
        archive_type,
        PathBuf::from(path),
        screen_size,
        window_size,
        config.base.filter,
    ); // use for resize image

    let arc_state = Arc::new(RwLock::new(State::Nothing));
    let arc_buffer = ArcTmpBuffer::new_arc();

    let mut canvas = Canvas::new(window_size.width as usize, window_size.height as usize);

    let mut scroll = Scroll::new(&data, page_list, buffer_max, config.base.step as usize);

    init(&mut mode, &mut scroll, &data); // init

    match mode {
        // Bit
        ViewMode::Scroll => {
            Scroll::start(
                config,
                &mut scroll,
                &mut canvas,
                &keymaps,
                &data,
                &arc_state,
                &arc_buffer,
            );
        }

        // Bit OR Anim
        ViewMode::Once => {
            // TODO: scale gif?
            Once::start(&mut scroll, &mut canvas, &keymaps);
        }

        // Bit OR Anim
        ViewMode::Turn => {
            todo!();

            let mut turn = Turn::from_scroll(scroll);
            turn.start(&mut canvas, &keymaps, &data, &arc_state, &arc_buffer);
        }
    }

    info!("*** EXIT ***");

    Ok(())
}

///
pub fn init(mode: &mut ViewMode, scroll: &mut Scroll, data: &Data) {
    let mut tmp = (0, 0);

    let mut len = 0;
    let mut anim_count = 0;

    scroll.head = 1;
    scroll.tail = 0;

    debug!("{:#?}", &scroll.page_list.list);

    // [Head, 1, 2, Tail]
    match scroll.page_list.len() {
        0..=2 => {
            unreachable!()
        }

        3 => {
            tmp = load_next(scroll, data);

            *mode = ViewMode::Once;

            return;
        }

        _ => {
            while len <= scroll.mem_limit && scroll.not_tail() {
                scroll.tail += 1;
                tmp = load_next(scroll, data);
                len += tmp.0;
                anim_count += tmp.1;
            }

            debug!("{}", scroll.tail);
        }
    }

    if anim_count >= 1 {
        *mode = ViewMode::Once;

        return;
    }

    debug!("    len = {}", len);
}

pub fn load_next(scroll: &mut Scroll, data: &Data) -> (usize, usize) {
    let tail = scroll.page_list.get_mut(scroll.tail);
    let pos = tail.pos;

    let (ty, mut buffer, format) =
        scroll::load_file(&data.archive_type, data.path.as_path(), pos).unwrap();
    let (meta, pts) =
        scroll::load_img(&format, &mut buffer, &data.screen_size, &data.window_size).unwrap();

    tail.ty = ty;
    tail.resize = meta.fix;
    tail.pts = pts;

    scroll::resize_page(tail, &mut buffer, &meta, &data.filter, &data.window_size);

    tail.is_ready = true;

    (tail.len(), (tail.ty == ImgType::Anim) as usize)
}
