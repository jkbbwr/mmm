mod commands;

use commands::*;
use config::{Config, Environment, File};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    color_eyre::install()?;

    let config = Config::builder()
        .add_source(File::with_name("./mmm.toml").required(false))
        .add_source(Environment::with_prefix("mmm"))
        .build()?;

    let cli = argh::from_env::<Cli>();
    cli.command.run(&config).await?;

    Ok(())
}
