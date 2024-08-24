use serenity::all::{ChannelId, Context, CreateMessage, EventHandler, Message, RoleId};
use serenity::async_trait;
use std::collections::HashMap;

pub struct NewMembersMention;

const NEW_MEMBERS_ROLE_ID: RoleId = RoleId::new(1263861260932485194);
const WELCOME_CHANNEL: ChannelId = ChannelId::new(778674893851983932);
const INTERNAL_LOGS: ChannelId = ChannelId::new(1230718736206532628);

async fn log(ctx: &Context, msg: impl serde::Serialize) {
    ctx.http
        .send_message(INTERNAL_LOGS, Vec::new(), &msg)
        .await
        .unwrap();
}

#[async_trait]
impl EventHandler for NewMembersMention {
    async fn message(&self, ctx: Context, msg: Message) {
        if !msg.content.starts_with("!new members") {
            return;
        }
        // if !(msg.mention_roles.contains(&NEW_MEMBERS_ROLE_ID) && msg.channel_id == WELCOME_CHANNEL)
        // {
        //     return;
        // }

        // let Some(guild) = msg.guild(ctx.cache.as_ref()) else {
        // ctx.http
        //     .send_message(
        //         INTERNAL_LOGS,
        //         Vec::new(),
        //         &format!("Cannot get GUILD from message: {}", msg.clone().link()),
        //     )
        //     .await
        //     .unwrap();
        // return;
        // };

        let mut message = "Resultado de reseteo de roles para los nuevos:\n\n".to_owned();

        let members = msg
            .guild(ctx.cache.as_ref())
            .unwrap()
            .members
            .iter()
            .filter(|(_, v)| v.roles.contains(&NEW_MEMBERS_ROLE_ID))
            .for_each(|(_, v)| message.push_str(&format!("- {}\n", v.display_name())));

        // for (_, v) in members.iter() {
        // let emoji = if v.remove_role(&ctx, NEW_MEMBERS_ROLE_ID).await.is_ok() {
        //     ":white_check_mark:"
        // } else {
        //     ":x:"
        // };
        // }

        if !message.is_empty() {
            INTERNAL_LOGS
                .send_message(&ctx, CreateMessage::new().content(&message))
                .await
                .unwrap();
        }
    }
}
