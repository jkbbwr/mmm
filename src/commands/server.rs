use std::net::SocketAddr;

use super::Run;
use argh::FromArgs;
use axum::Server as AxumServer;
use axum::{async_trait, Router};
use config::Config;
use eyre::WrapErr;
use tokio::signal;
use tracing::info;

#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "server", description = "Start the server.")]
pub struct Server {}

#[async_trait]
impl Run for Server {
    async fn run(&self, config: &Config) -> eyre::Result<()> {
        let bind = config
            .get_string("bind")?
            .parse::<SocketAddr>()
            .wrap_err("unable to parse bind address")?;

        let app = Router::new();

        info!("Starting server, listening on {}", bind);

        AxumServer::bind(&bind)
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
