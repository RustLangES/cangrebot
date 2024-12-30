use poise::serenity_prelude::CreateEmbed;
use poise::CreateReply;
use regex::Regex;
use reqwest::get;
use scraper::{Html, Selector};
use tracing::info;
use crate::bot;
use crate::bot::commands::krate::autocomplete;

fn strip_html(input: &str) -> String {
    Regex::new(r"</?[^>]+>")
        .unwrap()
        .replace_all(input, "")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .to_string()
}

#[poise::command(slash_command, prefix_command)]
pub async fn crate_docs(
    ctx: bot::Context<'_>,
    #[description = "El nombre del crate, crate@version | crate"]
    #[rename = "crate"]
    #[autocomplete = "autocomplete"]
    package: String,
    #[description = "Buscar un nombre o expresion minima, como Struct::method."]
    search: String
) -> Result<(), bot::Error> {

    let (name, version) = package
        .split_once("@")
        .unwrap_or((&package, "latest"));

    let response = get(format!("https://docs.rs/{name}/{version}/{name}/all.html"))
        .await?;

    if response.status() != 200 {
        ctx
            .say(format!("The crate `{package}` was not found."))
            .await
            .ok();

        return Ok(());
    }

    let html = response
        .text()
        .await?;

    let selector = Selector::parse("ul.all-items > li > a").unwrap();
    let elements = Html::parse_document(&html);
    let elements = elements.select(&selector);

    let Some((_, url)) = elements
        .into_iter()
        .map(|elem| (elem.inner_html(), elem.attr("href").unwrap()))
        .find(|(name, _)| name.to_lowercase().contains(&search.to_lowercase()))
    else {
        ctx
            .say(format!("The expression `{search}` was not found on `{package}`."))
            .await
            .ok();

        return Ok(());
    };

    let url = format!("https://docs.rs/{name}/{version}/{name}/{url}");
    let html = get(url.clone())
        .await?
        .text()
        .await?;

    let html = Html::parse_document(&html);

    let element_type_selector = Selector::parse(".main-heading > h1").unwrap();
    let element_name_selector = Selector::parse(".main-heading > h1 > span").unwrap();
    let element_code_selector = Selector::parse(".item-decl > code").unwrap();
    let element_description_selector = Selector::parse(".docblock > p:last-child").unwrap();

    let element_type = html.select(&element_type_selector)
        .into_iter()
        .next()
        .unwrap()
        .inner_html();
    let element_type = element_type
        .split_once(" ")
        .unwrap()
        .0;
    let element_name = html.select(&element_name_selector)
        .into_iter()
        .next()
        .unwrap()
        .inner_html();
    let element_name = element_name
        .split(" ")
        .last()
        .unwrap();
    let element_code = strip_html(&html.select(&element_code_selector)
        .into_iter()
        .next()
        .unwrap()
        .inner_html());
    let element_description = html.select(&element_description_selector)
        .into_iter()
        .next()
        .unwrap()
        .inner_html()
        .replace("<code>", "`")
        .replace("</code>", "`");

    ctx
        .send(
            CreateReply::default()
                .embed(
                    CreateEmbed::new()
                        .title(format!("{element_type} {element_name}"))
                        .description(format!(
                            "```rs\n{element_code}\n```\n{element_description}\n\nSee more: {url}"
                        ))
                )
        )
        .await?;

    Ok(())
}
