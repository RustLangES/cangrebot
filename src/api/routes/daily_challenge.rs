use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::api::RouteState;
use crate::serenity::builder::{CreateAllowedMentions, CreateForumPost, CreateMessage};
use crate::serenity::model::prelude::ChannelId;

const PARTICIPANT_ROLE: u64 = 1224238464958992495;

#[derive(Deserialize, Serialize)]
pub struct DailyChallengeRequest {
    title: String,
    message: String,
    tag_name: String,
}

pub async fn daily_challenge(
    State((secrets, ctx)): State<RouteState>,
    Json(DailyChallengeRequest {
        title,
        message,
        tag_name,
    }): Json<DailyChallengeRequest>,
) -> impl IntoResponse {
    info!("Running daily challenge events");
    let msg_channel = ChannelId::new(secrets.channel_daily);

    let Ok(forum) = msg_channel.to_channel(&ctx).await else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Cannot convert to channel",
        );
    };

    let Some(forum) = forum.guild() else {
        return (StatusCode::NOT_FOUND, "GuildId not found");
    };

    let Some(tag) = forum.available_tags.iter().find(|t| t.name == tag_name) else {
        return (StatusCode::NOT_FOUND, "Tag not found");
    };

    match msg_channel
        .create_forum_post(
            &ctx,
            CreateForumPost::new(
                title,
                CreateMessage::new()
                    .content(message)
                    .allowed_mentions(CreateAllowedMentions::new().roles([PARTICIPANT_ROLE])),
            )
            .add_applied_tag(tag.id),
        )
        .await
    {
        Ok(_) => (StatusCode::OK, "Ok"),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Cannot Create Forum Post",
        ),
    }
}
