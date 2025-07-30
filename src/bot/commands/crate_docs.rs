use crate::bot;
use crate::bot::commands::krate::autocomplete;
use poise::serenity_prelude::CreateEmbed;
use poise::CreateReply;
use reqwest::StatusCode;

static SEARCH: [&str; 7] = [
    "primitive",
    "derive",
    "trait",
    "struct",
    "fn",
    "keyword",
    "macro",
];

static OFFICIAL_CRATES: [&str; 5] = ["core", "std", "alloc", "proc_macro", "test"];

#[poise::command(broadcast_typing, slash_command)]
pub async fn docs(
    ctx: bot::Context<'_>,
    #[description = "Buscar un nombre o expresion minima, como Struct::method."] query: String,
    #[description = "El nombre del crate, crate@version | crate"]
    #[rename = "crate"]
    #[autocomplete = "autocomplete"]
    package: Option<String>,
) -> Result<(), bot::Error> {
    let msg = ctx
        .send(
            CreateReply::default().embed(
                CreateEmbed::new()
                    .title("Searching")
                    .description("Please wait..."),
            ),
        )
        .await?;

    let package = package.unwrap_or("std".to_string());

    let mut package_url = package.clone();

    if let Some((name, version)) = package.split_once('@') {
        package_url = format!("{name}/{version}/{name}");
    } else if !OFFICIAL_CRATES.contains(&package.as_str()) {
        package_url = format!("{package}/latest/{package}");
    }

    let mut repo = if OFFICIAL_CRATES.contains(&package.as_str()) {
        "https://doc.rust-lang.org/".to_owned()
    } else {
        "https://docs.rs/".to_owned()
    };

    repo = format!("{repo}{package_url}/");

    let mut results: Vec<String> = vec![];
    for search_term in SEARCH {
        let url = format!("{repo}{search_term}.{query}.html");
        let status = reqwest::get(&url).await?.status();
        if status == StatusCode::OK {
            results.push(url);
        }
    }

    if results.is_empty() {
        msg.edit(
            ctx,
            CreateReply::default().embed(
                CreateEmbed::new()
                    .title("Not found")
                    .description(format!("{query} was not found in {package}."))
                    .color(0x00FF_0000),
            ),
        )
        .await?;
        return Ok(());
    }

    msg.edit(
        ctx,
        CreateReply::default().embed(
            CreateEmbed::new()
                .title("Found")
                .description(format!(
                    "Results for {query} in {package}:\n- {}",
                    results.join("\n- ")
                ))
                .color(0x0000_FF00),
        ),
    )
    .await?;

    Ok(())
}
