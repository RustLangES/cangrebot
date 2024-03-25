use anyhow::Result;
use serenity::{model::prelude::*, prelude::*};
use std::convert::TryFrom;
use plantita_welcomes::create_welcome::combine_images;
use serenity::all::{CreateAttachment, CreateMessage};

pub async fn guild_member_addition(ctx: &Context, guild_id: &GuildId, member: &Member) {
    if let Err(e) = _guild_member_addition(ctx, guild_id, member).await {
        tracing::error!("Failed to handle welcome guild_member_addition: {}", e);
    }
}

#[tracing::instrument(skip(ctx))]
async fn _guild_member_addition(ctx: &Context, guild_id: &GuildId, member: &Member) -> Result<()> {
    let join_msg = "Bienvenid@ <mention> a <server>! Pásala lindo!".to_string();

    let msg_channel = ChannelId::new(778674893851983932_u64);

    let join_msg_replaced = join_msg
        .replace("<mention>", &member.user.mention().to_string())
        .replace("<username>", &member.user.name)
        .replace(
            "<server>",
            &guild_id.name(ctx).unwrap_or_else(|| "".into()),
        );

    // Download the user's avatar and create a welcome image
    let avatar_url = member.user.avatar_url().unwrap_or_else(|| member.user.default_avatar_url());
    let response = reqwest::get(avatar_url).await?;
    let bytes = response.bytes().await?;

    let img = image::load_from_memory(&bytes)?;
    img.resize(256, 256, image::imageops::Lanczos3);
    let mut background = image::open("./static/background.png")?;

    let output_path = format!("/tmp/{}_welcome.png", member.user.name);
    combine_images(&mut background, &img, 74, 74, 372)?;
    background.save(output_path.as_str())?;
    let attachment = CreateAttachment::path(output_path.as_str()).await?;

    let msg = msg_channel.send_files(&ctx, vec![attachment], CreateMessage::new().content(&join_msg_replaced)).await?;

    // Remove the file after sending the message
    std::fs::remove_file(&output_path)?;

    // Convert string emoji to ReactionType to allow custom emojis
    let reaction = ReactionType::try_from("👋")?;
    msg.react(&ctx, reaction).await?;

    Ok(())
}