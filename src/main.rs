use axum::extract::{Request, State};
use axum::http::{HeaderMap, StatusCode};
use axum::middleware::Next;
use axum::response::Response;
use axum::{middleware, Router, ServiceExt};
use events::daily_challenge::run_daily_challenge;
use events::send_message::send_message;
use serenity::http::Http;
use serenity::Client;
use shuttle_runtime::SecretStore;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

pub mod config;
pub mod events;
pub mod general_commands;
pub mod music;
pub mod slash_commands;
pub mod utils;
use config::setup::setup;

#[derive(Clone, Debug)]
struct SecretsState {
    bot_apikey: String,
}

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

async fn auth_token(
    State(secrets): State<SecretsState>,
    headers: HeaderMap,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let header_key = headers.get("Authorization");
    if header_key
        .as_ref()
        .is_some_and(|k| k.to_str().unwrap() == &secrets.bot_apikey)
    {
        return Ok(next.run(req).await);
    }

    tracing::error!(
        "UNAUTHORIZED: {header_key:?} - Local: {}",
        secrets.bot_apikey
    );

    Err(axum::http::StatusCode::UNAUTHORIZED)
}

fn build_router(secrets: SecretStore, ctx: Arc<Http>) -> Router {
    Router::new()
        .route("/daily_challenge", axum::routing::post(run_daily_challenge))
        .route("/send_message", axum::routing::post(send_message))
        .layer(middleware::from_fn_with_state(
            SecretsState {
                bot_apikey: secrets
                    .get("BOT_APIKEY")
                    .expect("Cannot get bot apikey to authorize"),
            },
            auth_token,
        ))
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

    let discord_bot = setup(secret_store.clone(), public_folder).await?;
    let router = build_router(secret_store, discord_bot.http.clone());

    Ok(CustomService {
        discord_bot,
        router,
    })
}
