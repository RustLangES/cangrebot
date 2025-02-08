pub use poise::serenity_prelude as serenity;
pub use secrets::CangrebotSecrets;

use std::net::SocketAddr;

use axum::{Router, ServiceExt};
use poise::serenity_prelude::Client;

mod api;
mod bot;
mod secrets;

pub struct CustomService {
    discord_bot: Client,
    router: Router,
}

impl CustomService {
    async fn bind(mut self, addr: SocketAddr) -> Result<(), std::io::Error> {
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    color_eyre::install().expect("Failed to install color_eyre");

    let secrets = CangrebotSecrets::from(std::env::var);

    let discord_bot = bot::setup(&secrets).await?;
    let router = api::build_router(&secrets, discord_bot.http.clone());

    let mut custom_service = CustomService {
        discord_bot,
        router,
    };
    custom_service.discord_bot.start_autosharded().await?;
    Ok(())
}
