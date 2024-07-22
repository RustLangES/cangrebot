use std::sync::Arc;

use crate::{events::join::guild_member_addition, slash_commands};
use serenity::all::Message;
use serenity::{
    all::{CreateInteractionResponse, CreateInteractionResponseMessage},
    async_trait,
    model::prelude::{GuildId, Interaction, Member, Ready},
    prelude::{Context, EventHandler},
};
use tracing::{error, log::info};

use crate::events::anti_spam::{extract_link, spam_checker};
use crate::slash_commands::ping;
use slash_commands::crate_lib;
use slash_commands::explica::run as explica_run;
use slash_commands::id::run as id_run;
use slash_commands::invite::run as invite_run;
use slash_commands::ping::run as ping_run;
use slash_commands::sugerencia;

pub struct Handler {
    guild_id: u64,
    client: reqwest::Client,
}

impl Handler {
    pub fn new(guild_id: u64) -> Self {
        Self {
            guild_id,
            client: reqwest::ClientBuilder::new()
                .build()
                .expect("Cannot create reqwest client"),
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn guild_member_addition(&self, ctx: Context, member: Member) {
        guild_member_addition(&ctx, &member.guild_id, &member).await;
    }

    async fn message(&self, ctx: Context, new_message: Message) {
        if extract_link(&new_message.content).is_some() {
            let message_content = Arc::new(new_message.content.to_string());
            spam_checker(
                message_content,
                new_message.channel_id,
                &ctx,
                604800,
                &new_message,
                new_message.guild_id.unwrap(),
            )
            .await
            .unwrap_or_default();
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            // info!("Received command interaction: {:#?}", command);

            let content = match command.data.name.as_str() {
                "ping" => ping_run(),
                "invite" => invite_run(&command.data.options),
                "id" => id_run(&command.data.options()),
                "explica" => explica_run(&command.data.options),
                "sugerencia" => {
                    sugerencia::run(
                        &ctx,
                        &command.channel_id,
                        &command.data.options,
                        &command.user,
                    )
                    .await
                }
                "crate" => {
                    crate_lib::run(&self.client, &command.data.options).await
                }
                _ => "Este comando no esa implementado, pero puedes hacer una sugerencia `/sugerencia`".to_string(),
            };

            let data = CreateInteractionResponseMessage::new().content(content);
            let builder = CreateInteractionResponse::Message(data);
            if let Err(why) = command.create_response(&ctx.http, builder).await {
                info!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        let guild_id = GuildId::new(self.guild_id.into());

        if let Err(error) = guild_id
            .set_commands(
                &ctx.http,
                vec![
                    slash_commands::explica::register(),
                    ping::register(),
                    slash_commands::id::register(),
                    slash_commands::invite::register(),
                    sugerencia::register(),
                    crate_lib::register(),
                ],
            )
            .await
        {
            error!("Cannot create slash commands: {}", error);
        };
    }
}
