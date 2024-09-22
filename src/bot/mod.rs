mod commands;
mod util;

use anyhow::anyhow;
use poise::serenity_prelude::GuildId;
use tracing::info;

use crate::{serenity, CangrebotSecrets};

use commands::commands;

pub(super) struct Data {
    secrets: CangrebotSecrets,
}

pub(super) type Error = Box<dyn std::error::Error + Send + Sync>;
pub(super) type Context<'a> = poise::Context<'a, Data, Error>;

pub async fn setup(secrets: &CangrebotSecrets) -> Result<serenity::Client, anyhow::Error> {
    let guild_id = GuildId::new(secrets.guild_id);

    let intents = serenity::GatewayIntents::all();

    let data_secrets = secrets.clone();
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands(),

            pre_command: |ctx| {
                Box::pin(async move {
                    info!(
                        "Got command '{}' by user '{}'",
                        ctx.command().qualified_name,
                        ctx.author().name
                    );
                })
            },
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some(secrets.discord_prefix.clone()),
                mention_as_prefix: true,
                ignore_bots: true,
                ..Default::default()
            },

            ..Default::default()
        })
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                let commands = &framework.options().commands;
                poise::builtins::register_globally(ctx, commands).await?;
                poise::builtins::register_in_guild(ctx, commands, guild_id).await?;
                Ok(Data {
                    secrets: data_secrets,
                })
            })
        })
        .build();

    serenity::ClientBuilder::new(&secrets.discord_token, intents)
        .framework(framework)
        .await
        .map_err(|err| anyhow!("Error crating client: {err}"))
}
