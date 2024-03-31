use std::net::SocketAddr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::events::daily_challenge::{run_daily_challenge, DailyChallengeRequest};
use crate::{events::join::guild_member_addition, slash_commands};
use serenity::{
    all::{Command, CreateInteractionResponse, CreateInteractionResponseMessage},
    async_trait,
    model::prelude::{GuildId, Interaction, Member, Ready},
    prelude::{Context, EventHandler},
};
use tiny_http::Method;
use tracing::{error, log::info};

use crate::slash_commands::ping;
use slash_commands::attachmentinput::run as attachmentinput_run;
use slash_commands::explica::run as explica_run;
use slash_commands::id::run as id_run;
use slash_commands::invite::run as invite_run;
use slash_commands::ping::run as ping_run;
use slash_commands::sugerencia;

pub struct Handler {
    guild_id: u64,
    http_running: AtomicBool,
}

impl Handler {
    pub fn new(guild_id: u64) -> Self {
        Self {
            guild_id,
            http_running: AtomicBool::new(false),
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn guild_member_addition(&self, ctx: Context, member: Member) {
        guild_member_addition(&ctx, &member.guild_id, &member).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            // info!("Received command interaction: {:#?}", command);

            let content = match command.data.name.as_str() {
                "ping" => ping_run(),
                "invite" => invite_run(&command.data.options),
                "id" => id_run(&command.data.options()),
                "attachmentinput" => attachmentinput_run(&command.data.options()),
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
                _ => "not implemented :(".to_string(),
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
                    slash_commands::welcome::register(),
                    slash_commands::attachmentinput::register(),
                    sugerencia::register(),
                ],
            )
            .await
        {
            error!("Cannot create slash commands: {}", error);
        };

        // info!("I now have the following guild slash commands: {:#?}", commands);

        let _guild_command = Command::create_global_command(
            &ctx.http,
            slash_commands::wonderful_command::register(),
        )
        .await;

        let ctx = Arc::new(ctx);

        if !self.http_running.load(Ordering::Relaxed) {
            let ctx1 = Arc::clone(&ctx);

            // start http server
            tokio::spawn(async move {
                let server = tiny_http::Server::http("0.0.0.0:1337").unwrap();

                tracing::debug!("Listening on {:?}", server.server_addr());

                while let Ok(mut req) = server.recv() {
                    println!("Request: {req:?}");
                    // Ejecuta el servidor
                    match (req.method(), req.url()) {
                        (Method::Post, "/daily_challenge") => {
                            let reader = req.as_reader();
                            let Ok(data) = serde_json::from_reader(reader) else {
                                tracing::error!("Failed load json from reader");
                                continue;
                            };
                            match run_daily_challenge(&ctx1, &data).await {
                                Ok(()) => {
                                    tracing::debug!("Success send daily");
                                    let r = req.respond(tiny_http::Response::empty(200));
                                    tracing::debug!("Response sended: {r:?}");
                                }
                                Err(e) => {
                                    tracing::error!("Cannot send daily: {e:?}");
                                    let r = req.respond(tiny_http::Response::from_string(e.to_string()).with_status_code(400));
                                    tracing::debug!("Response sended: {r:?}");
                                }
                            }
                        }
                        _ => {}
                    }
                }
            });

            // Now that the loop is running, we set the bool to true
            self.http_running.swap(true, Ordering::Relaxed);
        }

        // info!("I created the following global slash command: {:#?}", guild_command);
    }
}
