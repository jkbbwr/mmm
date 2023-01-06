use argh::FromArgs;

pub mod doctor;
pub mod server;
pub mod setup;

use axum::async_trait;

pub use doctor::Doctor;
pub use server::Server;
pub use setup::Setup;

use crate::settings::Settings;

#[async_trait]
pub trait Run {
    async fn run(&self, settings: &Settings) -> eyre::Result<()>;
}

#[derive(FromArgs, Debug)]
#[argh(description = "Mini mTLS Manager.")]
pub struct Cli {
    #[argh(subcommand)]
    pub command: SubCommand,
}

#[derive(FromArgs, Debug)]
#[argh(subcommand)]
pub enum SubCommand {
    Setup(Setup),
    Server(Server),
    Doctor(Doctor),
}

impl SubCommand {
    pub async fn run(&self, settings: &Settings) -> eyre::Result<()> {
        match self {
            SubCommand::Setup(setup) => setup.run(settings),
            SubCommand::Server(server) => server.run(settings),
            SubCommand::Doctor(doctor) => doctor.run(settings),
        }
        .await?;
        Ok(())
    }
}
