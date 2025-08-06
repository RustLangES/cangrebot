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

use super::{Data, Error};

pub fn commands() -> Vec<poise::Command<Data, Error>> {
    vec![
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
        ask::ask(),
        tts::join::join(),
        tts::leave::leave(),
        tts::tts::tts(),
    ]
}
