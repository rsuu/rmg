use crate::{
    archive::utils::ArchiveType,
    config::rsconf::Config,
    img::utils::MetaSize,
    render::{
        keymap::KeyMap,
        once::Once,
        scroll::Scroll,
        turn::Turn,
        utils::{AsyncTask, Data, ForAsyncTask, Page, PageList, TaskResize, ViewMode},
        window::Canvas,
    },
};
use std::path::PathBuf;

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

    let mut canvas = Canvas::new(meta.window.width as usize, meta.window.height as usize);
    let page_list = {
        // sort by filename
        page_list.sort_by(|a, b| a.name.partial_cmp(&b.name).unwrap());

        PageList::new(page_list.to_owned())
    };

    // init
    let mut scroll = Scroll::new(&data, page_list, buffer_max, config.base.step as usize);

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

    // WARN: new thread
    // TODO: ?threadpool
    new_thread(&arc_task, &data);

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
            once.start(&mut canvas, &keymap, &data);
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

pub fn new_thread(arc_task: &AsyncTask, data: &Data) {
    let arc_task = arc_task.clone();
    let data = data.clone();

    let thread = move || loop {
        if arc_task.try_start(&data) {
            // TODO: How about sleep()
            std::thread::yield_now();
        } else {
            std::thread::sleep(std::time::Duration::from_millis(30));
        }
    };

    std::thread::spawn(thread);
}
