use poise::serenity_prelude::{CreateEmbed, CreateEmbedFooter};
use poise::CreateReply;

use crate::bot;
use crate::bot::commands::TtsStateExt;

#[poise::command(slash_command, prefix_command, guild_only)]
pub async fn begin(ctx: bot::Context<'_>) -> Result<(), bot::Error> {
    if ctx.data().tts.check_same_channel(&ctx).await? {
        return Ok(());
    };

    ctx.data().tts.begin(ctx.author().id).await;

    ctx.send(
        CreateReply::default().reply(true).embed(
            CreateEmbed::default()
                .title("TTS iniciado")
                .description("Todos tus mensajes en este canal serán reproducidos por el bot")
                .footer(CreateEmbedFooter::new("Desactivalo con /tts end")),
        ),
    )
    .await?;

    Ok(())
}
