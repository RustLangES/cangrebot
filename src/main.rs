use axum::extract::Request;
use axum::http::{HeaderMap, StatusCode};
use axum::middleware::Next;
use axum::response::Response;
use axum::{middleware, Router, ServiceExt};
use events::daily_challenge::run_daily_challenge;
use events::send_message::send_message;
use serenity::http::Http;
use serenity::prelude::Context;
use serenity::Client;
use shuttle_runtime::SecretStore;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

pub mod config;
pub mod events;
pub mod general_commands;
pub mod slash_commands;
use config::setup::setup;
use once_cell::sync::Lazy;

#[macro_use]
extern crate litcrypt2;

use_litcrypt!();

static BOT_API_KEY: Lazy<String> = Lazy::new(|| lc!(env!("BOT_APIKEY")));

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

async fn auth_token(headers: HeaderMap, req: Request, next: Next) -> Result<Response, StatusCode> {
    if headers
        .get("Authorization")
        .as_ref()
        .is_some_and(|k| k.to_str().unwrap() == BOT_API_KEY.as_str())
    {
        return Ok(next.run(req).await);
    }
    Err(axum::http::StatusCode::UNAUTHORIZED)
}

fn build_router(ctx: Arc<Http>) -> Router {
    Router::new()
        .route("/daily_challenge", axum::routing::post(run_daily_challenge))
        .route("/send_message", axum::routing::post(send_message))
        .layer(middleware::from_fn(auth_token))
        .with_state(ctx)
}

#[shuttle_runtime::main]
async fn init(
    #[shuttle_runtime::Secrets] secret_store: SecretStore,
) -> Result<CustomService, shuttle_runtime::Error> {
    let Ok(_) = color_eyre::install() else {
        panic!("Failed to install color_eyre");
    };
    let public_folder = PathBuf::from("static");

    let discord_bot = setup(secret_store, public_folder).await?;
    let router = build_router(discord_bot.http.clone());

    Ok(CustomService {
        discord_bot,
        router,
    })
}
