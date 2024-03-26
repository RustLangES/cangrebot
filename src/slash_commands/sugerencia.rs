use serenity::all::{CommandDataOption, CommandDataOptionValue, CommandOptionType, CommandType, CreateCommand, CreateCommandOption, CreateThread};

use serenity::model::prelude::{ChannelId, ReactionType};
use serenity::model::user::User;
use serenity::prelude::{Context, Mentionable};
use tracing::info;

pub fn register() -> CreateCommand {
    CreateCommand::new("sugerencia")
        .description("Crea, Modifica y administra las sugerencias :D")
        .kind(CommandType::ChatInput)
        .add_option(
            CreateCommandOption::new(CommandOptionType::SubCommand, "nueva", "Crea una sugerencia")
                .required(false)
                .add_sub_option(
                    CreateCommandOption::new(CommandOptionType::String, "titulo", "Agrega un Titulo a tu sugerencia")
                        .required(true)
                        .min_length(10)
                )
                .add_sub_option(
                    CreateCommandOption::new(CommandOptionType::String, "contenido", "Cuentanos acerca de tu sugerencia")
                        .required(true)
                        .min_length(50)
                )
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::SubCommand, "implementada", "Marca una sugerencia como implementada")
                .required(false)
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::SubCommand, "cancelada", "Marca una sugerencia como cancelada o que no se realizara")
                .required(false)
        )
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
            "nueva" => Some(run_create(ctx, o.value.clone(), user).await),
            "implementada" => Some(run_implemented(ctx, channel_id, o.value.clone(), user).await),
            "cancelada" => Some(run_canceled(ctx, channel_id, o.value.clone(), user).await),
            _ => None,
        };
        if res.is_some() {
            break;
        }
    }

    res.unwrap_or("Subcomando invalido".to_string())
}

pub async fn run_create(ctx: &Context, options: CommandDataOptionValue, user: &User) -> String {
    let CommandDataOptionValue::SubCommand(subcommand) = options else {
        return "error".to_string();
    };

    info!("Running create suggestion");
    let msg_channel = ChannelId::new(824695624665923594_u64);
    let mut name = String::from("Comparte tu opinion!");
    let mut content = String::from("<Hubo un Error>");

    for opt in subcommand {
        match opt.name.as_str() {
            "titulo" => {
                if let CommandDataOptionValue::String(value) = opt.value {
                    name = value;
                }
            }
            "contenido" => {
                if let CommandDataOptionValue::String(value) = opt.value {
                    content = value;
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

    let builder = CreateThread::new(name.to_string()).auto_archive_duration(4320.into());
    msg_channel
        .create_thread(ctx, builder)
        .await
        .unwrap();

    "Sugerencia Creada".to_string()
}

pub async fn run_canceled(
    _ctx: &Context,
    _channel_id: &ChannelId,
    _message: CommandDataOptionValue,
    _user: &User,
) -> String {
    // "Sugerencia Marcada como **Cancelada**".to_string()
    "Esta caracteristica aun no se encuentra disponible".to_string()
}

pub async fn run_implemented(
    _ctx: &Context,
    _channel_id: &ChannelId,
    _message: CommandDataOptionValue,
    _user: &User,
) -> String {
    // "Sugerencia Marcada como **Implementada**".to_string()
    "Esta caracteristica aun no se encuentra disponible".to_string()
}
