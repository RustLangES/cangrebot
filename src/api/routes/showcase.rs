use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::api::RouteState;
use crate::serenity::builder::{CreateForumPost, CreateMessage};
use crate::serenity::model::prelude::ChannelId;

#[derive(Deserialize, Serialize)]
pub struct ShowcaseRequest {
    name: String,
    desc: String,
    #[serde(default)]
    tags: Vec<String>,
    url: String,
}

pub async fn showcase(
    State((secrets, ctx)): State<RouteState>,
    Json(ShowcaseRequest {
        name,
        desc,
        tags,
        url,
    }): Json<ShowcaseRequest>,
) -> impl IntoResponse {
    info!("Running showcase creation from API");

    let msg_channel = ChannelId::new(secrets.channel_showcase);

    let Ok(channel) = msg_channel.to_channel(&ctx).await else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Cannot convert to channel".to_string(),
        );
    };

    let Some(forum) = channel.guild() else {
        return (StatusCode::NOT_FOUND, "Guild channel not found".to_string());
    };

    let mut forum_post = CreateForumPost::new(
        name,
        CreateMessage::new().content(format!("{desc}\n\n{url}")),
    );

    for tag_name in tags {
        let Some(tag) = forum.available_tags.iter().find(|tag| tag.name == tag_name) else {
            return (StatusCode::NOT_FOUND, format!("Tag '{tag_name}' not found"));
        };

        forum_post = forum_post.add_applied_tag(tag.id);
    }

    match msg_channel.create_forum_post(&ctx, forum_post).await {
        Ok(_) => (StatusCode::OK, "Ok".to_string()),
        Err(err) => {
            tracing::error!("Cannot create showcase forum post: {err:?}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Cannot create showcase forum post".to_string(),
            )
        }
    }
}
