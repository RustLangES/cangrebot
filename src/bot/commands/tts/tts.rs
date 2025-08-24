use poise::serenity_prelude::CreateEmbed;
use poise::CreateReply;

use crate::bot;
use crate::bot::commands::tts::TtsStateExt;
use crate::bot::commands::TtsState;

#[poise::command(
    slash_command,
    prefix_command,
    guild_only,
    subcommands("super::begin::begin", "super::end::end")
)]
pub async fn tts(ctx: bot::Context<'_>, #[rest] text: String) -> Result<(), bot::Error> {
    let guild_id = ctx.guild_id().ok_or(".")?;
    let http = ctx.serenity_context().http.clone();

    if ctx.data().tts.check_same_channel(&ctx).await? {
        return Ok(());
    }

    let manager = songbird::get(ctx.serenity_context())
        .await
        .ok_or("No se pudo obtener el manager de voz")?
        .clone();

    let Some(handler_lock) = manager.get(guild_id) else {
        ctx.send(
            CreateReply::default().embed(
                CreateEmbed::new()
                    .title("Error")
                    .description("No estoy en ningún canal de voz. Usa /join primero.")
                    .color(0x00FF_0000),
            ),
        )
        .await?;
        return Ok(());
    };

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
