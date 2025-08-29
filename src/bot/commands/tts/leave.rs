use crate::bot;
use crate::bot::commands::tts::TtsStateExt;
use poise::serenity_prelude::CreateEmbed;
use poise::CreateReply;

#[poise::command(slash_command, prefix_command, guild_only)]
pub async fn leave(ctx: bot::Context<'_>) -> Result<(), bot::Error> {
    let guild_id = ctx.guild().ok_or("No se pudo obtener el guild")?.id;
    let tts = &ctx.data().tts;

    if tts.check_same_channel(&ctx).await? {
        return Ok(());
    }

    if tts.leave(ctx.serenity_context(), guild_id).await.is_err() {
        ctx.send(
            CreateReply::default().embed(
                CreateEmbed::new()
                    .title("Error")
                    .description("No pude salir del canal de voz. ¿Estoy conectado?")
                    .color(0x00FF_0000),
            ),
        )
        .await?;

        return Ok(());
    }

    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::new()
                .title("Desconectado")
                .description("He salido del canal de voz.")
                .color(0x0000_FF00),
        ),
    )
    .await?;

    Ok(())
}
