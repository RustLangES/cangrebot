use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::anyhow;
use shuttle_secrets::SecretStore;
use songbird::driver::Bitrate;
use songbird::{SerenityInit, input};
use songbird::input::cached::{Memory, Compressed};
use serenity::prelude::*;
use serenity::framework::{StandardFramework, standard::macros::hook};
use serenity::model::channel::Message;
use tracing::{instrument, info};

use crate::config::songbird_config::{CachedSound, SoundStore};

use super::general_command_loader::GENERAL_GROUP;

use super::slash_command_loader::Handler;

pub async fn setup( secret_store: SecretStore, public_folder: PathBuf) -> Result<Client, anyhow::Error> {
    // Get the discord token set in `Secrets.toml`
        let token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
            token
        } else {
            return Err(anyhow!("'DISCORD_TOKEN' was not found"));
        };

        let prefix = secret_store.get("DISCORD_PREFIX").unwrap_or("!".to_string());

        let framework = StandardFramework::new()
        .configure(|c| c.prefix(prefix)) // set the bot's prefix to "!"
        .before(before)
        .unrecognised_command(unknown_command)
        .group(&GENERAL_GROUP);


        info!("Starting bot with token: {}", token);
    
        // Set gateway intents, which decides what events the bot will be notified about
        let intents = 
        GatewayIntents::all();
    
        info!("{:?}", secret_store.get("GUILD_ID"));

        let Some(guild_id) = secret_store.get("GUILD_ID") else {
            return Err(anyhow!("'GUILD_ID' was not found"));
        };
        let handler = Handler(guild_id.parse().unwrap());

        let client = Client::builder(token, intents)
            .event_handler(handler)
            .framework(framework)
            .register_songbird()
            .await
            .expect("Error creating client");
    // Obtain a lock to the data owned by the client, and insert the client's
    // voice manager into it. This allows the voice manager to be accessible by
    // event handlers and framework commands.
    {
        let mut data = client.data.write().await;

        // Loading the audio ahead of time.
        let mut audio_map = HashMap::new();

        // Creation of an in-memory source.
        //
        // This is a small sound effect, so storing the whole thing is relatively cheap.
        //
        // `spawn_loader` creates a new thread which works to copy all the audio into memory 
        // ahead of time. We do this in both cases to ensure optimal performance for the audio
        // core.
        let ting_src = Memory::new(
            input::ffmpeg(public_folder.clone().join("ting.wav")).await.unwrap_or_else(|_| panic!("File should be in root folder. {}", public_folder.clone().join("ting.wav").as_path().to_str().unwrap_or_default())),
        ).expect("These parameters are well-defined.");
        let _ = ting_src.raw.spawn_loader();
        audio_map.insert("ting".into(), CachedSound::Uncompressed(ting_src));

        // Another short sting, to show where each loop occurs.
        let loop_src = Memory::new(
            input::ffmpeg(public_folder.clone().join("loop.wav")).await.expect("File should be in root folder."),
        ).expect("These parameters are well-defined.");
        let _ = loop_src.raw.spawn_loader();
        audio_map.insert("loop".into(), CachedSound::Uncompressed(loop_src));

        // Creation of a compressed source.
        //
        // This is a full song, making this a much less memory-heavy choice.
        //
        // Music by Cloudkicker, used under CC BY-SC-SA 3.0 (https://creativecommons.org/licenses/by-nc-sa/3.0/).
        let song_src = Compressed::new(
                input::ffmpeg(public_folder.clone().join("Cloudkicker_-_Loops_-_22_2011_07.mp3")).await.expect("Link may be dead."),
                Bitrate::BitsPerSecond(128_000),
            ).expect("These parameters are well-defined.");
        let _ = song_src.raw.spawn_loader();
        audio_map.insert("song".into(), CachedSound::Compressed(song_src));

        data.insert::<SoundStore>(Arc::new(Mutex::new(audio_map)));
    }

        Ok(client)
}


#[hook]
#[instrument]
pub async fn before(_: &Context, msg: &Message, command_name: &str) -> bool {
    info!("Got command '{}' by user '{}'", command_name, msg.author.name);

    true
}

#[hook]
pub async fn unknown_command(_ctx: &Context, _msg: &Message, unknown_command_name: &str) {
    info!("Could not find command named '{}'", unknown_command_name);
}