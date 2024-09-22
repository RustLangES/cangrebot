mod explain;
mod help;
mod invite;
mod krate;
mod ping;
mod suggest;

use super::{Data, Error};

pub fn commands() -> Vec<poise::Command<Data, Error>> {
    vec![
        explain::explain(),
        help::help(),
        invite::invite(),
        krate::cargo(),
        ping::ping(),
        suggest::sugerencia(),
    ]
}
