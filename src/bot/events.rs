mod compiler;
mod godbolt;
mod join;
mod new_members_mention;
mod read_github_links;
pub mod temporal_voice;
mod tts;

use poise::serenity_prelude::{ChannelId, Context, FullEvent, GuildId, Member, VoiceState};
use poise::FrameworkContext;
use temporal_voice::{temporal_voice_join, temporal_voice_quit};
use tracing::info;

use crate::bot::{self, Data};
use crate::CangrebotSecrets;

#[expect(dead_code, reason = "Maybe it is useful in the future")]
enum VoiceChange<'a> {
    Join {
        member: &'a Member,

        state: &'a VoiceState,
        channel_id: ChannelId,
    },
    Move {
        member: &'a Member,

        new: &'a VoiceState,
        new_channel_id: ChannelId,

        old: &'a VoiceState,
        old_channel_id: ChannelId,
    },
    Quit {
        member: &'a Member,

        state: &'a VoiceState,
        channel_id: ChannelId,
    },
}

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

            let old = old.as_ref().and_then(|old| Some((old, old.channel_id?)));

            let change = match (old, new.channel_id) {
                // Join
                (None, Some(channel_id)) => {
                    VoiceChange::Join {
                        member,
                        state: new,
                        channel_id
                    }
                }

                // Move
                (Some((old, old_channel_id)), Some(new_channel_id)) if old_channel_id != new_channel_id => {
                    VoiceChange::Move {
                        member,
                        old,
                        old_channel_id,
                        new,
                        new_channel_id
                    }
                }

                // Quit
                (Some((old, old_channel)), None) => {
                    VoiceChange::Quit {
                        member,
                        state: old,
                        channel_id: old_channel
                    }
                }

                // Any other voice state update
                (Some(_), Some(_)) => return Ok(()),

                // Impossible
                (None, None) => unreachable!("If old and new state are none, it means that the user has no interaction with vc ")
            };

            match change {
                VoiceChange::Move {
                    member,
                    new_channel_id: channel_id,
                    ..
                }
                | VoiceChange::Join {
                    member, channel_id, ..
                } if channel_id == ChannelId::new(secrets.temporal_wait) => {
                    temporal_voice_join(ctx, member, guild_id, secrets.temporal_category).await?;
                }

                VoiceChange::Move { new_channel_id, .. } if member.user.id == fm.bot_id => {
                    tts::moved(ctx, new_channel_id, data).await?;
                }

                VoiceChange::Move {
                    new_channel_id: channel_id,
                    ..
                }
                | VoiceChange::Join { channel_id, .. } => {
                    tts::join(ctx, member, guild_id, channel_id, data).await?;
                }

                VoiceChange::Quit {
                    state, channel_id, ..
                } => {
                    temporal_voice_quit(ctx, &channel_id).await?;
                    tts::quit(ctx, fm.bot_id, guild_id, state, data).await?;
                }
            }

            Ok(())
        }
        _ => Ok(()),
    }
}
