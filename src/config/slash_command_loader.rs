use serenity::{prelude::{EventHandler, Context}, async_trait, model::prelude::{interaction::{Interaction, InteractionResponseType}, Ready, GuildId, command::Command, Member}};
use tracing::{log::info, error};
use crate::{slash_commands, events::join::guild_member_addition};

use slash_commands::ping::{register as ping_register, run as ping_run};
use slash_commands::invite::{register as invite_register, run as invite_run};
use slash_commands::welcome::register as welcome_register;
use slash_commands::id::{register as id_register, run as id_run};
use slash_commands::attachmentinput::{register as attachmentinput_register, run as attachmentinput_run};
use slash_commands::explica::{register as explica_register, run as explica_run};

pub struct Handler(pub u64);


#[async_trait]
impl EventHandler for Handler {

    async fn guild_member_addition(&self, ctx: Context, member: Member) {
        guild_member_addition(&ctx, &member.guild_id, &member).await; 
    }


    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            // info!("Received command interaction: {:#?}", command);

            let content = match command.data.name.as_str() {
                "ping" => ping_run(&command.data.options),
                "invite" => invite_run(&command.data.options),
                "id" => id_run(&command.data.options),
                "attachmentinput" => attachmentinput_run(&command.data.options),
                "explica" => explica_run(&command.data.options),
                _ => "not implemented :(".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                info!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        let guild_id = GuildId(self.0);

        if let Err(error) = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| explica_register(command))
                .create_application_command(|command| ping_register(command))
                .create_application_command(|command| id_register(command))
                .create_application_command(|command| invite_register(command))
                .create_application_command(|command| welcome_register(command))
                .create_application_command(|command| attachmentinput_register(command))
        }).await {
            error!("Cannot create slash commands: {}", error);
        };

        // info!("I now have the following guild slash commands: {:#?}", commands);

        let _guild_command = Command::create_global_application_command(&ctx.http, |command| {
            slash_commands::wonderful_command::register(command)
        })
        .await;

        // info!("I created the following global slash command: {:#?}", guild_command);
        
    }

}