use poise::serenity_prelude::{Color, CreateEmbed, CreateEmbedFooter, UserId};
use poise::CreateReply;

use crate::bot;
use crate::bot::commands::{TtsState, TtsStateExt};

#[poise::command(slash_command, prefix_command, guild_only)]
pub async fn begin(ctx: bot::Context<'_>, user: Option<UserId>) -> Result<(), bot::Error> {
    let guild_id = ctx.guild_id().expect("This command is for guild only");

    if ctx.data().tts.active_channel().await.is_none()
        && TtsState::join_vc(ctx.serenity_context(), guild_id, ctx.channel_id()).await?
    {
        ctx.data().tts.join(ctx.channel_id()).await;
    }

    if !ctx.data().tts.check_same_channel(&ctx).await? {
        return Ok(());
    };

    let user_id = if let Some(user) = user {
        let guild_channel = ctx.guild_channel().await.ok_or("Not a guild channel")?;

        let member = ctx.author_member().await.ok_or("Not a guild member")?;

        let perms = ctx
            .guild()
            .ok_or("Not in a guild")?
            .user_permissions_in(&guild_channel, member.as_ref());

        if !perms.manage_channels() {
            ctx.send(
                CreateReply::default().reply(true).embed(
                    CreateEmbed::default()
                        .color(Color::RED)
                        .title("TTS denegado")
                        .description(
                            "La activación externa de TTS mode es exclusivo para moderadores",
                        ),
                ),
            )
            .await?;
            return Ok(());
        }

        user
    } else {
        ctx.author().id
    };

    ctx.data().tts.begin(user_id).await;

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
