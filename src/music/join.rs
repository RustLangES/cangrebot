use serenity::all::{CommandInteraction, Context, UserId};
use songbird::events::TrackEvent;

use super::events;

pub async fn join(ctx: &Context, cmd: &CommandInteraction) -> Result<(), String> {
    let guild_id = cmd
        .guild_id
        .expect("Server ID on interaction. Only can be called from servers");

    let author_id = cmd.user.id;

    let connect_to = {
        let guild = ctx
            .cache
            .guild(guild_id)
            .expect("Server on interaction. Possible Bot is out the server");

        let is_god = author_id == UserId::new(330194401861042176); // Is Lemi chan
        let connect_to = if is_god {
            if let Some(a) = guild
                .voice_states
                .get(&ctx.cache.current_user().id)
                .and_then(|voice_state| voice_state.channel_id)
            {
                a
            } else {
                return Err(String::from("Not in a voice channel"));
            }
        } else {
            if let Some(a) = guild
                .voice_states
                .get(&author_id)
                .and_then(|voice_state| voice_state.channel_id)
            {
                a
            } else {
                return Err(String::from("Not in a voice channel"));
            }
        };

        connect_to
    };

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Ok(handler_lock) = manager.join(guild_id, connect_to).await {
        let mut handler = handler_lock.lock().await;
        handler.add_global_event(TrackEvent::Error.into(), events::MusicEventErrorHandler);
    }

    Ok(())
}
