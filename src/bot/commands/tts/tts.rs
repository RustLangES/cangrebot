use poise::serenity_prelude::CreateEmbed;
use poise::CreateReply;

use crate::bot;
use crate::bot::commands::tts::TtsStateExt;
use crate::bot::commands::TtsState;

async fn tts_play(ctx: bot::Context<'_>, text: String) -> Result<(), bot::Error> {
    let guild_id = ctx.guild_id().ok_or(".")?;
    let http = ctx.serenity_context().http.clone();

    if ctx.data().tts.active_channel().await.is_none() {
        if TtsState::join_vc(ctx.serenity_context(), guild_id, ctx.channel_id()).await? {
            ctx.data().tts.join(ctx.channel_id()).await;
        }
    }

    if ctx.data().tts.check_same_channel(&ctx).await? {
        return Ok(());
    }

    let guild_channel = ctx.guild_channel().await.ok_or("Not a guild channel")?;

    let member = ctx.author_member().await.ok_or("Not a guild member")?;

    let perms = ctx
        .guild()
        .ok_or("Not in a guild")?
        .user_permissions_in(&guild_channel, member.as_ref());

    if !perms.speak() {
        ctx.send(
            CreateReply::default().embed(
                CreateEmbed::new()
                    .title("Error")
                    .description("No tienes permiso de utilizar este comando")
                    .color(0x00FF_0000),
            ),
        )
        .await?;

        return Ok(());
    }

    let manager = songbird::get(ctx.serenity_context())
        .await
        .ok_or("No se pudo obtener el manager de voz")?
        .clone();

    let handler_lock = manager
        .get(guild_id)
        .expect("asserted by check_same_channel");

    let raw_text = format!("{} dice: {}", ctx.author().display_name(), &text);

    TtsState::send_tts(guild_id, http, &handler_lock, ctx.author().id, &raw_text).await?;

    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::new()
                .title("TTS")
                .description(format!("Reproduciendo: {text}"))
                .color(0x0000_FF00),
        ),
    )
    .await?;

    Ok(())
}

// Import all subcommands
use super::*;

#[poise::command(
    slash_command,
    prefix_command,
    guild_only,
    subcommands("begin::begin", "end::end", "leave::leave", "skip::skip", "play")
)]
pub async fn tts(ctx: bot::Context<'_>, #[rest] text: String) -> Result<(), bot::Error> {
    tts_play(ctx, text).await
}

#[poise::command(slash_command, guild_only)]
async fn play(ctx: bot::Context<'_>, #[rest] text: String) -> Result<(), bot::Error> {
    tts_play(ctx, text).await
}
