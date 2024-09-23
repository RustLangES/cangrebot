mod anti_spam;
mod compile;
mod join;
mod new_members_mention;
mod read_github_links;

use poise::serenity_prelude::{Context, FullEvent, GuildId};
use poise::FrameworkContext;
use tracing::info;

use crate::bot::{self, Data};

pub async fn handle(
    ctx: &Context,
    event: &FullEvent,
    _: FrameworkContext<'_, Data, bot::Error>,
    data: &Data,
) -> Result<(), bot::Error> {
    match event {
        FullEvent::Ready { data_about_bot, .. } => {
            info!("Logged in as {}", data_about_bot.user.name);
        }
        FullEvent::Message { new_message } => {
            if anti_spam::message(ctx, new_message).await?
                || compile::message(ctx, new_message).await?
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
        _ => {}
    }

    Ok(())
}
