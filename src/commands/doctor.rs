use argh::FromArgs;
use axum::async_trait;

use crate::settings::Settings;

use super::Run;

#[derive(FromArgs, Debug)]
#[argh(
    subcommand,
    name = "doctor",
    description = "Check everything is setup correctly."
)]
pub struct Doctor {}

#[async_trait]
impl Run for Doctor {
    async fn run(&self, _settings: &Settings) -> eyre::Result<()> {
        Ok(())
    }
}
