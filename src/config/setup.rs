use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::anyhow;
use shuttle_runtime::SecretStore;
use songbird::driver::Bitrate;
use songbird::SerenityInit;
use songbird::input::cached::Compressed;
use songbird::input::File;
use serenity::prelude::*;
use serenity::framework::{StandardFramework, standard::macros::hook};
use serenity::framework::standard::Configuration;
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
        .unrecognised_command(unknown_command)
        .group(&GENERAL_GROUP)
        .before(before);

        framework.configure(Configuration::new().prefix(prefix));


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

        load_sound_file(&mut audio_map, &public_folder, "loop", "loop.mp3").await;
        load_sound_file(&mut audio_map, &public_folder, "ting", "ting.mp3").await;
        load_sound_file(&mut audio_map, &public_folder, "song", "Cloudkicker_-_Loops_-_22_2011_07.mp3").await;


        data.insert::<SoundStore>(Arc::new(Mutex::new(audio_map)));
    }

    Ok(client)
}


async fn load_sound_file(audio_map: &mut HashMap<String, CachedSound>, public_folder: &PathBuf, sound: &str, song_file: &str) {
            // Creation of a compressed source.
        //
        // This is a full song, making this a much less memory-heavy choice.
        //
        // Music by Cloudkicker, used under CC BY-SC-SA 3.0 (https://creativecommons.org/licenses/by-nc-sa/3.0/).
        let src = public_folder.clone().join(song_file);
        let file = File::new(src);
        let song_src = Compressed::new(
            file.into(),
            Bitrate::Auto,
        )
        .await
        .expect("These parameters are well-defined.");
        let _ = song_src.raw.spawn_loader();

        // Compressed sources are internally stored as DCA1 format files.
        // Because `Compressed` implements `std::io::Read`, we can save these
        // to disk and use them again later if we want!
        let mut creator = song_src.new_handle();

        let song_file_compressed = song_file.replace("mp3", "dca");

        std::thread::spawn(move || {
            let mut out_file = std::fs::File::create(&song_file_compressed).unwrap();
            std::io::copy(&mut creator, &mut out_file).expect("Error writing out song!");
        });

        audio_map.insert(sound.into(), CachedSound::Compressed(song_src));
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