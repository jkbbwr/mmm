use argh::FromArgs;
use axum::async_trait;
use dotenvy::dotenv;
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
    async fn run(&self) -> eyre::Result<()> {
        match dotenv() {
            Ok(path) => {
                info!("Found a .env file here: {}", path.to_str().unwrap());
            },
            Err(error) => warn!("Failed to find a .env because: {}", error),
        }

        Ok(())
    }
}
