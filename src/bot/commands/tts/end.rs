use poise::serenity_prelude::{Color, CreateEmbed};
use poise::CreateReply;

use crate::bot;
use crate::bot::commands::TtsStateExt;

#[poise::command(slash_command, prefix_command, guild_only)]
pub async fn end(ctx: bot::Context<'_>) -> Result<(), bot::Error> {
    if ctx.data().tts.check_same_channel(&ctx).await? {
        return Ok(());
    };

    if !ctx.data().tts.is_active_user(&ctx.author().id).await {
        ctx.send(
            CreateReply::default().reply(true).embed(
                CreateEmbed::default()
                    .description("No has iniciado el modo tts")
                    .color(Color::RED),
            ),
        )
        .await?;

        return Ok(());
    }

    ctx.data().tts.end(&ctx.author().id).await;

    ctx.send(
        CreateReply::default()
            .reply(true)
            .embed(CreateEmbed::default().title("TTS Detenido")),
    )
    .await?;

    Ok(())
}
