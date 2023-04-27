use crate::{
    render, sleep_ms, thread, ArchiveType, AsyncTask, Canvas, Config, Data, ForAsyncTask, KeyMap,
    MetaSize, Once, Page, PageList, PathBuf, Scroll, TaskResize, Turn, ViewMode,
};

/// display images
pub fn cat_img(
    config: &Config,
    page_list: &mut Vec<Page>,
    meta: MetaSize<u32>,
    path: PathBuf,
    archive_type: ArchiveType,
) -> anyhow::Result<()> {
    let keymap = KeyMap::new();
    let buffer_max = meta.window.width as usize * meta.window.height as usize;
    let data = Data::new(archive_type, path, meta, config.base.filter); // use for resize image

    let mut canvas = Canvas::new(
        meta.window.width as usize,
        meta.window.height as usize,
        config,
    );
    let page_list = PageList::new(page_list);

    // init
    let mut scroll = Scroll::new(
        &data,
        page_list,
        buffer_max,
        config,
        data.meta.window.width as usize,
    );

    let arc_task = {
        let mut tmp: Vec<TaskResize> = vec![];

        for page in scroll.page_list.list.iter() {
            tmp.push(TaskResize::new(page.clone()));
        }

        <AsyncTask as ForAsyncTask>::new(tmp)
    };

    let mode = {
        if scroll.page_list.list.len() == 1 {
            ViewMode::Once
        } else {
            config.base.view_mode
        }
    };

    // NOTE: new threads
    for _ in 0..(config.base.thread_limit) {
        render::new_thread(&arc_task, &data);
    }

    match mode {
        // Bit
        ViewMode::Scroll => {
            // TODO: ?support Anim
            Scroll::start(&mut scroll, config, &mut canvas, &keymap, &data, &arc_task);
        }

        // Bit OR Anim
        ViewMode::Once => {
            // TODO: ?scale gif
            let mut once = Once::from_scroll(scroll);
            once.start(&mut canvas, &keymap, &data, config);
        }

        // Bit OR Anim
        ViewMode::Turn => {
            todo!();

            //let mut turn = Turn::from_scroll(scroll);
            //turn.start(&mut canvas, &keymap, &data, );
        }
    }

    tracing::info!("*** EXIT ***");

    Ok(())
}
