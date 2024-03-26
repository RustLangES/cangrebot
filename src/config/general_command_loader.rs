use serenity::framework::standard::macros::group;
use serenity::{prelude::EventHandler, model::prelude::Member};
use crate::general_commands::ping::PING_COMMAND;
use crate::general_commands::meetups::MEETUPS_COMMAND;
use crate::general_commands::songbird_commands::DEAFEN_COMMAND;
use crate::general_commands::songbird_commands::JOIN_COMMAND;
use crate::general_commands::songbird_commands::LEAVE_COMMAND;
use crate::general_commands::songbird_commands::MUTE_COMMAND;
use crate::general_commands::songbird_commands::TING_COMMAND;
use crate::general_commands::songbird_commands::UNDEAFEN_COMMAND;
use crate::general_commands::songbird_commands::UNMUTE_COMMAND;
use serenity::{prelude::Context, async_trait, };
use crate::events::join::guild_member_addition;

#[group]
#[commands(ping, meetups, deafen, join, leave, mute, ting, undeafen, unmute)]
pub struct General;

#[async_trait]
impl EventHandler for General {

    async fn guild_member_addition(&self, ctx: Context, member: Member) {
        println!("guild_member_addition");
        guild_member_addition(&ctx, &member.guild_id, &member).await; 
    }

}