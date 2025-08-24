use poise::serenity_prelude::{Context, CreateEmbed, GuildId, Message, VoiceState};
use poise::CreateReply;

use crate::bot;
use crate::bot::commands::{TtsState, TtsStateExt};

pub async fn message(ctx: &Context, msg: &Message, data: &bot::Data) -> Result<bool, bot::Error> {
    let Some(call_channel) = data.tts.active_channel().await else {
        return Ok(false);
    };

    if msg.channel_id != call_channel {
        return Ok(false);
    }

    if !data.tts.is_active_user(&msg.author.id).await {
        return Ok(false);
    }

    let guild_id = msg.guild_id.ok_or("No se pudo obtener el guild")?;

    let manager = songbird::get(ctx)
        .await
        .ok_or("No se pudo obtener el manager de voz")?
        .clone();

    let Some(handler_lock) = manager.get(guild_id) else {
        msg.channel_id
            .send_message(
                ctx,
                CreateReply::default()
                    .embed(
                        CreateEmbed::new()
                            .title("Error")
                            .description("No estoy en ningún canal de voz. Usa /join primero.")
                            .color(0x00FF_0000),
                    )
                    .to_prefix(msg.into()),
            )
            .await?;

        return Ok(true);
    };

    let raw_text = format!("{} dice: {}", msg.author.display_name(), &msg.content);

    TtsState::send_tts(
        guild_id,
        ctx.http.clone(),
        &handler_lock,
        msg.author.id,
        &raw_text,
    )
    .await?;

    Ok(false)
}
