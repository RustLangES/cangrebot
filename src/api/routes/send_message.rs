use crate::api::RouteState;
use crate::serenity::all::MESSAGE_CODE_LIMIT;
use crate::serenity::builder::{CreateAllowedMentions, CreateMessage};
use crate::serenity::model::prelude::ChannelId;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SendMessagePayload {
    message: String,
    channel_id: u64,
    roles: Vec<u64>,
}

fn split_into_chunks(s: &str, chunk_size: usize) -> Vec<&str> {
    s.as_bytes()
        .chunks(chunk_size)
        .map(|chunk| std::str::from_utf8(chunk).unwrap())
        .collect()
}

pub async fn send_message(
    State((_, ctx)): State<RouteState>,
    Json(SendMessagePayload {
        message,
        channel_id,
        roles,
    }): Json<SendMessagePayload>,
) -> impl IntoResponse {
    info!("Running Send Message from API");
    let msg_channel = ChannelId::new(channel_id);

    let messages = split_into_chunks(&message, MESSAGE_CODE_LIMIT);

    for message in &messages {
        let message = CreateMessage::new()
            .content(*message)
            .allowed_mentions(CreateAllowedMentions::new().roles(roles.clone()));

        if let Err(err) = msg_channel.send_message(&ctx, message).await {
            tracing::error!("Cannot send message: {err:?}");
            return (StatusCode::INTERNAL_SERVER_ERROR, "Cannot send message");
        }
    }

    (StatusCode::OK, "Ok")
}
