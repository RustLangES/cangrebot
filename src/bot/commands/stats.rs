use crate::bot::{Context, Error};
use serde_json::json;
use poise::serenity_prelude::{ChannelType,OnlineStatus};

///muestra estadisticas del bot en Json
#[poise::command(slash_command, prefix_command)]
pub async fn server_info(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = match ctx.guild_id() {
        Some(id) => id,
        None => {
            ctx.say("este comando se usa en server").await?;
            return Ok(());
        }
    };

    let http = ctx.serenity_context().http.clone();
    let guild = guild_id.to_partial_guild(&http).await?;

    let name = guild.name.clone();
    let roles_names:Vec<String> = guild
        .roles
        .values()
        .filter(|role| role.name != "@everyone")
        .map(|role| role.name.clone())
        .collect();


    let channels = guild.channels(&http).await?;
    let total_channels = channels
    .values()
    .filter(|channel| channel.kind != ChannelType::Category)
    .count();
    let total_roles = guild.roles
    .values()
    .filter(|role| role.name != "@everyone")
    .count();


    let presences = ctx
    .serenity_context()
    .cache
    .guild(guild_id)
    .map(|guild| guild.presences.clone())
    .unwrap_or_default();


    let active_members = guild
    .members(&http, None, None)
    .await?
    .iter()
    .filter(|member| {
        if let Some(presence) = presences.get(&member.user.id) {
            matches!(
                presence.status,
                OnlineStatus::Online | OnlineStatus::Idle | OnlineStatus::DoNotDisturb
            )
        } else {
            false
        }
    })
    .count();

    let total_members = guild.members(&http, None, None).await?.len();

    let has_icon = guild.icon.is_some();
    let created_at = guild.id.created_at().format("%Y-%m-%d %H:%M:%S").to_string();
    let level_verificacion = format!("{:?}", guild.verification_level);
    let emojis = guild.emojis.len();
    let boots= guild.premium_subscription_count.unwrap_or(0);
    let level_boost = format!("{:?}", guild.premium_tier);
    let features = guild.features.clone();

    let stats = json!({
        "name": name,
        "roles_names": roles_names,
        "total_channels": total_channels,
        "total_roles": total_roles,
        "total_members": total_members,
        "active_members": active_members,
        "has_icon": has_icon,
        "level_verificacion": level_verificacion,
        "emojis": emojis,
        "boosts": boots,
        "nivel_boost": level_boost,
        "features": features,
        "created_at": created_at
    });

    
    ctx.say(format!("```json\n{:#}\n```", stats)).await?;

    Ok(())
}
