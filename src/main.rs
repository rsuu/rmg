use rmg::{
    config::rsconf::{print_help, Config},
    img::size::{MetaSize, TMetaSize},
    render::display,
    utils::file,
};

fn main() {
    init_log();

    let mut config = Config::new();
    config.try_from_config_file();
    config.try_from_cli();

    let meta_size = MetaSize::new(0, 0, config.base.size.width, config.base.size.height, 0, 0);

    tracing::debug!("{:#?}", config);
    tracing::debug!("meta_size: {:#?}", &meta_size);

    let Some(path) = &config.cli.file_path else { print_help() };
    let archive_type = file::get_path_type(path.as_str());
    let file_list = file::get_file_list(path.as_str()).unwrap();
    let mut page_list = file::get_page_list(&file_list, config.base.rename_pad as usize);

    tracing::debug!("file_list: {:#?}", file_list);
    tracing::debug!("page_list: {:#?}", page_list);
    println!("Open: {}", path.as_str());

    if let Err(e) = display::cat_img(
        &config,
        &mut page_list,
        meta_size,
        path.as_str(),
        archive_type,
    ) {
        println!("{}", e);
    }
}

fn init_log() {
    use tracing_subscriber::{fmt, prelude::*, EnvFilter};

    // e.g. RUST_LOG="rmg::render::scroll=debug"
    let env_filter = EnvFilter::builder().with_regex(true).from_env_lossy();
    let log_fmt = fmt::layer().without_time().with_thread_names(true);

    tracing_subscriber::registry()
        .with(log_fmt)
        .with(env_filter)
        .init();

    tracing::info!("init_log()");
}
