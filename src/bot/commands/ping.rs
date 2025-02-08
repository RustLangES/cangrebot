use crate::bot::{Context, Error};

/// Estoy vivo? Recuerdo el sonido... Ping!... Pong!
#[poise::command(slash_command, prefix_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("pong!").await?;
    Ok(())
}
