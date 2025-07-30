use crate::api::routes::send_stats::{send_stats_to_api, ServerStats};
use crate::bot::{Context, Error};
use anyhow::anyhow;
use chrono::Utc;
use poise::serenity_prelude::{ChannelType, GetMessages, GuildId, Message, Timestamp};
use serde_json::{json, Value};

#[poise::command(slash_command, prefix_command)]
#[allow(clippy::too_many_lines)] // TODO: too many lines allowed until someone works reworks this function
pub async fn send_stats(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;

    // 1. Asegurarnos de que estamos en un guild
    let guild_id: GuildId = ctx
        .guild_id()
        .ok_or_else(|| anyhow!("Este comando solo funciona en un servidor"))?;

    // 2. Obtener PartialGuild con counts
    let guild = guild_id
        .to_partial_guild_with_counts(&ctx.http())
        .await
        .map_err(|_| anyhow!("No se pudo obtener la info del servidor. Revisa permisos"))?;

    // 3. Traer todos los canales
    let channels = guild.channels(&ctx.http()).await?;

    // ─────────────────────────────
    // 4. Recopilar mensajes de texto
    // ─────────────────────────────
    let mut all_messages: Vec<Message> = Vec::new();
    for channel in channels.values() {
        if channel.kind == ChannelType::Text {
            if let Ok(msgs) = channel
                .messages(&ctx.http(), GetMessages::new().limit(100))
                .await
            {
                all_messages.extend(msgs);
            }
        }
    }
    // Ordenar de más nuevo a más viejo y quedarnos con los 100 primeros
    all_messages.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    let latest_100 = all_messages.into_iter().take(100).collect::<Vec<_>>();

    let serialized_msgs: Vec<Value> = latest_100
        .iter()
        .map(|m| {
            json!({
                "id": m.id.to_string(),
                "channel_id": m.channel_id.to_string(),
                "author": {
                    "id": m.author.id.to_string(),
                    "name": m.author.name,
                },
                "content": m.content,
                "timestamp": m.timestamp.to_string(),
            })
        })
        .collect();

    // ─────────────────────────────
    // 5. Capturar nuevos miembros del día
    // ─────────────────────────────
    let members = guild_id
        .members(&ctx.http(), None, None)
        .await
        .map_err(|_| anyhow!("No se pudieron enumerar los miembros"))?;

    let today = Utc::now().date_naive();
    let _new_members: Vec<Value> = members // TODO: Underscored until is used
        .iter()
        .filter_map(|m| {
            m.joined_at.map(|dt| {
                let date = dt.naive_utc().date();
                (m, date)
            })
        })
        .filter(|(_, date)| *date == today)
        .map(|(m, date)| {
            json!({
                "id": m.user.id.to_string(),
                "name": m.user.name.clone(),
                "joined_at": date.to_string(),
            })
        })
        .collect();

    // ─────────────────────────────
    // 6. variable xd
    // ─────────────────────────────

    let members = guild_id
        .members(&ctx.http(), Some(1000), None)
        .await
        .map_err(|_| anyhow!("No se pudieron enumerar los miembros"))?;

    let now_chrono = chrono::Utc::now();
    let threshold =
        Timestamp::from_unix_timestamp((now_chrono - chrono::Duration::hours(24)).timestamp())
            .expect("Invalid timestamp");

    let new_members: Vec<Value> = members
        .into_iter()
        .filter_map(|m| m.joined_at.map(|joined_at| (m, joined_at)))
        .filter(|(_, joined_at)| *joined_at > threshold)
        .map(|(m, joined_at)| {
            json!({
                "id": m.user.id.to_string(),
                "name": m.user.name,
                "joined_at": joined_at.to_string(),
            })
        })
        .collect();

    // ─────────────────────────────
    // 7. Construir ServerStats y enviar
    // ─────────────────────────────
    let stats = ServerStats {
        guild_id: guild_id.to_string(),
        guild_name: guild.name.clone(),
        total_members: usize::try_from(guild.approximate_member_count.unwrap_or(0))?,
        active_members: usize::try_from(guild.approximate_presence_count.unwrap_or(0))?,
        total_channels: channels.len(),
        total_messages: latest_100.len() as u64,
        daily_messages: 0,
        monthly_messages: 0,
        latest_messages: Some(serialized_msgs),
        new_members: Some(new_members),
    };

    send_stats_to_api(stats.clone())
        .await
        .map_err(|e| anyhow!("Error al enviar las estadísticas: {}", e))?;

    ctx.say(format!(
        "✅ Estadísticas enviadas. Se incluyeron {} mensajes y {} nuevos miembros.",
        stats.total_messages,
        stats.new_members.as_ref().map_or(0, std::vec::Vec::len)
    ))
    .await?;

    Ok(())
}
