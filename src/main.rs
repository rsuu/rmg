use rmg::{
    archive::utils::{ArchiveType, ForExtract},
    config::rsconf::{print_help, Config},
    img::utils::{MetaSize, TMetaSize},
    render::display,
};
use std::path::PathBuf;

fn main() {
    fn init_log() {
        use tracing_subscriber::{fmt, prelude::*, registry, EnvFilter};

        // e.g. RUST_LOG="rmg::render::scroll=debug"
        let env_filter = EnvFilter::builder().with_regex(true).from_env_lossy();
        let log_fmt = fmt::layer().without_time().with_thread_names(true);

        registry().with(log_fmt).with(env_filter).init();

        tracing::info!("init_log()");
    }

    init_log();

    let config = {
        let mut res = Config::new();

        let _ = res.try_from_config_file().unwrap_or_else(|_| {});
        let _ = res.try_from_cli().unwrap_or_else(|_| {});

        res
    };

    let meta_size = MetaSize::new(0, 0, config.base.size.width, config.base.size.height, 0, 0);

    tracing::trace!("{:#?}", config);
    tracing::trace!("meta_size: {:#?}", &meta_size);

    let path = {
        let Some(tmp)=&config.cli.file_path else { print_help() };

        PathBuf::from(tmp)
    };
    let archive_type = ArchiveType::new(path.as_path()).unwrap();
    let file_list = archive_type.get_list(path.as_path()).unwrap();
    let mut page_list = file_list.to_page_list(config.base.rename_pad as usize);

    tracing::trace!("file_list: {:#?}", file_list);
    tracing::trace!("page_list: {:#?}", page_list);
    println!("Open: {}", path.as_path().display());

    if let Err(_) = display::cat_img(&config, &mut page_list, meta_size, path, archive_type) {
        tracing::debug!("err");
    }
}
