use crate::bot;
use crate::bot::commands::tts::TtsStateExt;
use poise::serenity_prelude::CreateEmbed;
use poise::CreateReply;

#[poise::command(slash_command, prefix_command, guild_only)]
pub async fn join(ctx: bot::Context<'_>) -> Result<(), bot::Error> {
    let (guild_id, channel_id) = {
        let guild = ctx.guild().ok_or("No se pudo obtener el guild")?;
        let channel_id = guild
            .voice_states
            .get(&ctx.author().id)
            .and_then(|voice_state| voice_state.channel_id);

        (guild.id, channel_id)
    };

    let Some(target_channel) = channel_id else {
        ctx.send(
            CreateReply::default().embed(
                CreateEmbed::new()
                    .title("Error")
                    .description("Debes unirte a un canal de voz primero.")
                    .color(0x00FF_0000),
            ),
        )
        .await?;
        return Ok(());
    };

    let manager = songbird::get(ctx.serenity_context())
        .await
        .ok_or("No se pudo obtener el manager de voz")?
        .clone();

    match manager.join(guild_id, target_channel).await {
        Ok(_) => {
            ctx.data().tts.join(target_channel).await;

            ctx.send(
                CreateReply::default().embed(
                    CreateEmbed::new()
                        .title("Conectado")
                        .description("Me uní al canal de voz correctamente.")
                        .color(0x0000_FF00),
                ),
            )
            .await?;
        }
        Err(_) => {
            ctx.send(
                CreateReply::default().embed(
                    CreateEmbed::new()
                        .title("Error")
                        .description("No se pudo unir al canal de voz. Revisa mis permisos.")
                        .color(0x00FF_0000),
                ),
            )
            .await?;
        }
    }

    Ok(())
}
