use std::sync::Arc;

use poise::serenity_prelude::{
    futures::future::join_all, prelude::TypeMapKey, ChannelId, ChannelType, Context, CreateChannel,
    GuildChannel, GuildId, Member,
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
                    if channel.members(&ctx).is_ok_and(|m| m.len() == 0) {
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
    let mut data = ctx.data.write().await;
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
        let data = ctx.data.read().await;
        data.get::<TempVcStore>().cloned()
    };

    if store_mutex.is_some() {
        channels_list = store_mutex.unwrap().lock().await.to_vec();
    }

    let builder = CreateChannel::new(format!("🗣-&{}.vc()", member.user.name))
        .kind(ChannelType::Voice)
        .category(category);
    let Ok(temp_channel) = guild_id.create_channel(&ctx, builder).await else {
        return Err(bot::Error::from("Error al crear el canal temporal"));
    };
    let member_move = member.move_to_voice_channel(&ctx, &temp_channel).await;

    if member_move.is_ok() {
        channels_list.push(temp_channel.id);
        let mut data = ctx.data.write().await;
        data.insert::<TempVcStore>(Arc::new(Mutex::new(channels_list.clone())));
        return Ok(());
    } else {
        return Err(bot::Error::from(
            "Error al mover al miembro al canal temporal",
        ));
    }
}

pub async fn temporal_voice_quit(ctx: &Context, channel: &ChannelId) -> Result<(), bot::Error> {
    let mut channels_list: Vec<ChannelId> = vec![];
    let store_mutex = {
        let data = ctx.data.read().await;
        data.get::<TempVcStore>().cloned()
    };

    if store_mutex.is_some() {
        channels_list = store_mutex.unwrap().lock().await.to_vec();
    }

    let Ok(channel) = channel.to_channel(&ctx).await else {
        return Err(bot::Error::from("Error al obtener canal temporal"));
    };

    let Some(channel_guild) = channel.guild() else {
        return Err(bot::Error::from(
            "Error al convertir ChannelId en GuildChannel",
        ));
    };

    if channels_list.contains(&channel_guild.id) && channel_guild.members(&ctx).unwrap().len() == 0
    {
        if let Ok(_) = channel_guild.delete(&ctx).await {
            channels_list.remove(
                channels_list
                    .iter()
                    .position(|cid| channel_guild.id == *cid)
                    .unwrap(),
            );
            let mut data = ctx.data.write().await;
            data.insert::<TempVcStore>(Arc::new(Mutex::new(channels_list.clone())));

            return Ok(());
        };
    }
    return Err(bot::Error::from("Error al eliminar canal temporal"));
}
