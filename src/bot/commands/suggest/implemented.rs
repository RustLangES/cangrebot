use poise::{
    serenity_prelude::{ChannelType, CreateEmbed, CreateMessage, EditThread},
    CreateReply,
};

use crate::bot;

/// Marca una sugerencia como implementada
#[poise::command(
    slash_command,
    hide_in_help = true,
    default_member_permissions = "MANAGE_CHANNELS"
)]
pub async fn sugerencia_implementada(ctx: bot::Context<'_>) -> Result<(), bot::Error> {
    ctx.send(
        CreateReply::default()
            .ephemeral(true)
            .content("La sugerencia se marcara como implementada"),
    )
    .await?;

    let channel = ctx.channel_id().to_channel(&ctx.http()).await?;
    let mut channel = channel.guild().ok_or("Couldn't get GuildChannel")?;

    if channel.kind != ChannelType::PublicThread {
        ctx.send(
            CreateReply::default()
                .ephemeral(true)
                .content("Este comando debe ser utilizado en un hilo publico"),
        )
        .await?;
        return Ok(());
    }

    channel
        .send_message(
            ctx.http(),
            CreateMessage::default().add_embed(
                CreateEmbed::new()
                    .title("Gracias por tus comentarios!")
                    .description("Esta sugerencia fue marcada como implementada! Muchas gracias!")
                    .color(0x0000_FF00),
            ),
        )
        .await?;

    channel
        .edit_thread(&ctx.http(), EditThread::default().locked(true))
        .await?;

    Ok(())
}
