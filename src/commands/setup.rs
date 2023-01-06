use argh::FromArgs;
use axum::async_trait;
use config::Config;

use super::Run;

#[derive(FromArgs, Debug)]
#[argh(subcommand, name="setup", description="Setup mmm.")]
pub struct Setup {

}

#[async_trait]
impl Run for Setup {
    async fn run(&self, config: &Config) -> eyre::Result<()> {
        todo!()
    }
}