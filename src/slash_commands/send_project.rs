use serenity::all::{
    ChannelId, CommandDataOption, CommandDataOptionValue, CommandOptionType, Context,
    CreateCommand, CreateCommandOption, Mention, Mentionable,
};

#[derive(Default)]
struct Project {
    name: Vec<String>,
    description: String,
    link: String,
    brand_src: String,
    button_link: String,
    button_text: String,
    brand_as_letter: bool,
    button_bg_color: String,
}

async fn send_succes_message(ctx: &Context, channel_id: &ChannelId, user: Mention, pr: String) {
    channel_id.say(ctx, format!("Se ha creado una Pull Request para agregar este proyecto en la pagina web de RustLangES.\n\n> {user} si deseas modificar o agregar detalles de tu proyecto, porfavor hazlo desde el siguiente enlace https://github.com/RustLangES/proyectos-comunitarios/pull/{pr}")).await.unwrap()
}

pub async fn run(ctx: &Context, channel_id: &ChannelId, options: &[CommandDataOption]) -> String {
    let mut project = Project {
        brand_as_letter: true,
        button_bg_color: "white".to_owned(),
        ..Default::default()
    };
    let user;

    for o in options {
        match o.name.as_str() {
            "autor" => {
                if let CommandDataOptionValue::User(user_id) = o.value {
                    user = user_id;
                }
            }
            "nombre" => {
                if let CommandDataOptionValue::String(v) = &o.value {
                    project.name = v.split(' ').collect();
                    project.brand_src = v.chars().next().unwrap_or('R').to_string();
                }
            }
            "descripcion" => {
                if let CommandDataOptionValue::String(v) = &o.value {
                    project.description = v.to_owned();
                }
            }
            "enlace" => {
                if let CommandDataOptionValue::String(v) = &o.value {
                    project.link = v.to_owned();
                    project.button_link = v.to_owned();
                }
            }
            "texto" => {
                if let CommandDataOptionValue::String(v) = &o.value {
                    project.button_text = v.to_owned();
                }
            }
            "background" => {
                if let CommandDataOptionValue::String(v) = &o.value {
                    project.button_bg_color = v.to_owned();
                }
            }
            _ => continue,
        };
    }

    // Extract empty values from thread
    send_succes_message(ctx, channel_id, user.mention(), "14".to_owned()).await;

    "Proyecto enviado correctamente".to_owned()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("showcase")
        .description("Publica el proyecto en el repositorio de showcase")
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "autor", "Autor del proyecto")
                .kind(CommandOptionType::User)
                .required(true),
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "nombre", "Nombre del proyecto")
                .kind(CommandOptionType::String)
                .required(false),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "descripcion",
                "Descripcion del proyecto",
            )
            .required(true),
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "enlace", "Enlace del proyecto")
                .required(false),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "texto",
                "Texto en el boton del enlace proyecto",
            )
            .required(true),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "background",
                "Color de fondo para el boton del enlace proyecto",
            )
            .required(false),
        )
}
