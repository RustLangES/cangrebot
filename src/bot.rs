use std::collections::HashSet;

use serenity::{Client, framework::StandardFramework, http::Http, model::id::UserId};

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

async fn get_framework(token: &str) -> StandardFramework {
    let owners = get_bot_owners(token).await;

    StandardFramework::new()
        .configure(|c| c.owners(owners).prefix("!"))
        .group(&crate::commands::GENERAL_GROUP)
}

pub async fn get_client() -> Client {
    let token = std::env::var("DISCORD_TOKEN").expect("Cannot get the `DISCORD_TOKEN` environment variable.");

    Client::builder(&token)
        .framework(get_framework(&token).await)
        .event_handler(crate::handler::Handler)
        .await
        .expect("Error creating the client.")
}
