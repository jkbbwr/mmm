use argh::FromArgs;
use axum::async_trait;
use config::Config;
use tracing::info;
use tracing::warn;

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
    async fn run(&self, config: &Config) -> eyre::Result<()> {

        Ok(())
    }
}
