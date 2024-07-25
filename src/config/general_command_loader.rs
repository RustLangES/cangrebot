use crate::events::join::guild_member_addition;
use crate::general_commands::meetups::MEETUPS_COMMAND;
use crate::general_commands::ping::PING_COMMAND;
use serenity::framework::standard::macros::group;
use serenity::{async_trait, prelude::Context};
use serenity::{model::prelude::Member, prelude::EventHandler};

#[group]
#[commands(ping, meetups)]
pub struct General;

#[async_trait]
impl EventHandler for General {
    async fn guild_member_addition(&self, ctx: Context, member: Member) {
        println!("guild_member_addition");
        guild_member_addition(&ctx, &member.guild_id, &member).await;
    }
}
