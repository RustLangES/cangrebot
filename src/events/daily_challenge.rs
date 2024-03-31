use serde::{Deserialize, Serialize};
use serenity::all::ForumTagId;
use serenity::builder::{CreateForumPost, CreateMessage};
use serenity::model::prelude::ChannelId;
use serenity::prelude::Context;
use serenity::Result;
use tracing::info;

#[derive(Deserialize, Serialize)]
pub struct DailyChallengeRequest {
    title: String,
    message: String,
    tag: u64,
}

pub async fn run_daily_challenge(
    ctx: &Context,
    DailyChallengeRequest {
        title,
        message,
        tag,
    }: &DailyChallengeRequest,
) -> Result<()> {
    info!("Running create suggestion");
    let msg_channel = ChannelId::new(824695624665923594_u64.into());

    let _ = msg_channel
        .create_forum_post(
            &ctx,
            CreateForumPost::new(title, CreateMessage::new().content(message))
                .add_applied_tag(ForumTagId::new(*tag)),
        )
        .await?;

    Ok(())
}
