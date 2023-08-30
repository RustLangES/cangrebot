use anyhow::Result;
use serenity::{model::prelude::*, prelude::*};
use std::convert::TryFrom;


pub async fn guild_member_addition(ctx: &Context, guild_id: &GuildId, member: &Member) {
    if let Err(e) = _guild_member_addition(ctx, guild_id, member).await {
        tracing::error!("Failed to handle welcome guild_member_addition: {}", e);
    }
}

#[tracing::instrument(skip(ctx))]
async fn _guild_member_addition(ctx: &Context, guild_id: &GuildId, member: &Member) -> Result<()> {
    let join_msg = "Bienvenid@ <mention> a <server>! Pásala lindo!".to_string();

    let msg_channel = ChannelId(778674893851983932_u64);

    let join_msg_replaced = join_msg
        .replace("<mention>", &member.user.mention().to_string())
        .replace("<username>", &member.user.name)
        .replace(
            "<server>",
            &guild_id.name(ctx).unwrap_or_else(|| "".into()),
        );

    let msg = msg_channel.say(&ctx, join_msg_replaced).await?;

    // Convert string emoji to ReactionType to allow custom emojis
    let reaction = ReactionType::try_from("👋")?;
    msg.react(&ctx, reaction).await?;

    Ok(())
}