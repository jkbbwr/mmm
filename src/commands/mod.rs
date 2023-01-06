use argh::FromArgs;

pub mod doctor;
pub mod server;
pub mod setup;

use axum::async_trait;
use config::Config;
pub use doctor::Doctor;
pub use setup::Setup;
pub use server::Server;

#[async_trait]
pub trait Run {
    async fn run(&self, config: &Config) -> eyre::Result<()>;
}

#[derive(FromArgs, Debug)]
#[argh(description="Mini mTLS Manager.")]
pub struct Cli {
    #[argh(subcommand)]
    pub command: SubCommand
}

#[derive(FromArgs, Debug)]
#[argh(subcommand)]
pub enum SubCommand {
    Setup(Setup),
    Server(Server),
    Doctor(Doctor),
}

impl SubCommand {
    pub async fn run(&self, config: &Config) -> eyre::Result<()> {
        match self {
            SubCommand::Setup(setup) => setup.run(config),
            SubCommand::Server(server) => server.run(config),
            SubCommand::Doctor(doctor) => doctor.run(config)
        }.await?;
        Ok(())
    }
}