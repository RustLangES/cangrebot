use poise::serenity_prelude::{Color, CreateEmbed, UserId};
use poise::CreateReply;

use crate::bot;
use crate::bot::commands::TtsStateExt;

#[poise::command(slash_command, prefix_command, guild_only)]
pub async fn end(ctx: bot::Context<'_>, user: Option<UserId>) -> Result<(), bot::Error> {
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
                            "La desactivación externa de TTS mode es exclusivo para moderadores",
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

    if !ctx.data().tts.is_active_user(&user_id).await {
        ctx.send(
            CreateReply::default().reply(true).embed(
                CreateEmbed::default()
                    .description("No has iniciado el modo tts")
                    .color(Color::RED),
            ),
        )
        .await?;

        return Ok(());
    }

    ctx.data().tts.end(&user_id).await;

    ctx.send(
        CreateReply::default()
            .reply(true)
            .embed(CreateEmbed::default().title("TTS Detenido")),
    )
    .await?;

    Ok(())
}
