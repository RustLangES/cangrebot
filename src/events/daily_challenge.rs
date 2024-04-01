use serde::{Deserialize, Serialize};
use serenity::all::ForumTagId;
use serenity::builder::{CreateForumPost, CreateForumTag, CreateMessage};
use serenity::model::prelude::ChannelId;
use serenity::prelude::Context;
use serenity::Result;
use tracing::info;

#[derive(Deserialize, Serialize)]
pub struct DailyChallengeRequest {
    title: String,
    message: String,
    tag_name: String,
}

pub async fn run_daily_challenge(
    ctx: &Context,
    DailyChallengeRequest {
        title,
        message,
        tag_name,
    }: &DailyChallengeRequest,
) -> Result<()> {
    info!("Running create suggestion");
    let msg_channel = ChannelId::new(824695624665923594_u64.into());

    let forum = msg_channel.to_channel(ctx).await?.guild().ok_or(serenity::Error::Other("GuildId not found"))?;
    let Some(tag) = forum.available_tags.iter().find(|t| &t.name == tag_name) else {
        return Err(serenity::Error::Other("Tag not found"));
    };

    let _ = msg_channel
        .create_forum_post(
            &ctx,
            CreateForumPost::new(title, CreateMessage::new().content(message))
                .add_applied_tag(tag.id),
        )
        .await?;

    Ok(())
}
