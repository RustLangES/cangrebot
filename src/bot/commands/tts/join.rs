use crate::bot;

#[poise::command(slash_command, prefix_command, guild_only)]
pub async fn join(ctx: bot::Context<'_>) -> Result<(), bot::Error> {
    let (guild_id, channel_id) = {
        let guild = ctx.guild().ok_or("")?;
        let channel_id = guild
            .voice_states
            .get(&ctx.author().id)
            .and_then(|voice_state| voice_state.channel_id);

        (guild.id, channel_id)
    };

    let Some(target_channel) = channel_id else {
        ctx.say("No  VC").await?;
        return Ok(());
    };

    let manager = songbird::get(ctx.serenity_context())
        .await
        .ok_or("")?
        .clone();

    if let Err(_) = manager.join(guild_id, target_channel).await {
        ctx.say("Ola").await?;
        return Ok(());
    };

    Ok(())
}
