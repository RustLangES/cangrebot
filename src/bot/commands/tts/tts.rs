use crate::bot;
use reqwest::Client;
use songbird::input::HttpRequest;
use tracing::info;
use urlencoding::encode;

#[poise::command(slash_command, prefix_command, guild_only)]
pub async fn tts(ctx: bot::Context<'_>, text: String) -> Result<(), bot::Error> {
    let guild_id = ctx.guild().ok_or("No guild context")?.id;
    info!("Guild ID: {}", guild_id);

    let manager = songbird::get(ctx.serenity_context())
        .await
        .ok_or("")?
        .clone();

    let Some(handler_lock) = manager.get(guild_id) else {
        ctx.say("no vc").await?;
        return Ok(());
    };

    let url = format!(
        "https://translate.google.com/translate_tts?client=tw-ob&tl=es&q={}",
        encode(&text)
    );

    let data = HttpRequest::new(Client::new(), url).clone();

    handler_lock.lock().await.play_input(data.into());

    Ok(())
}
