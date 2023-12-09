use crate::{
    render, sleep_ms, thread, ArchiveType, AsyncTask, Canvas, Config, Data, ForAsyncTask, KeyMap,
    MetaSize, Once, Page, PageList, PathBuf, Scroll, TaskResize, Turn, ViewMode,
};

// TODO:
// InitMode -> Scroll
//          -> Once
//          -> Turn
pub struct InitMode {}

/// display images
pub fn cat_img(
    config: &Config,
    page_list: &mut Vec<Page>,
    meta: MetaSize<u32>,
    path: PathBuf,
    archive_type: ArchiveType,
) -> anyhow::Result<()> {
    // window.set_position(20, 20);
    let mut keymap = KeyMap::new();
    KeyMap::update(&mut keymap, config);

    let buffer_max = meta.window.width as usize * meta.window.height as usize;
    let data = Data::new(archive_type, path, meta, config.base.filter);

    let mut canvas = Canvas::new(
        meta.window.width as usize,
        meta.window.height as usize,
        config,
    );
    let page_list = PageList::new(page_list);

    // TODO: Scroll -> InitMode
    let mut scroll = Scroll::new(
        &data,
        page_list,
        buffer_max,
        config,
        data.meta.window.width as usize,
    );

    let vec = scroll.page_list.list;
    let page_len = vec.len();

    // Vec<Page> -> Vec<TaskResize>
    let arc_task = <AsyncTask as ForAsyncTask>::new(
        vec.iter()
            .map(|page| TaskResize::new(page.clone()))
            .collect(),
    );

    let mode = {
        if page_len == 1 {
            ViewMode::Once
        } else {
            config.base.view_mode
        }
    };

    for _ in 0..(config.base.thread_limit) {
        render::new_thread(&arc_task, &data);
    }

    match mode {
        // Bit
        ViewMode::Scroll => {
            Scroll::start(&mut scroll, config, &mut canvas, &keymap, &data, &arc_task);
        }

        // Bit/Anim
        ViewMode::Once => {
            let mut once = Once::from_scroll(scroll);
            once.start(&mut canvas, &keymap, &data, config);
        }

        // Bit/Anim
        ViewMode::Turn => {
            todo!();

            //let mut turn = Turn::from_scroll(scroll);
            //turn.start(&mut canvas, &keymap, &data, );
        }
    }

    tracing::info!("*** EXIT ***");

    Ok(())
}
