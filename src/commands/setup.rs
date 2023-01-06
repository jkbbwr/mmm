use std::io::prelude::*;
use std::{fs::File, path::Path};

use argh::FromArgs;
use axum::async_trait;
use eyre::bail;

use crate::settings::Settings;

use super::Run;

#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "setup", description = "Setup mmm.")]
pub struct Setup {
    #[argh(switch, description = "overwrite the existing config file.")]
    overwrite: bool,
}

#[async_trait]
impl Run for Setup {
    async fn run(&self, _settings: &Settings) -> eyre::Result<()> {
        let settings = Settings::default();
        if Path::new("./mmm.toml").exists() && !self.overwrite {
            bail!("./mmm.toml exists and --overwrite wasn't given.");
        }

        bunt::println!("{$blue}Writing a default config file to ./mmm.toml{/$}");
        let mut file = File::create("./mmm.toml")?;
        write!(file, "{}", toml::to_string(&settings)?)?;

        // Generate a default config file.
        Ok(())
    }
}
