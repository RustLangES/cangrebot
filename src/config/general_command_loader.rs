use serenity::framework::standard::macros::group;
use crate::general_commands::ping::PING_COMMAND;


#[group]
#[commands(ping)]
pub struct General;

