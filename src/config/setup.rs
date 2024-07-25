use std::path::PathBuf;
use std::sync::Arc;

use anyhow::anyhow;
use lavalink_rs::client::LavalinkClient;
use lavalink_rs::model::client::NodeDistributionStrategy;
use lavalink_rs::node::NodeBuilder;
use serenity::framework::standard::Configuration;
use serenity::framework::{standard::macros::hook, StandardFramework};
use serenity::model::channel::Message;
use serenity::prelude::*;
use shuttle_runtime::SecretStore;
use songbird::SerenityInit;
use tracing::{info, instrument};

use crate::music::{self, MusicStore};

use super::general_command_loader::GENERAL_GROUP;

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

    info!("Starting bot");

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::all();

    let Some(guild_id) = secret_store.get("GUILD_ID") else {
        return Err(anyhow!("'GUILD_ID' was not found"));
    };

    let handler = Handler::new(guild_id.parse().unwrap());

    let client = Client::builder(token, intents)
        .event_handler(handler)
        .framework(framework)
        .register_songbird()
        .await
        .expect("Error creating client");

    {
        let bot_id = client.cache.current_user().id.into();

        let mut data = client.data.write().await;

        data.insert::<MusicStore>(Arc::new(Mutex::new(MusicStore::new())));

        let events = music::events::get_music_events();

        let node_local = NodeBuilder {
            hostname: "lavalink.rustlang-es.org:2333".to_string(),
            is_ssl: false,
            events: lavalink_rs::model::events::Events::default(),
            password: secret_store.get("LAVALINK_PASSWORD").unwrap(),
            user_id: bot_id,
            session_id: None,
        };

        let client = LavalinkClient::new(
            events,
            vec![node_local],
            NodeDistributionStrategy::round_robin(),
        )
        .await;

        data
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
