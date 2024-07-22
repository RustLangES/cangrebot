use reqwest::Client;
use serde::Deserialize;
use serenity::all::{
    CommandDataOption, CommandDataOptionValue, CommandOptionType, CommandType, CreateCommand,
    CreateCommandOption,
};

use tracing::info;

use crate::utils::{mask_url, ToSnakeCase};

const MESSAGE: &str = r#"## [@crate_name@](https://crates.io/crates/@crate_name@)
@description@@keywords@

@last_version@@stable_version@
@doc@
@repo@
@web@"#;

enum TypeSearch {
    Get,
    Docs,
}

#[derive(Deserialize)]
struct CratesIO {
    #[serde(rename = "crate")]
    krate: CratesDetails,
}

#[derive(Deserialize)]
struct CratesDetails {
    name: String,
    description: String,
    max_stable_version: String,
    newest_version: String,
    keywords: Vec<String>,
    homepage: Option<String>,
    repository: Option<String>,
    documentation: Option<String>,
}

#[derive(Deserialize, Debug)]
struct CratesError {
    errors: Vec<CratesErrorDetail>,
}

#[derive(Deserialize, Debug)]
struct CratesErrorDetail {
    detail: String,
}

impl From<&str> for TypeSearch {
    fn from(value: &str) -> Self {
        match value {
            "docs" | "doc" => Self::Docs,
            _ => Self::Get,
        }
    }
}

pub fn register() -> CreateCommand {
    const COMMANDS: [(&str, &str); 2] = [
        (
            "get",
            "Busca y obtiene el enlace del crate, documentacion, repositorio y pagina web",
        ),
        ("docs", "Busca y obtiene el enlace de la Documentacion"),
    ];
    let mut cmd = CreateCommand::new("crate")
        .description("Obten o busca informacion de un crate")
        .kind(CommandType::ChatInput);

    for (name, desc) in COMMANDS {
        cmd = cmd.add_option(
            CreateCommandOption::new(CommandOptionType::SubCommand, name, desc)
                .required(false)
                .add_sub_option(
                    CreateCommandOption::new(
                        CommandOptionType::String,
                        "nombre",
                        "el nombre del crate que quieres buscar",
                    )
                    .required(true)
                    .min_length(1),
                ),
        );
    }

    cmd
}

pub async fn run(client: &Client, options: &[CommandDataOption]) -> String {
    let Some(option) = options
        .iter()
        .find(|o| o.kind() == CommandOptionType::SubCommand)
    else {
        return "Subcomando invalido".to_owned();
    };

    match fetch_crate(
        client,
        TypeSearch::from(option.name.as_str()),
        &option.value,
    )
    .await
    {
        Ok(v) => v,
        Err(e) => e.to_string(),
    }
}

fn maskurl(u: String) -> String {
    let masked = mask_url(&u);
    format!("[{masked}]({u})")
}

async fn fetch_crate(
    client: &Client,
    search_type: TypeSearch,
    options: &CommandDataOptionValue,
) -> reqwest::Result<String> {
    let CommandDataOptionValue::SubCommand(subcommand) = options else {
        return Ok("The option is not a subcommand".to_owned());
    };

    info!("Running crate searching");
    let Some(crate_name) = subcommand.iter().find_map(|opt| {
        if opt.name == "nombre" {
            if let CommandDataOptionValue::String(value) = &opt.value {
                return Some(value.clone());
            }
        }
        None
    }) else {
        return Ok("Tienes que indicar el `nombre` del crate".to_owned());
    };

    let res = client
        .get(format!(
            "https://crates.io/api/v1/crates/{}",
            crate_name.to_snake_case()
        ))
        .header("User-Agent", "RustLangES Automation Agent")
        .send()
        .await?
        .text()
        .await?;

    if let Ok(err) = serde_json::from_str::<CratesError>(&res) {
        if let Some(CratesErrorDetail { detail }) = err.errors.first() {
            return Ok(format!("{detail}"));
        }
    }

    let Ok(CratesIO {
        krate:
            CratesDetails {
                description,
                documentation,
                keywords,
                homepage,
                max_stable_version,
                name,
                newest_version,
                repository,
            },
    }) = serde_json::from_str::<CratesIO>(&res)
        .inspect_err(|e| tracing::error!("Serde Crates.io: {e:?}"))
    else {
        return Ok(format!("No se pudo deserializar la respuesta"));
    };

    let res = MESSAGE
        .replace("@crate_name@", &name)
        .replace("@description@", &description)
        .replace(
            "@last_version@",
            &format!("Última Version: ``{newest_version}``"),
        )
        .replace(
            "@stable_version@",
            &(if max_stable_version == newest_version {
                String::new()
            } else {
                format!("\nÚltima Version Estable: ``{max_stable_version}``")
            }),
        )
        .replace(
            "@keywords@",
            &(if keywords.is_empty() {
                String::new()
            } else {
                format!("\n-# {}", keywords.join(" | "))
            }),
        );

    let res = match search_type {
        TypeSearch::Get => res
            .replace(
                "@doc@",
                &format!(
                    "Documentación: {}",
                    documentation.map(maskurl).as_deref().unwrap_or("None")
                ),
            )
            .replace(
                "@repo@",
                &format!(
                    "Repositorio: {}",
                    repository.map(maskurl).as_deref().unwrap_or("None")
                ),
            )
            .replace(
                "@web@",
                &format!(
                    "Página Web: {}",
                    homepage.map(maskurl).as_deref().unwrap_or("None")
                ),
            ),
        TypeSearch::Docs => res
            .replace(
                "@doc@",
                &format!(
                    "Documentación: {}",
                    documentation.map(maskurl).as_deref().unwrap_or("None")
                ),
            )
            .replace("@repo@", "")
            .replace("@web@", ""),
    };

    Ok(res)
}
