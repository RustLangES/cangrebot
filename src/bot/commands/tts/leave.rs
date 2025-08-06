use crate::bot;

#[poise::command(slash_command, prefix_command, guild_only)]
pub async fn leave(ctx: bot::Context<'_>) -> Result<(), bot::Error> {
    let guild_id = ctx.guild().ok_or("")?.id;

    let manager = songbird::get(ctx.serenity_context())
        .await
        .ok_or("")?
        .clone();

    manager.leave(guild_id).await?;

    Ok(())
}
