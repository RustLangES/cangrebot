use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::anyhow;
use serenity::framework::standard::Configuration;
use serenity::framework::{standard::macros::hook, StandardFramework};
use serenity::model::channel::Message;
use serenity::prelude::*;
use shuttle_runtime::SecretStore;
use songbird::SerenityInit;
use tracing::{info, instrument};

use crate::config::songbird_config::SoundStore;
use crate::events::new_members_mention::NewMembersMention;

use super::general_command_loader::GENERAL_GROUP;

use crate::events::read_github_links::ReadGithubLinkHandler;

use super::slash_command_loader::Handler;

pub async fn setup(secret_store: SecretStore, _: PathBuf) -> Result<Client, anyhow::Error> {
    // Get the discord token set in `Secrets.toml`
    let token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
        token
    } else {
        return Err(anyhow!("'DISCORD_TOKEN' was not found"));
    };

    let prefix = secret_store
        .get("DISCORD_PREFIX")
        .unwrap_or("!".to_string());

    let framework = StandardFramework::new()
        .unrecognised_command(unknown_command)
        .group(&GENERAL_GROUP)
        .before(before);

    framework.configure(Configuration::new().prefix(prefix));

    info!("Starting bot with token: {}", token);

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::all();

    info!("{:?}", secret_store.get("GUILD_ID"));

    let Some(guild_id) = secret_store.get("GUILD_ID") else {
        return Err(anyhow!("'GUILD_ID' was not found"));
    };

    let handler = Handler::new(guild_id.parse().unwrap());

    let client = Client::builder(token, intents)
        .event_handler(handler)
        .event_handler(ReadGithubLinkHandler)
        .event_handler(NewMembersMention)
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
        let audio_map = HashMap::new();

        // load_sound_file(&mut audio_map, &public_folder, "loop", "loop.mp3").await;
        // load_sound_file(&mut audio_map, &public_folder, "ting", "ting.mp3").await;
        // load_sound_file(&mut audio_map, &public_folder, "song", "Cloudkicker_-_Loops_-_22_2011_07.mp3").await;

        data.insert::<SoundStore>(Arc::new(Mutex::new(audio_map)));
    }

    Ok(client)
}

#[hook]
#[instrument]
pub async fn before(_: &Context, msg: &Message, command_name: &str) -> bool {
    info!(
        "Got command '{}' by user '{}'",
        command_name, msg.author.name
    );

    true
}

#[hook]
pub async fn unknown_command(_ctx: &Context, _msg: &Message, unknown_command_name: &str) {
    info!("Could not find command named '{}'", unknown_command_name);
}
