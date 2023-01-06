use crate::settings::Settings;

use super::Run;
use argh::FromArgs;

use axum::Server as AxumServer;
use axum::{async_trait, Router};

use tokio::signal;
use tracing::info;

#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "server", description = "Start the server.")]
pub struct Server {}

#[async_trait]
impl Run for Server {
    async fn run(&self, settings: &Settings) -> eyre::Result<()> {
        tracing_subscriber::fmt::init();

        let host = settings.server.host.parse()?;
        let app = Router::new();

        info!("Starting server, listening on {}", host);
        AxumServer::bind(&host)
            .serve(app.into_make_service())
            .with_graceful_shutdown(async {
                let ctrl_c = async {
                    signal::ctrl_c()
                        .await
                        .expect("failed to install ctrl+c handler.");
                };

                tokio::select! {
                    _ = ctrl_c => {},
                }

                info!("Gracefully shutting down.");
            })
            .await?;

        Ok(())
    }
}
