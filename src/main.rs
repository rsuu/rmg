use rmg::{App, Config};

fn main() -> eyre::Result<()> {
    init_log();

    let mut config = Config::new()?;
    config.update()?;
    tracing::info!("{:#?}", &config);

    App::start(config)?;

    Ok(())
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
