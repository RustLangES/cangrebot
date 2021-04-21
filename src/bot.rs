use color_eyre::eyre::{Result, WrapErr};
use tracing::instrument;
use serenity::{framework::StandardFramework, http::Http, model::id::UserId, Client};
use std::collections::HashSet;

#[instrument(skip(token))]
async fn get_bot_owners(token: &str) -> HashSet<UserId> {
    let http = Http::new_with_token(token);

    match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            owners
        }
        Err(e) => panic!("Could not access application info: {:?}", e),
    }
}

#[instrument(skip(token))]
async fn get_framework(token: &str, prefix: &str) -> StandardFramework {
    let owners = get_bot_owners(token).await;

    StandardFramework::new()
        .configure(|c| c.owners(owners).prefix(prefix))
        .group(&crate::commands::GENERAL_GROUP)
}

#[instrument(skip(token))]
pub async fn get_client(token: &str, prefix: &str) -> Result<Client> {
    Client::builder(token)
        .framework(get_framework(token, prefix).await)
        .event_handler(crate::handler::Handler)
        .await
        .context("Error creating the client.")
}
