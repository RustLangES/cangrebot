use anyhow::anyhow;
use shuttle_secrets::SecretStore;
use tracing::info;
use serenity::prelude::*;
use serenity::framework::{StandardFramework, standard::macros::hook};
use serenity::model::channel::Message;
use tracing::instrument;

use super::general_command_loader::GENERAL_GROUP;
use super::general_command_loader::General;
use super::slash_command_loader::Handler;

pub async fn setup( secret_store: SecretStore) -> Result<Client, anyhow::Error> {
        // Get the discord token set in `Secrets.toml`
        let token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
            token
        } else {
            return Err(anyhow!("'DISCORD_TOKEN' was not found").into());
        };

    
        let prefix = secret_store.get("DISCORD_PREFIX").unwrap_or("!".to_string());

        let framework = StandardFramework::new()
        .configure(|c| c.prefix(prefix)) // set the bot's prefix to "!"
        .before(before)
        .unrecognised_command(unknown_command)
        .group(&GENERAL_GROUP);


        info!("Starting bot with token: {}", token);
    
        // Set gateway intents, which decides what events the bot will be notified about
        let intents = GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_BANS
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MESSAGE_REACTIONS
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::DIRECT_MESSAGE_REACTIONS;
    
        info!("{:?}", secret_store.get("GUILD_ID"));

        let Some(guild_id) = secret_store.get("GUILD_ID") else {
            return Err(anyhow!("'GUILD_ID' was not found").into());
        };
        let handler = Handler(guild_id.parse().unwrap());

        let client = Client::builder(token, intents)
            .event_handler(handler)
            .event_handler(General)
            .framework(framework)
            .await
            .expect("Error creating client");


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