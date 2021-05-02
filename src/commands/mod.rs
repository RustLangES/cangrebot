pub mod ping;

use ping::*;
use serenity::framework::standard::macros::group;

#[group]
#[commands(ping)]
struct General;
