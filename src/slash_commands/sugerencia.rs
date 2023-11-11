use serenity::builder::CreateApplicationCommand;
use serenity::json::Value;
use serenity::model::prelude::application_command::CommandDataOption;
use serenity::model::prelude::command::{CommandOptionType, CommandType};
use serenity::model::prelude::{ChannelId, ReactionType};
use serenity::model::user::User;
use serenity::prelude::{Context, Mentionable};
use tracing::info;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("sugerencia")
        .description("Crea, Modifica y administra las sugerencias :D")
        .kind(CommandType::ChatInput)
        .create_option(|o| {
            o.name("nueva")
                .description("Crea una sugerencia")
                .required(false)
                .kind(CommandOptionType::SubCommand)
                .create_sub_option(|o| {
                    o.name("titulo")
                        .required(true)
                        .min_length(10)
                        .description("Agrega un Titulo a tu sugerencia")
                        .kind(CommandOptionType::String)
                })
                .create_sub_option(|o| {
                    o.name("contenido")
                        .required(true)
                        .min_length(50)
                        .description("Cuentanos acerca de tu sugerencia")
                        .kind(CommandOptionType::String)
                })
        })
        .create_option(|o| {
            o.name("implementada")
                .description("Marca una sugerencia como implementada")
                .kind(CommandOptionType::SubCommand)
                .required(false)
        })
        .create_option(|o| {
            o.name("cancelada")
                .description("Marca una sugerencia como cancelada o que no se realizara")
                .kind(CommandOptionType::SubCommand)
                .required(false)
        })
}

pub async fn run(
    ctx: &Context,
    channel_id: &ChannelId,
    options: &[CommandDataOption],
    user: &User,
) -> String {
    let mut res = None;

    for o in options {
        res = match o.name.as_str() {
            "nueva" => Some(run_create(ctx, &o.options, user).await),
            "implementada" => Some(run_implemented(ctx, channel_id, o.value.as_ref(), user).await),
            "cancelada" => Some(run_canceled(ctx, channel_id, o.value.as_ref(), user).await),
            _ => None,
        };
        if res.is_some() {
            break;
        }
    }

    res.unwrap_or("Subcomando invalido".to_string())
}

pub async fn run_create(ctx: &Context, options: &[CommandDataOption], user: &User) -> String {
    info!("Running create suggestion");
    let msg_channel = ChannelId(824695624665923594_u64);
    let mut name = String::from("Comparte tu opinion!");
    let mut content = String::from("<Hubo un Error>");

    for opt in options {
        match opt.name.as_str() {
            "titulo" => {
                if let Some(value) = opt.value.as_ref() {
                    name = value.as_str().map(|n| n.to_string()).unwrap();
                }
            }
            "contenido" => {
                if let Some(value) = opt.value.as_ref() {
                    content = value.as_str().map(|n| n.to_string()).unwrap();
                }
            }
            _ => {}
        }
    }

    let msg = format!("{} nos sugiere\n\n{content}", user.mention(),);
    let msg = msg_channel.say(&ctx, msg).await.unwrap();

    // Convert string emoji to ReactionType to allow custom emojis
    let check_reaction = ReactionType::Unicode("✅".to_string());
    let reject_reaction = ReactionType::Unicode("❌".to_string());
    msg.react(&ctx, check_reaction).await.unwrap();
    msg.react(&ctx, reject_reaction).await.unwrap();

    msg_channel
        .create_public_thread(ctx, msg.id, |t| {
            t.name(name.to_string()).auto_archive_duration(4320)
        })
        .await
        .unwrap();

    "Sugerencia Creada".to_string()
}

pub async fn run_canceled(
    _ctx: &Context,
    _channel_id: &ChannelId,
    _message: Option<&Value>,
    _user: &User,
) -> String {
    // "Sugerencia Marcada como **Cancelada**".to_string()
    "Esta caracteristica aun no se encuentra disponible".to_string()
}

pub async fn run_implemented(
    _ctx: &Context,
    _channel_id: &ChannelId,
    _message: Option<&Value>,
    _user: &User,
) -> String {
    // "Sugerencia Marcada como **Implementada**".to_string()
    "Esta caracteristica aun no se encuentra disponible".to_string()
}
