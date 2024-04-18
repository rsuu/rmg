use rmg::{window, Config};

fn main() -> eyre::Result<()> {
    let mut config = Config::new()?;
    config.update()?;

    window::desktop::main(config)?;

    Ok(())
}
