mod auth;
pub mod routes;

use std::sync::Arc;

use axum::Router;
use poise::serenity_prelude::Http;

use crate::CangrebotSecrets;

pub(super) type RouteState = (CangrebotSecrets, Arc<Http>);

pub fn build_router(secrets: &CangrebotSecrets, ctx: Arc<Http>) -> Router {
    Router::new()
        .route("/healthcheck", axum::routing::get(routes::healthcheck))
        .nest(
            "/",
            Router::new()
                .route(
                    "/daily_challenge",
                    axum::routing::post(routes::daily_challenge),
                )
                .route("/send_message", axum::routing::post(routes::send_message))
                .layer(axum::middleware::from_fn_with_state(
                    secrets.clone(),
                    auth::middleware,
                ))
                .with_state((secrets.clone(), ctx)),
        )
}
