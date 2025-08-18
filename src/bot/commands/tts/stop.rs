use crate::bot;
use poise::serenity_prelude::{ChannelId, CreateEmbed, UserId};
use poise::CreateReply;
use std::sync::Arc;
use tracing::info;

#[poise::command(slash_command, prefix_command, guild_only)]
pub async fn stop(ctx: bot::Context<'_>) -> Result<(), bot::Error> {
    info!("1");
    let guild_id = ctx.guild().ok_or("No se pudo obtener el guild")?.id;

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
    let track_data: Arc<UserId> = handler
        .queue()
        .current()
        .ok_or("Failed to get track data")?
        .data();

    let author_id = &*track_data;
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

    if author_id != &ctx.author().id && !perms.priority_speaker() {
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

    handler.queue().skip()?;
    Ok(())
}
