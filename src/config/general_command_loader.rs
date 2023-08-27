use serenity::{framework::standard::macros::group, prelude::EventHandler, model::prelude::Member};
use crate::general_commands::ping::PING_COMMAND;
use serenity::{prelude::Context, async_trait, };
use crate::events::join::guild_member_addition;


#[group]
#[commands(ping)]
pub struct General;

#[async_trait]
impl EventHandler for General {

    async fn guild_member_addition(&self, ctx: Context, member: Member) {
        println!("guild_member_addition");
        guild_member_addition(&ctx, &member.guild_id, &member).await; 
    }

}