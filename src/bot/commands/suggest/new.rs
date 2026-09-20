use poise::{
    CreateReply,
    serenity_prelude::{
        AutoArchiveDuration, ChannelId, ChannelType, CreateEmbed, CreateMessage, CreateThread,
        Mentionable, ReactionType,
    },
};
use tracing::{info, warn};

use crate::bot;

/// Crea una sugerencia
#[poise::command(slash_command, prefix_command)]
pub async fn sugerencia(
    ctx: bot::Context<'_>,
    #[description = "Agrega un Titulo a tu sugerencia"] titulo: String,
    #[description = "Cuentanos acerca de tu sugerencia"] contenido: String,
) -> Result<(), bot::Error> {
    info!("Running create suggestion");
    let data = ctx.data();

    if !data.secrets.features.suggestions {
        ctx.send(
            CreateReply::default()
                .ephemeral(true)
                .content("Suggestions are disabled"),
        )
        .await?;
        return Ok(());
    }

    let Some(channel_suggest) = data.secrets.channel_suggest else {
        warn!("Suggestion channel was not found");
        return Ok(());
    };
    let msg_channel = ChannelId::new(channel_suggest);

    let msg = msg_channel
        .send_message(
            ctx.http(),
            CreateMessage::default().add_embed(
                CreateEmbed::new()
                    .title("Sugerencia")
                    .description(format!(
                        "{} nos sugiere:\n\n{contenido}",
                        ctx.author().mention(),
                    ))
                    .color(0x0042_87F5),
            ),
        )
        .await?;

    // Convert string emoji to ReactionType to allow custom emojis
    let check_reaction = ReactionType::Unicode("✅".to_string());
    let reject_reaction = ReactionType::Unicode("❌".to_string());
    msg.react(&ctx, check_reaction).await.unwrap();
    msg.react(&ctx, reject_reaction).await.unwrap();

    let builder = CreateThread::new(titulo.to_string())
        .kind(ChannelType::PublicThread)
        .auto_archive_duration(AutoArchiveDuration::ThreeDays);
    msg_channel.create_thread(ctx, builder).await.unwrap();

    ctx.send(
        CreateReply::default()
            .ephemeral(true)
            .content("Sugerencia creada ✅"),
    )
    .await?;

    Ok(())
}
