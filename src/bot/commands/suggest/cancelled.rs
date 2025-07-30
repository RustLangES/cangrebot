use poise::serenity_prelude::ChannelType;
use poise::serenity_prelude::CreateEmbed;
use poise::serenity_prelude::CreateMessage;
use poise::serenity_prelude::EditThread;
use poise::CreateReply;

use crate::bot;

/// Marca una sugerencia como cancelada o que no se realizara
#[poise::command(
    slash_command,
    hide_in_help = true,
    default_member_permissions = "MANAGE_CHANNELS"
)]
pub async fn sugerencia_cancelada(ctx: bot::Context<'_>, reason: String) -> Result<(), bot::Error> {
    ctx.send(
        CreateReply::default()
            .ephemeral(true)
            .content("La sugerencia se marcara como cancelada"),
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
                    .description(format!(
                        "Esta sugerencia fue marcada como cancelada.\nRazon: {reason}"
                    ))
                    .color(0x00FF_0000),
            ),
        )
        .await?;

    channel
        .edit_thread(&ctx.http(), EditThread::default().locked(true))
        .await?;

    Ok(())
}
