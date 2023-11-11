use std::collections::HashMap;

use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::application_command::CommandDataOption;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::{ChannelId, Message, MessageId, ReactionType};
use serenity::model::user::User;
use serenity::prelude::{Context, Mentionable};

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("sugerencia")
        .description("Crea, Modifica y administra las sugerencias :D")
        .create_option(|o| {
            o.name("nueva")
                .description("Crea una sugerencia")
                .kind(CommandOptionType::String)
                .required(false)
                .min_length(50)
        })
        .create_option(|o| {
            o.name("implementada")
                .description("Marca una sugerencia como implementada")
                .kind(CommandOptionType::String)
                .required(false)
        })
        .create_option(|o| {
            o.name("cancelada")
                .description("Marca una sugerencia como cancelada o que no se realizara")
                .kind(CommandOptionType::String)
                .required(false)
        })
}

pub async fn run(
    ctx: &Context,
    channel_id: &ChannelId,
    options: &[CommandDataOption],
    message: &HashMap<MessageId, Message>,
    user: &User,
) -> String {
    let mut res = None;

    for o in options {
        res = match o.name.as_str() {
            "nueva" => Some(run_create(ctx, message, user).await),
            "implementada" => Some(run_implemented(ctx, channel_id, message, user).await),
            "cancelada" => Some(run_canceled(ctx, channel_id, message, user).await),
            _ => None,
        };
        if res.is_some() {
            break;
        }
    }

    res.unwrap_or("Subcomando invalido".to_string())
}

pub async fn run_create(
    ctx: &Context,
    message: &HashMap<MessageId, Message>,
    user: &User,
) -> String {
    let msg_channel = ChannelId(824695624665923594_u64);

    let msg = format!(
        "{} nos sugiere\n\n{}",
        user.mention(),
        message
            .iter()
            .map(|(_, m)| m.content.clone())
            .collect::<Vec<String>>()
            .join(" ")
    );
    let msg = msg_channel.say(&ctx, msg).await.unwrap();

    // Convert string emoji to ReactionType to allow custom emojis
    let check_reaction = ReactionType::Unicode("✅".to_string());
    let reject_reaction = ReactionType::Unicode("❌".to_string());
    msg.react(&ctx, check_reaction).await.unwrap();
    msg.react(&ctx, reject_reaction).await.unwrap();

    msg_channel
        .create_public_thread(ctx, msg.id, |t| {
            t.name("Comparte tu opinion!".to_string())
                .auto_archive_duration(4320)
        })
        .await
        .unwrap();

    "Sugerencia Creada".to_string()
}

pub async fn run_canceled(
    _ctx: &Context,
    _channel_id: &ChannelId,
    _message: &HashMap<MessageId, Message>,
    _user: &User,
) -> String {
    // "Sugerencia Marcada como **Cancelada**".to_string()
    "Esta caracteristica aun no se encuentra disponible".to_string()
}

pub async fn run_implemented(
    _ctx: &Context,
    _channel_id: &ChannelId,
    _message: &HashMap<MessageId, Message>,
    _user: &User,
) -> String { 
    // "Sugerencia Marcada como **Implementada**".to_string()
    "Esta caracteristica aun no se encuentra disponible".to_string()
}
