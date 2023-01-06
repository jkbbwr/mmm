mod commands;
mod settings;

use commands::*;
use eyre::Result;
use figment::{
    providers::{Env, Format, Serialized, Toml},
    Figment,
};
use settings::Settings;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let settings = Figment::from(Serialized::defaults(Settings::default()))
        .merge(Toml::file("mmm.toml"))
        .merge(Env::prefixed("mmm_"))
        .extract()?;

    let cli = argh::from_env::<Cli>();
    cli.command.run(&settings).await?;

    Ok(())
}
