use crate::bot::{Context, Error};
use chrono::{DateTime, Utc};
use poise::{serenity_prelude::CreateEmbed, CreateReply};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

/// Muestra la estadistica de cangrebot
#[poise::command(slash_command, prefix_command)]
pub async fn stats(ctx: Context<'_>) -> Result<(), Error> {
    let start: Instant = Instant::now();

    let bot_user = ctx.serenity_context().cache.current_user().clone();

    let server_count = ctx.serenity_context().cache.guilds().len();

    let latency = start.elapsed().as_millis();

    let guild_id = ctx.guild_id().ok_or("Guild ID not found")?;
    let member_count = ctx
        .serenity_context()
        .cache
        .guild(guild_id)
        .map(|guild| guild.member_count)
        .unwrap_or(0);

    let server_creation = guild_id.created_at().unix_timestamp();

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let server_creation_time = DateTime::<Utc>::from_utc(
        chrono::NaiveDateTime::from_timestamp(server_creation as i64, 0),
        Utc,
    );

    let now_time =
        DateTime::<Utc>::from_utc(chrono::NaiveDateTime::from_timestamp(now as i64, 0), Utc);

    let server_age = now_time.signed_duration_since(server_creation_time);

    let _years = server_age.num_days() / 365;
    let _months = (server_age.num_days() % 365) / 30;
    let _days = (server_age.num_days() % 365) % 30;
    let uptime_message = format!("{} años, {} meses, {} días", _years, _months, _days);

    let embed = CreateEmbed::new()
        .title("📊 Estadísticas del Bot")
        .color(0xEA9010)
        .field("🤖 Nombre", &bot_user.name, true)
        .field("🌍 Servidores activos", &server_count.to_string(), true)
        .field("🏁 Creado hace", uptime_message, true)
        .field("👥 Miembros activos", format!("{}", member_count), true)
        .field("📡 Latencia", &format!("{} ms", latency), true);

    let replay = CreateReply::default();

    ctx.send(replay.embed(embed)).await?;
    Ok(())
}

