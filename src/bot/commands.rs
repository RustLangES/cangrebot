pub mod ask;
mod crate_docs;
mod explain;
mod help;
mod invite;
mod krate;
mod ping;
mod selftimeout;
mod stats;
mod suggest;
mod tts;
mod wipe_commands;

use super::{Data, Error};
pub use tts::{TtsState, TtsStateExt};

pub fn commands() -> Vec<poise::Command<Data, Error>> {
    vec![
        ask::ask(),
        crate_docs::docs(),
        explain::explain(),
        help::help(),
        invite::invite(),
        krate::cargo(),
        ping::ping(),
        suggest::new::sugerencia(),
        suggest::implemented::sugerencia_implementada(),
        suggest::cancelled::sugerencia_cancelada(),
        //stats::send_stats(), TODO: Removed for now
        selftimeout::selftimeout(),
        tts::tts::tts(),
        wipe_commands::wipe_commands(),
    ]
}
