use serenity::all::{ChannelId, CommandInteraction, Context, GuildId, Http, UserId};

use super::MusicStore;

pub async fn join(ctx: &Context, cmd: &CommandInteraction) -> Result<(), String> {
    let (guild_id, connect_to) = get_voice_channel(ctx, cmd).await?;

    let data = ctx.data.read().await;
    let lava_client = data.get::<MusicStore>().unwrap().lock().await;

    if lava_client.get_player_context(guild_id).is_none() {
        let manager = songbird::get(ctx)
            .await
            .expect("Songbird Voice client placed in at initialisation.")
            .clone();

        let handler = manager.join_gateway(guild_id, connect_to).await;

        match handler {
            Ok((connection_info, _)) => {
                if let Err(err) = lava_client
                    .create_player_context_with_data::<(ChannelId, std::sync::Arc<Http>)>(
                        guild_id,
                        connection_info,
                        std::sync::Arc::new((connect_to, ctx.http.clone())),
                    )
                    .await
                {
                    return Err(err.to_string());
                }

                return Ok(());
            }
            Err(why) => return Err(format!("Error joining the channel: {}", why)),
        }
    }

    Ok(())
}

async fn get_voice_channel(
    ctx: &Context,
    cmd: &CommandInteraction,
) -> Result<(GuildId, ChannelId), String> {
    let author_id = cmd.user.id;
    let is_god = author_id == UserId::new(330194401861042176); // Is Lemi chan

    let guild_id = cmd
        .guild_id
        .expect("Server ID on interaction. Only can be called from servers");

    let guild = ctx
        .cache
        .guild(guild_id)
        .expect("Server on interaction. Possible Bot is out the server");

    let voice_channels = &guild.voice_states;

    let author_channel_id = voice_channels
        .get(&author_id)
        .and_then(|voice_state| voice_state.channel_id);

    let bot_channel_id = if is_god {
        voice_channels
            .get(&ctx.cache.current_user().id)
            .and_then(|voice_state| voice_state.channel_id)
    } else {
        None
    };

    let Some(connect_to) = author_channel_id.or(bot_channel_id) else {
        return Err(String::from("No te encuentras en un canal de voz"));
    };

    Ok((guild_id, connect_to))
}
