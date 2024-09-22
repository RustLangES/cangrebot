use crate::bot::{Context, Error};

/// Retorna el link de invitación del servidor
#[poise::command(slash_command, prefix_command)]
pub async fn invite(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("https://discord.gg/4ng5HgmaMg").await?;
    Ok(())
}
