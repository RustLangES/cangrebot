use crate::bot;
use crate::bot::commands::tts::{TtsStateExt, TtsTrackData};
use poise::serenity_prelude::{ChannelId, CreateEmbed};
use poise::CreateReply;
use std::sync::Arc;

#[poise::command(slash_command, prefix_command, guild_only)]
pub async fn stop(ctx: bot::Context<'_>) -> Result<(), bot::Error> {
    let guild_id = ctx.guild().ok_or("No se pudo obtener el guild")?.id;

    if ctx.data().tts.check_same_channel(&ctx).await? {
        return Ok(());
    }

    let manager = songbird::get(ctx.serenity_context())
        .await
        .ok_or("No se pudo obtener el manager de voz")?;

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

    let handler = handler_lock.lock().await;
    let track_data: Arc<TtsTrackData> = handler
        .queue()
        .current()
        .ok_or("Failed to get track data")?
        .data();

    let member = ctx.author_member().await.ok_or("Failed to get member")?;

    let guild_channel = ChannelId::new(
        handler
            .current_channel()
            .ok_or("Failed to get channel by ID")?
            .0
            .into(),
    )
    .to_channel(&ctx.http())
    .await?
    .guild()
    .ok_or("Not a guild channel")?;

    let perms = ctx
        .guild()
        .ok_or("Not in a guild")?
        .user_permissions_in(&guild_channel, member.as_ref());

    if track_data.author_id != ctx.author().id && !perms.priority_speaker() {
        ctx.send(
            CreateReply::default().embed(
                CreateEmbed::new()
                    .title("Error")
                    .description("Solo el autor original o un usuario con el permiso de prioridad puede detener la reproducción.")
                    .color(0x00FF_0000),
            ),
        )
        .await?;
        return Ok(());
    }

    ctx.send(CreateReply::default().content("⏭ ")).await?;

    let c_uuid = track_data.uuid;
    handler.queue().modify_queue(|q| {
        let ids: Vec<usize> = q
            .iter()
            .enumerate()
            .filter_map(|(i, q)| {
                let data: Arc<TtsTrackData> = q.data();
                (data.uuid == c_uuid).then_some(i)
            })
            .rev()
            .collect();

        for id in ids {
            if let Some(track) = q.remove(id) {
                // Errors when removing tracks don't really make
                // a difference: an error just implies it's already gone.
                drop(track.stop())
            }
        }
    });

    // Errors when no more tracks
    drop(handler.queue().resume());

    Ok(())
}
