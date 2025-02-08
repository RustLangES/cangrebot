use poise::serenity_prelude::{
    AutoArchiveDuration, ChannelId, ChannelType, CreateThread, Mentionable, ReactionType,
};
use tracing::info;

use crate::bot;

/// Crea una sugerencia
#[poise::command(slash_command, prefix_command)]
pub async fn nueva(
    ctx: bot::Context<'_>,
    #[description = "Agrega un Titulo a tu sugerencia"] titulo: String,
    #[description = "Cuentanos acerca de tu sugerencia"] contenido: String,
) -> Result<(), bot::Error> {
    info!("Running create suggestion");
    let data = ctx.data();

    let msg_channel = ChannelId::new(data.secrets.channel_suggest);

    let msg = format!("{} nos sugiere\n\n{contenido}", ctx.author().mention(),);
    let msg = msg_channel.say(&ctx, msg).await.unwrap();

    // Convert string emoji to ReactionType to allow custom emojis
    let check_reaction = ReactionType::Unicode("✅".to_string());
    let reject_reaction = ReactionType::Unicode("❌".to_string());
    msg.react(&ctx, check_reaction).await.unwrap();
    msg.react(&ctx, reject_reaction).await.unwrap();

    let builder = CreateThread::new(titulo.to_string())
        .kind(ChannelType::PublicThread)
        .auto_archive_duration(AutoArchiveDuration::ThreeDays);
    msg_channel.create_thread(ctx, builder).await.unwrap();

    ctx.say("Sugerencia Creada").await?;

    Ok(())
}
