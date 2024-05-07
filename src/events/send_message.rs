use std::sync::Arc;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};
use serenity::builder::{CreateAllowedMentions, CreateMessage};
use serenity::http::Http;
use serenity::model::prelude::ChannelId;
use tracing::info;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SendMessagePayload {
    message: String,
    channel_id: u64,
    roles: Vec<u64>,
}

pub async fn send_message(
    State(ctx): State<Arc<Http>>,
    Json(SendMessagePayload {
        message,
        channel_id,
        roles,
    }): Json<SendMessagePayload>,
) -> impl IntoResponse {
    info!("Running create suggestion");
    let msg_channel = ChannelId::new(channel_id);
    let message = CreateMessage::new()
        .content(message)
        .allowed_mentions(CreateAllowedMentions::new().roles(roles));

    if msg_channel.send_message(&ctx, message).await.is_ok() {
        return (StatusCode::OK, "Ok");
    }

    (StatusCode::INTERNAL_SERVER_ERROR, "Cannot send message")
}
