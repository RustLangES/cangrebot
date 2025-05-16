use crate::serenity::all::AutocompleteChoice;

use crate::bot;
use std::path::PathBuf;

const TOPICS: [&str; 33] = [
    "arrays",
    "borrowing",
    "closures",
    "condicionales",
    "constantes",
    "enums",
    "for",
    "funciones",
    "generics",
    "if_let",
    "iterators",
    "let_else",
    "lifetimes",
    "loop",
    "macros",
    "match",
    "metodos",
    "modulos",
    "operadores",
    "ownership",
    "result",
    "return",
    "scopes",
    "shadowing",
    "slices",
    "string",
    "struct",
    "tipo_de_datos",
    "traits",
    "tuplas",
    "variables",
    "vectores",
    "while",
];

/// Explica un concepto de Rust
#[poise::command(slash_command, prefix_command)]
pub async fn explain(
    ctx: bot::Context<'_>,
    #[description = "Este sera el concepto que se explicara"]
    #[autocomplete = "autocomplete"]
    concepto: String,
) -> Result<(), bot::Error> {
    let concepts_folder = PathBuf::from(format!(
        "{}/rust-examples/docs",
        std::env::var("STATIC_ROOT")
            .as_deref()
            .unwrap_or_else(|_| "static")
    ));

    let concept = concepto.to_lowercase() + ".md";
    let concept = std::fs::read_to_string(concepts_folder.join(concept))
        .unwrap_or("No se ha encontrado el concepto".to_string());

    ctx.say(concept).await?;

    Ok(())
}

async fn autocomplete(
    _: bot::Context<'_>,
    partial: &str,
) -> impl Iterator<Item = AutocompleteChoice> {
    TOPICS
        .iter()
        .filter(|topic| topic.contains(partial))
        .map(|topic| AutocompleteChoice::new(*topic, *topic))
        .collect::<Vec<_>>()
        .into_iter()
}
