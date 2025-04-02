use crate::bot::{Context, Error};
use poise::{serenity_prelude::CreateEmbed, CreateReply};
use std::time::Instant;

/// Estoy vivo? Recuerdo el sonido... Ping!... Pong!
#[poise::command(slash_command, prefix_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let start: Instant = Instant::now();

    
    ctx.say("Calculating...⌛").await?;

    
    let latency: u128 = start.elapsed().as_millis();

    
    let mensaje:String = format!("📡 Latencia: `{}` ms", latency);

    
    let embed = CreateEmbed::new()
        .title("🏓 Pong!")
        .description(mensaje)
        .color(0xEA9010);

    
    let replay = CreateReply::default();


    
    ctx.send(replay.embed(embed)).await?;
    Ok(())
}
