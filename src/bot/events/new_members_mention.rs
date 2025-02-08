use poise::serenity_prelude::{ChannelId, Context, Message, RoleId};
use tokio_schedule::Job;

use crate::bot;

const NEW_MEMBERS_ROLE_ID: RoleId = RoleId::new(1263861260932485194);
const WELCOME_CHANNEL: ChannelId = ChannelId::new(778674893851983932);
// const INTERNAL_LOGS: ChannelId = ChannelId::new(1230718736206532628);

// async fn log(ctx: &Context, msg: impl serde::Serialize) {
//     ctx.http
//         .send_message(INTERNAL_LOGS, Vec::new(), &msg)
//         .await
//         .unwrap();
// }

pub async fn message(ctx: &Context, msg: &Message) -> Result<bool, bot::Error> {
    if !(msg.mention_roles.contains(&NEW_MEMBERS_ROLE_ID) && msg.channel_id == WELCOME_CHANNEL) {
        return Ok(false);
    }

    let members = msg
        .guild(ctx.cache.as_ref())
        .unwrap()
        .members
        .iter()
        .filter(|&(_, v)| v.roles.contains(&NEW_MEMBERS_ROLE_ID)).map(|(_, v)| v.clone())
        .collect::<Vec<_>>();

    tracing::info!("New Members: {}", members.len());

    {
        let ctx = ctx.clone();
        let members = members.clone();
        let remove_role = tokio_schedule::every(30)
            .minute()
            .until(&(chrono::Utc::now() + chrono::Duration::hours(1)))
            .in_timezone(&chrono::Utc)
            .perform(move || {
                let ctx = ctx.clone();
                let members = members.clone();
                async move {
                    for v in members.iter() {
                        if let Err(e) = v.remove_role(&ctx, NEW_MEMBERS_ROLE_ID).await {
                            tracing::error!(
                                "Failed to remove role of: {} - {:?}\nReason: {e:?}",
                                v.display_name(),
                                v.nick
                            );
                        }
                    }
                }
            });

        tokio::spawn(remove_role);
    }

    Ok(true)
}
