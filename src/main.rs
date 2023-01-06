mod commands;

use commands::*;

use dotenvy::dotenv;
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let cli = argh::from_env::<Cli>();
    cli.command.run().await?;

    Ok(())
}