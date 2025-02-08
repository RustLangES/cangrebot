pub use poise::serenity_prelude as serenity;
pub use secrets::CangrebotSecrets;

use std::net::SocketAddr;

use axum::{Router, ServiceExt};
use poise::serenity_prelude::Client;
use shuttle_runtime::SecretStore;

mod api;
mod bot;
mod secrets;

pub struct CustomService {
    discord_bot: Client,
    router: Router,
}

#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for CustomService {
    async fn bind(mut self, addr: SocketAddr) -> Result<(), shuttle_runtime::Error> {
        let router = self.router.into_service();

        let listener = tokio::net::TcpListener::bind(&addr).await?;
        let serve_router = async move {
            axum::serve(listener, router.into_make_service())
                .await
                .unwrap();
        };

        tokio::select! {
            _ = self.discord_bot.start_autosharded() => {},
            _ = serve_router => {},
        };

        Ok(())
    }
}

#[shuttle_runtime::main]
async fn init(
    #[shuttle_runtime::Secrets] secret_store: SecretStore,
) -> Result<CustomService, shuttle_runtime::Error> {
    let Ok(_) = color_eyre::install() else {
        panic!("Failed to install color_eyre");
    };

    let secrets = CangrebotSecrets::from(secret_store);

    let discord_bot = bot::setup(&secrets).await?;
    let router = api::build_router(&secrets, discord_bot.http.clone());

    Ok(CustomService {
        discord_bot,
        router,
    })
}
