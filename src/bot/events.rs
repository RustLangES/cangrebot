mod compiler;
mod godbolt;
mod join;
mod new_members_mention;
mod read_github_links;
pub mod temporal_voice;
mod tts;

use poise::serenity_prelude::{ChannelId, Context, FullEvent, GuildId};
use poise::FrameworkContext;
use temporal_voice::{temporal_voice_join, temporal_voice_quit};
use tracing::info;

use crate::bot::{self, Data};
use crate::CangrebotSecrets;

pub async fn handle(
    ctx: &Context,
    event: &FullEvent,
    fm: FrameworkContext<'_, Data, bot::Error>,
    data: &Data,
    secrets: &CangrebotSecrets,
) -> Result<(), bot::Error> {
    match event {
        FullEvent::Ready { data_about_bot, .. } => {
            info!("Logged in as {}", data_about_bot.user.name);
            Ok(())
        }
        FullEvent::Message { new_message } => {
            _ = compiler::message(ctx, new_message, &secrets.discord_prefix).await?
                || new_members_mention::message(ctx, new_message).await?
                || read_github_links::message(ctx, new_message).await
                || temporal_voice::message(ctx, new_message, ChannelId::new(secrets.temporal_logs))
                    .await?
                || tts::message(ctx, new_message, data).await?;

            Ok(())
        }
        FullEvent::MessageUpdate { event, .. } => {
            let msg = ctx.http.get_message(event.channel_id, event.id).await?;

            temporal_voice::message(ctx, &msg, ChannelId::new(secrets.temporal_logs)).await?;

            Ok(())
        }

        FullEvent::GuildMemberAddition { new_member } => {
            join::guild_member_addition_event(
                ctx,
                &GuildId::new(data.secrets.guild_id),
                new_member,
            )
            .await;

            Ok(())
        }
        FullEvent::InteractionCreate { interaction } => {
            // for buttons
            if let Some(interaction) = interaction.as_message_component() {
                _ = read_github_links::handle_delete_embed(ctx, interaction).await
                    || read_github_links::handle_save_embed(ctx, interaction).await;
            }

            Ok(())
        }
        FullEvent::VoiceStateUpdate { old, new } => {
            let Some(guild_id) = &new.guild_id else {
                return Ok(());
            };
            let Some(member) = &new.member else {
                return Ok(());
            };

            if let Some(channel_id) = new.channel_id {
                let has_moved = old
                    .as_ref()
                    .and_then(|old| old.channel_id.as_ref())
                    .is_some_and(|old| old != &channel_id);

                if has_moved && member.user.id == fm.bot_id {
                    tts::moved(ctx, channel_id, data).await?;

                    return Ok(());
                }

                if channel_id == ChannelId::new(secrets.temporal_wait) {
                    temporal_voice_join(ctx, member, guild_id, secrets.temporal_category).await?;
                } else {
                    tts::join(ctx, member, guild_id, channel_id, data).await?;
                }

                return Ok(());
            }

            if let Some(old) = old {
                temporal_voice_quit(ctx, &old.channel_id.unwrap()).await?;
                tts::quit(ctx, fm.bot_id, guild_id, old, data).await?;
                return Ok(());
            }

            Ok(())
        }
        _ => Ok(()),
    }
}
