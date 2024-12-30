use poise::serenity_prelude::AutocompleteChoice;
use tracing::error;

use crate::bot;

mod fetch;

/// Busca y obtiene el enlace del crate, documentacion, repositorio y pagina web
#[poise::command(slash_command, prefix_command)]
pub async fn cargo(
    ctx: bot::Context<'_>,
    #[description = "El nombre del crate que quieres buscar"]
    #[autocomplete = "autocomplete"]
    nombre: String,
) -> Result<(), bot::Error> {
    let client = reqwest::Client::new();

    match fetch::fetch_crate(&client, nombre).await {
        Ok(s) => ctx.say(s).await?,
        Err(e) => ctx.say(e.to_string()).await?,
    };

    Ok(())
}

pub async fn autocomplete(
    _: bot::Context<'_>,
    partial: &str,
) -> impl Iterator<Item = AutocompleteChoice> {
    let client = reqwest::Client::new();

    let Ok(value) = fetch::search_crate(&client, partial)
        .await
        .inspect_err(|err| error!("{err}"))
    else {
        return vec![AutocompleteChoice::new("Error", "Error")].into_iter();
    };

    let Ok(value) = value else {
        return vec![AutocompleteChoice::new("Error", "Error")].into_iter();
    };

    let v: Vec<_> = value
        .into_iter()
        .map(|v| AutocompleteChoice::new(v.clone(), v))
        .collect();

    v.into_iter()
}
