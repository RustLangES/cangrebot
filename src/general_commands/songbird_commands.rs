use std::{collections::HashMap, sync::{Arc, Weak}};

use serenity::{
    async_trait, client::Context, framework::standard::{Args, CommandResult}, model::channel::Message, prelude::{Mentionable, Mutex}, Result as SerenityResult
};
use serenity::all::standard::macros::command;

use songbird::{
    Call, Event, EventContext, EventHandler as VoiceEventHandler, TrackEvent
};

use crate::config::songbird_config::{CachedSound,SoundStore};

// This imports `typemap`'s `Key` as `TypeMapKey`.

#[command]
#[only_in(guilds)]
pub async fn deafen(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    let handler_lock = match manager.get(guild_id) {
        Some(handler) => handler,
        None => {
            check_msg(msg.reply(ctx, "Not in a voice channel").await);

            return Ok(());
        },
    };

    let mut handler = handler_lock.lock().await;

    if handler.is_deaf() {
        check_msg(msg.channel_id.say(&ctx.http, "Already deafened").await);
    } else {
        if let Err(e) = handler.deafen(true).await {
            check_msg(msg.channel_id.say(&ctx.http, format!("Failed: {:?}", e)).await);
        }

        check_msg(msg.channel_id.say(&ctx.http, "Deafened").await);
    }

    Ok(())
}

#[command]
#[bucket = "A"]
#[only_in("guilds")]
#[description("join voice channel")]
async fn join(ctx: &Context, msg: &Message) -> CommandResult {
    let voice_states = match msg.guild(ctx.cache.as_ref()) {
        Some(guild) => guild.voice_states.clone(),
        None => { return Ok(()); }
      };

    let guild_id = msg.guild_id.unwrap();
    let channel_id = voice_states
    .get(&msg.author.id)
    .and_then(|voice_state| voice_state.channel_id);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            check_msg(msg.reply(ctx, "Not in a voice channel").await);

            return Ok(());
        }
    };

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    if let Ok(handler_lock) = manager.join(guild_id, connect_to).await {
        let call_lock_for_evt = Arc::downgrade(&handler_lock);

        let mut handler = handler_lock.lock().await;

        check_msg(
            msg.channel_id
                .say(&ctx.http, &format!("Joined {}", connect_to.mention()))
                .await,
        );

        let sources_lock = ctx
            .data
            .read()
            .await
            .get::<SoundStore>()
            .cloned()
            .expect("Sound cache was installed at startup.");
        let sources_lock_for_evt = sources_lock.clone();
        let sources = sources_lock.lock().await;
        let source = sources
            .get("song")
            .expect("Handle placed into cache at startup.");

        let song = handler.play(source.into());
        let _ = song.set_volume(1.0);
        let _ = song.enable_loop();

        // Play a guitar chord whenever the main backing track loops.
        let _ = song.add_event(
            Event::Track(TrackEvent::Loop),
            LoopPlaySound {
                call_lock: call_lock_for_evt,
                sources: sources_lock_for_evt,
            },
        );

    } else {
        check_msg(
            msg.channel_id
                .say(&ctx.http, "Error joining the channel")
                .await,
        );
    }

    Ok(())
}

pub struct LoopPlaySound {
    call_lock: Weak<Mutex<Call>>,
    sources: Arc<Mutex<HashMap<String, CachedSound>>>,
}

#[async_trait]
impl VoiceEventHandler for LoopPlaySound {
    async fn act(&self, _ctx: &EventContext<'_>) -> Option<Event> {
        if let Some(call_lock) = self.call_lock.upgrade() {
            let src: songbird::tracks::Track = {
                let sources = self.sources.lock().await;
                sources.get("loop").expect("Handle placed into cache at startup.").into()
            };

            let mut handler = call_lock.lock().await;
            let sound = handler.play(src);
            let _ = sound.set_volume(0.5);
            if let Err(error) = sound.disable_loop() {
                tracing::error!("Hubo un problema: {}", error);
            };
        }else {
            println!("no se pudo");
        }

        None
    }
}

#[command]
#[only_in(guilds)]
pub async fn leave(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();
    let has_handler = manager.get(guild_id).is_some();

    if has_handler {
        if let Err(e) = manager.remove(guild_id).await {
            check_msg(msg.channel_id.say(&ctx.http, format!("Failed: {:?}", e)).await);
        }

        check_msg(msg.channel_id.say(&ctx.http, "Left voice channel").await);
    } else {
        check_msg(msg.reply(ctx, "Not in a voice channel").await);
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
pub async fn mute(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    let handler_lock = match manager.get(guild_id) {
        Some(handler) => handler,
        None => {
            check_msg(msg.reply(ctx, "Not in a voice channel").await);

            return Ok(());
        },
    };

    let mut handler = handler_lock.lock().await;

    if handler.is_mute() {
        check_msg(msg.channel_id.say(&ctx.http, "Already muted").await);
    } else {
        if let Err(e) = handler.mute(true).await {
            check_msg(msg.channel_id.say(&ctx.http, format!("Failed: {:?}", e)).await);
        }

        check_msg(msg.channel_id.say(&ctx.http, "Now muted").await);
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
pub async fn ting(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        let sources_lock = ctx.data.read().await.get::<SoundStore>().cloned().expect("Sound cache was installed at startup.");
        let sources = sources_lock.lock().await;

        let Some(source) = sources.get("ting") else {
            check_msg(msg.channel_id.say(&ctx.http, "Hubo un problema cargando el sonido!").await);
            return Ok(())
        };

        handler.stop();

        let sound = handler.play_only(source.into());

        let _ = sound.set_volume(1.0);
        
        check_msg(msg.channel_id.say(&ctx.http, "Ting!").await);
    } else {
        check_msg(msg.channel_id.say(&ctx.http, "Not in a voice channel to play in").await);
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
pub async fn undeafen(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        if let Err(e) = handler.deafen(false).await {
            check_msg(msg.channel_id.say(&ctx.http, format!("Failed: {:?}", e)).await);
        }

        check_msg(msg.channel_id.say(&ctx.http, "Undeafened").await);
    } else {
        check_msg(msg.channel_id.say(&ctx.http, "Not in a voice channel to undeafen in").await);
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
pub async fn unmute(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        if let Err(e) = handler.mute(false).await {
            check_msg(msg.channel_id.say(&ctx.http, format!("Failed: {:?}", e)).await);
        }

        check_msg(msg.channel_id.say(&ctx.http, "Unmuted").await);
    } else {
        check_msg(msg.channel_id.say(&ctx.http, "Not in a voice channel to unmute in").await);
    }

    Ok(())
}

fn check_msg(result: SerenityResult<Message>) {
    if let Err(why) = result {
        println!("Error sending message: {:?}", why);
    }
}