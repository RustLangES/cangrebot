mod cancelled;
mod implemented;
mod new;

use crate::bot;

/// Crea, Modifica y administra las sugerencias :D
#[poise::command(
    slash_command,
    prefix_command,
    subcommands("new::nueva", "implemented::implementada", "cancelled::cancelada")
)]
pub async fn sugerencia(_: bot::Context<'_>) -> Result<(), bot::Error> {
    Ok(())
}
