use anyhow::Result;
use serenity::{model::prelude::*, prelude::*};
use std::convert::TryFrom;
use plantita_welcomes::create_welcome::combine_images;

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

    let avatar_url = member.user.avatar_url().unwrap_or_else(|| member.user.default_avatar_url());
    let response = reqwest::get(avatar_url).await?;
    let bytes = response.bytes().await?;

    let img = image::load_from_memory(&bytes)?;
    img.resize(256, 256, image::imageops::Lanczos3);
    img.save_with_format(format!("/tmp/{}_avatar.png", member.user.name), image::ImageFormat::Png)?;

    let avatar_path = format!("/tmp/{}_avatar.png", member.user.name);
    let output_path = format!("/tmp/{}_welcome.png", member.user.name);
    combine_images("./static/background.png", &avatar_path, 74, 74, 372, &output_path)?;

    let msg = msg_channel.send_files(&ctx, vec![output_path.as_str()], |m| {
        m.content(&join_msg_replaced)
    }).await?;

    std::fs::remove_file(avatar_path)?;
    std::fs::remove_file(output_path)?;

    // Convert string emoji to ReactionType to allow custom emojis
    let reaction = ReactionType::try_from("👋")?;
    msg.react(&ctx, reaction).await?;

    Ok(())
}