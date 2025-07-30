use std::sync::Arc;

use poise::serenity_prelude::{
    futures::future::join_all, prelude::TypeMapKey, ChannelId, ChannelType, Context,
    CreateAllowedMentions, CreateChannel, CreateMessage, GuildChannel, GuildId, Member, Message,
    PermissionOverwrite, PermissionOverwriteType, Permissions,
};
use tokio::sync::Mutex;

use crate::bot;

struct TempVcStore;
impl TypeMapKey for TempVcStore {
    type Value = Arc<Mutex<Vec<ChannelId>>>;
}

pub async fn setup(ctx: &Context, guild_id: &GuildId, category: u64, waiting: u64) {
    let channels_list = Arc::new(Mutex::new(vec![]));
    let Ok(channels) = guild_id.channels(&ctx).await else {
        return;
    };

    let list_ref = Arc::clone(&channels_list);

    let futures = channels
        .iter()
        .filter_map(|(id, channel): (&ChannelId, &GuildChannel)| {
            if channel.kind == ChannelType::Voice
                && channel.parent_id.is_some_and(|id| id == category)
                && *id != waiting
            {
                let list_ref = Arc::clone(&list_ref);
                Some(async move {
                    if channel.members(ctx).is_ok_and(|c| c.is_empty()) {
                        let _ = channel.delete(&ctx).await;
                    } else {
                        let mut lock = list_ref.lock().await;
                        lock.push(*id);
                    }
                })
            } else {
                None
            }
        });

    join_all(futures).await;
    let mut data = { ctx.data.write().await };
    data.insert::<TempVcStore>(channels_list);
}

pub async fn temporal_voice_join(
    ctx: &Context,
    member: &Member,
    guild_id: &GuildId,
    category: u64,
) -> Result<(), bot::Error> {
    let mut channels_list: Vec<ChannelId> = vec![];
    let store_mutex = {
        let data = { ctx.data.read().await };
        data.get::<TempVcStore>().cloned()
    };

    if store_mutex.is_some() {
        channels_list = { store_mutex.unwrap().lock().await }.to_vec();
    }

    let builder = CreateChannel::new(format!("🗣-&{}.vc()", member.user.name))
        .kind(ChannelType::Voice)
        .rate_limit_per_user(5)
        .permissions([PermissionOverwrite {
            deny: Permissions::USE_EXTERNAL_STICKERS
                | Permissions::USE_EXTERNAL_SOUNDS
                | Permissions::USE_EXTERNAL_APPS
                | Permissions::USE_EXTERNAL_EMOJIS,
            allow: Permissions::empty(),
            kind: PermissionOverwriteType::Role(guild_id.everyone_role()),
        }])
        .category(category);
    let temp_channel = guild_id.create_channel(&ctx, builder).await;
    let Ok(temp_channel) = temp_channel else {
        return Err(bot::Error::from(format!(
            "Error al crear el canal temporal: {temp_channel:?}"
        )));
    };

    let member_move = member.move_to_voice_channel(&ctx, &temp_channel).await;

    if member_move.is_ok() {
        channels_list.push(temp_channel.id);
        let mut data = { ctx.data.write().await };
        data.insert::<TempVcStore>(Arc::new(Mutex::new(channels_list.clone())));
        return Ok(());
    }
    Err(bot::Error::from(
        "Error al mover al miembro al canal temporal",
    ))
}

pub async fn temporal_voice_quit(ctx: &Context, channel: &ChannelId) -> Result<(), bot::Error> {
    let mut channels_list: Vec<ChannelId> = vec![];
    let store_mutex = {
        let data = { ctx.data.read().await };
        data.get::<TempVcStore>().cloned()
    };

    if store_mutex.is_some() {
        channels_list = { store_mutex.unwrap().lock().await }.to_vec();
    }

    let Ok(channel) = channel.to_channel(&ctx).await else {
        return Err(bot::Error::from("Error al obtener canal temporal"));
    };

    let Some(channel_guild) = channel.guild() else {
        return Err(bot::Error::from(
            "Error al convertir ChannelId en GuildChannel",
        ));
    };

    if channels_list.contains(&channel_guild.id) && channel_guild.members(ctx).unwrap().is_empty() {
        if let Err(e) = channel_guild.delete(&ctx).await {
            return Err(bot::Error::from(format!(
                "Error al eliminar canal temporal: {e}"
            )));
        }
        channels_list.remove(
            channels_list
                .iter()
                .position(|cid| channel_guild.id == *cid)
                .unwrap(),
        );
        let mut data = { ctx.data.write().await };
        data.insert::<TempVcStore>(Arc::new(Mutex::new(channels_list.clone())));
    }
    Ok(())
}

pub async fn message(
    ctx: &Context,
    msg: &Message,
    log_channel: ChannelId,
) -> Result<bool, bot::Error> {
    if msg.author.bot {
        return Ok(false);
    }

    let store_mutex = {
        let data = { ctx.data.read().await };
        data.get::<TempVcStore>().cloned()
    };

    let mut channels_list: Vec<ChannelId> = vec![];
    if let Some(store) = store_mutex.as_ref() {
        let store = { store.lock().await };
        channels_list = store.to_vec();
    }

    if channels_list.contains(&msg.channel_id) {
        let message_builder = CreateMessage::new()
            .content(format!(
                "<t:{time}>\n{content}> {attachment}\nMessage Author: <@{author}>\nChannel: {channel}",
                time = msg
                    .edited_timestamp
                    .unwrap_or(msg.timestamp)
                    .timestamp(),
                content = if msg.content.is_empty() {
                    String::new()
                } else {
                    format!("> {}\n", msg.content)
                },
                attachment = msg
                    .attachments
                    .iter()
                    .map(|a| a.url.clone())
                    .collect::<Vec<_>>()
                    .join("\n> "),
                author = msg.author.id,
                channel = msg.channel_id.name(&ctx).await?
            ))
            .sticker_ids(msg.sticker_items.iter().map(|s| s.id).collect::<Vec<_>>())
            .allowed_mentions(
                CreateAllowedMentions::new()
                    .all_users(false)
                    .all_users(false),
            );
        let Err(e) = log_channel.send_message(&ctx, message_builder).await else {
            return Ok(true);
        };
        return Err(bot::Error::from(format!(
            "Error al loggear mensaje del chat temporal: {e}"
        )));
    }
    Ok(false)
}
