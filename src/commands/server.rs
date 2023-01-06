use argh::FromArgs;
use axum::async_trait;

use super::Run;

#[derive(FromArgs, Debug)]
#[argh(subcommand, name="server", description="Start the server.")]
pub struct Server {

}

#[async_trait]
impl Run for Server {
    async fn run(&self) -> eyre::Result<()> {
        todo!()
    }
}