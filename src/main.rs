use rmg::{cat_img, print_help, ArchiveType, Config, ForExtract, MetaSize, PathBuf, TMetaSize};

fn main() {
    init_log();

    let config = {
        let mut res = Config::new();

        let _ = res.try_from_config_file().unwrap_or_else(|_| {});
        let _ = res.try_from_cli().unwrap_or_else(|_| {});

        res
    };
    let meta_size = MetaSize::new(0, 0, config.base.size.width, config.base.size.height, 0, 0);

    tracing::info!("config: {:#?}", &config);
    tracing::info!("meta_size = {:#?}", &meta_size);

    let path = {
        if let Some(ref tmp) = config.cli.file_path {
            PathBuf::from(tmp)
        } else {
            print_help();
        }
    };
    let archive_type = ArchiveType::new(path.as_path()).unwrap();
    let file_list = archive_type.get_list(path.as_path()).unwrap();
    let mut page_list = file_list.to_page_list(config.base.rename_pad as usize);

    println!("Open: {}", path.as_path().display());
    tracing::trace!("file_list: {:#?}", &file_list);
    tracing::trace!("page_list: {:#?}", &page_list);

    if let Err(e) = cat_img(&config, &mut page_list, meta_size, path, archive_type) {
        tracing::error!("{:#?}", e);
    }
}

fn init_log() {
    //env_logger::init();

    use tracing_subscriber::{fmt, prelude::*, registry, EnvFilter};

    // e.g. RUST_LOG="rmg::render::scroll=debug"
    let filter = EnvFilter::builder().with_regex(true).from_env_lossy();
    let fmt = fmt::layer().without_time().with_thread_names(true);

    registry().with(fmt).with(filter).init();

    //tracing::info!("init_log()");
}
