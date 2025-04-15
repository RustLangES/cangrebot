use crate::bot::{Context, Error};
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::{ChannelType, CreateAttachment, Message, OnlineStatus};
use serde_json::{json, Value};
use std::collections::{BTreeMap, HashMap};
use std::fs::{self, File}; //para crear el archivo y leer contenido
use std::io::Write; // para que en el archivo genere el texto en el json :v
use std::path::Path; // para la ruta del archivo xd
///muestra estadisticas del bot en Json
#[poise::command(slash_command, prefix_command)]
pub async fn server_info(
    ctx: Context<'_>,
    #[description = "Reiniciar historial de mensajes anteriores"] reset: Option<bool>,
) -> Result<(), Error> {
    ctx.defer().await?;
    let guild_id = match ctx.guild_id() {
        Some(id) => id,
        None => {
            ctx.say("este comando se usa en server").await?;
            return Ok(());
        }
    };

    let http = ctx.serenity_context().http.clone();
    let guild = guild_id.to_partial_guild(&http).await?;

    //para resetear los messsage
    if reset.unwrap_or(false) {
        let reset_path = Path::new("last_messages.json");
        if reset_path.exists() {
            fs::remove_file(reset_path)?; // Borra el archivo viejo
        }
    }

    let name = guild.name.clone();
    let roles_names: Vec<String> = guild
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
    let total_roles = guild
        .roles
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

    let created_at = guild
        .id
        .created_at()
        .format("%Y-%m-%d %H:%M:%S")
        .to_string();
    //let level_verificacion = format!("{:?}", guild.verification_level);
    //let emojis = guild.emojis.len();
    let boots = guild.premium_subscription_count.unwrap_or(0);
    let level_boost = format!("{:?}", guild.premium_tier);
    let features = guild.features.clone();

    //miembros
    let mut messages_by_channel: HashMap<String, Vec<Value>> = HashMap::new();

    let last_ids_path = Path::new("last_messages.json");
    let mut last_message_ids: BTreeMap<String, String> = if last_ids_path.exists() {
        let content = fs::read_to_string(last_ids_path)?;
        serde_json::from_str(&content)?
    } else {
        BTreeMap::new()
    };

    for (channel_id, channel) in channels
        .iter()
        .map(|(id, ch)| (id as &serenity::ChannelId, ch))
    {
        if channel.kind != ChannelType::Text {
            continue;
        }

        let get_messages = {
            let mut builder = serenity::builder::GetMessages::default().limit(100);

            if let Some(last_id) = last_message_ids.get(&channel.name) {
                if let Ok(msg_id) = last_id.parse::<u64>() {
                    builder = builder.before(serenity::MessageId::new(msg_id));
                }
            }

            builder
        };
        let msgs = channel_id.messages(&http, get_messages).await?;
        let extracted_msgs: Vec<Value> = msgs
            .iter()
            .filter(|msg| !msg.author.bot && msg.webhook_id.is_none())
            .map(|msg: &Message| {
                json!({
                    "channel_id": channel_id.to_string(),
                    "channel_name": channel.name.clone(),
                    "user_id": msg.author.id.to_string(),
                    "username": msg.author.name,
                    "timestamp": msg.timestamp.format("%Y-%m-%d %H:%M:%S").to_string(),
                    "content": msg.content
                })
            })
            .collect();

        messages_by_channel.insert(channel.name.clone(), extracted_msgs);

        if let Some(oldest) = msgs.last() {
            last_message_ids.insert(channel.name.clone(), oldest.id.to_string());
        }
    }
    //miembros news :)
    let miembros = guild.members(&http, None, None).await?;
    let new_members: Vec<Value> = miembros
        .iter()
        .filter_map(|m| {
            m.joined_at.map(|fecha| {
                json!({
                    "user_id": m.user.id.to_string(),
                    "username": m.user.name,
                    "joined_at": fecha.format("%Y-%m-%d %H:%M:%S").to_string()
                })
            })
        })
        .collect();

    //boosts del servidor
    let boosters: Vec<Value> = miembros
        .iter()
        .filter_map(|m: &serenity::Member| {
            if let Some(premium_since) = m.premium_since {
                Some(json!({
                    "user_id": m.user.id.to_string(),
                    "username": m.user.name,
                    "boosted_since": premium_since.format("%Y-%m-%d %H:%M:%S").to_string()
                }))
            } else {
                None
            }
        })
        .collect();
    //envia los ultimos 100 mensajes del canal

    let stats = json!({
        "name": name,
        "roles_names": roles_names,
        "total_channels": total_channels,
        "total_roles": total_roles,
        "total_members": total_members,
        "active_members": active_members,
        //"level_verificacion": level_verificacion,
        "boosts": boots,
        "nivel_boost": level_boost,
        "features": features,
        "created_at": created_at,
        "messages_by_channel": messages_by_channel,
        "new_members": new_members,
        "boosters": boosters,
    });

    //crea un archivo JSON "NO TOCAR"
    let file_path = Path::new("server_stats.json");
    let mut file = File::create(&file_path)?;
    write!(file, "{}", serde_json::to_string_pretty(&stats)?)?;

    // paso final, envia el archivo en JSON
    let filename = "server_stats.json";
    let mut last_file = File::create("last_messages.json")?;
    write!(
        last_file,
        "{}",
        serde_json::to_string_pretty(&last_message_ids)?
    )?;

    ctx.send(
        poise::CreateReply::default()
            .content("📄 Info del server en Json:")
            .attachment(CreateAttachment::bytes(
                std::fs::read(&file_path)?,
                filename,
            )),
    )
    .await?;

    Ok(())
}
