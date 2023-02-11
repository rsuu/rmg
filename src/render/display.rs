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
use std::{
    path::PathBuf,
    thread::{self, sleep_ms},
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
    let page_list = {
        // sort by filename
        page_list.sort_by(|a, b| a.name.partial_cmp(&b.name).unwrap());

        PageList::new(page_list.to_owned())
    };

    // init
    let mut scroll = Scroll::new(&data, page_list, buffer_max, config);

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
    for _ in 0..num_cpus::get() {
        new_thread(&arc_task, &data);
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
            once.start(&mut canvas, &keymap, &data);
        }

        // Bit OR Anim
        ViewMode::Turn => {
            todo!();

            //let mut turn = Turn::from_scroll(scroll);
            //turn.start(&mut canvas, &keymap, &data, );
        }
    }

    log::info!("*** EXIT ***");

    Ok(())
}

pub fn new_thread(arc_task: &AsyncTask, data: &Data) {
    let arc_task = arc_task.clone();
    let data = data.clone();

    let f = move || loop {
        if let Some(index) = arc_task.try_start(&data) {
            log::info!(
                "
Thread: {:?}
task: {index}",
                thread::current().id(),
            );
        } else {
            sleep_ms(10);
            //thread::yield_now();
        }
    };

    thread::spawn(f);
}