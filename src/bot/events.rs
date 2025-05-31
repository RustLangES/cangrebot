mod anti_spam;
mod compiler;
mod godbolt;
mod join;
mod new_members_mention;
mod read_github_links;
pub mod temporal_voice;

use poise::serenity_prelude::{ChannelId, Context, FullEvent, GuildId};
use poise::FrameworkContext;
use temporal_voice::{temporal_voice_join, temporal_voice_quit};
use tracing::info;

use crate::bot::{self, Data};
use crate::CangrebotSecrets;

pub async fn handle(
    ctx: &Context,
    event: &FullEvent,
    _: FrameworkContext<'_, Data, bot::Error>,
    data: &Data,
    secrets: &CangrebotSecrets,
) -> Result<(), bot::Error> {
    match event {
        FullEvent::Ready { data_about_bot, .. } => {
            info!("Logged in as {}", data_about_bot.user.name);
        }
        FullEvent::Message { new_message } => {
            if compiler::message(ctx, new_message, &secrets.discord_prefix).await?
                || new_members_mention::message(ctx, new_message).await?
                || read_github_links::message(ctx, new_message).await
            {
                return Ok(());
            }
        }

        FullEvent::GuildMemberAddition { new_member } => {
            join::guild_member_addition(ctx, &GuildId::new(data.secrets.guild_id), new_member)
                .await;
        }
        FullEvent::InteractionCreate { interaction } => {
            // for buttons
            if let Some(interaction) = interaction.as_message_component() {
                if read_github_links::handle_delete_embed(ctx, interaction).await
                    || read_github_links::handle_save_embed(ctx, interaction).await
                {
                    return Ok(());
                }
            }
        }
        FullEvent::VoiceStateUpdate { old, new } => {
            let Some(guild_id) = &new.guild_id else {
                return Ok(());
            };
            let Some(member) = &new.member else {
                return Ok(());
            };

            if let Some(channel_id) = new.channel_id {
                if channel_id == ChannelId::new(secrets.temporal_wait) {
                    temporal_voice_join(ctx, &member, &guild_id, secrets.temporal_category).await?;
                }
            }
            if let Some(old) = old {
                temporal_voice_quit(ctx, &old.channel_id.unwrap()).await?;
            }
        }
        _ => {}
    }

    Ok(())
}
