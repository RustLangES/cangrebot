use serenity::{prelude::{EventHandler, Context}, async_trait, model::prelude::{interaction::{Interaction, InteractionResponseType}, Ready, GuildId, command::Command}};
use tracing::log::info;
use crate::slash_commands;


pub struct Handler(pub u64);


#[async_trait]
impl EventHandler for Handler {

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            // info!("Received command interaction: {:#?}", command);

            let content = match command.data.name.as_str() {
                "ping" => slash_commands::ping::run(&command.data.options),
                "invite" => slash_commands::invite::run(&command.data.options),
                "id" => slash_commands::id::run(&command.data.options),
                "attachmentinput" => slash_commands::attachmentinput::run(&command.data.options),
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

        let _commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| slash_commands::ping::register(command))
                .create_application_command(|command| slash_commands::id::register(command))
                .create_application_command(|command| slash_commands::invite::register(command))
                .create_application_command(|command| slash_commands::welcome::register(command))
                .create_application_command(|command| slash_commands::numberinput::register(command))
                .create_application_command(|command| slash_commands::attachmentinput::register(command))
        })
        .await;

        // info!("I now have the following guild slash commands: {:#?}", commands);

        let _guild_command = Command::create_global_application_command(&ctx.http, |command| {
            slash_commands::wonderful_command::register(command)
        })
        .await;

        // info!("I created the following global slash command: {:#?}", guild_command);
        
    }

}