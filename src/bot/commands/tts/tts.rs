use std::sync::Arc;

use crate::bot;
use poise::serenity_prelude::CreateEmbed;
use poise::CreateReply;
use reqwest::Client;
use songbird::input::HttpRequest;
use songbird::tracks::Track;
use tracing::info;
use urlencoding::encode;

#[poise::command(slash_command, prefix_command, guild_only)]
pub async fn tts(ctx: bot::Context<'_>, #[rest] text: String) -> Result<(), bot::Error> {
    let guild_id = ctx.guild().ok_or("No se pudo obtener el guild")?.id;

    let manager = songbird::get(ctx.serenity_context())
        .await
        .ok_or("No se pudo obtener el manager de voz")?
        .clone();

    let Some(handler_lock) = manager.get(guild_id) else {
        ctx.send(
            CreateReply::default().embed(
                CreateEmbed::new()
                    .title("Error")
                    .description("No estoy en ningún canal de voz. Usa /join primero.")
                    .color(0x00FF_0000),
            ),
        )
        .await?;
        return Ok(());
    };

    let text = format!("{} dice: {}", ctx.author().display_name(), &text);

    let url = format!(
        "https://translate.google.com/translate_tts?client=tw-ob&tl=es&q={}",
        encode(&text)
    );

    info!("requesting {}", url);

    let data = HttpRequest::new(Client::new(), url).clone();
    let t: Track = Track::new_with_data(data.into(), Arc::new(ctx.author().id));

    handler_lock.lock().await.enqueue(t).await;

    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::new()
                .title("TTS")
                .description(format!("Reproduciendo: {text}"))
                .color(0x0000_FF00),
        ),
    )
    .await?;

    Ok(())
}
