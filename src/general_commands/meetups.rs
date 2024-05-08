use reqwest::Client;
use scraper::{Html, Selector};
use serenity::all::{CreateEmbed, CreateEmbedFooter};
use serenity::builder::CreateMessage;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{CommandError, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::Context;

#[command]
pub async fn meetups(ctx: &Context, msg: &Message) -> CommandResult {
    let meetups = vec![
        "Rust-MX".to_string(),
        "madrust".to_string(),
        "rust-argentina".to_string(),
        "bcnrust".to_string(),
        "rustlang-spain".to_string(),
        "rust-medellin".to_string(),
    ];
    for meetup in meetups {
        let meetup_future = async move {
            if let Ok(meetup_fields) = get_meetup(meetup.clone()).await {
                Ok(meetup_fields)
            } else {
                let Ok(_) = msg
                    .channel_id
                    .say(
                        &ctx.http,
                        format!("No hay eventos próximos para {}", meetup),
                    )
                    .await
                else {
                    return Err("No hay eventos próximos".to_string());
                };
                Err("No hay eventos próximos".to_string())
            }
        };
        let Ok(meetup) = meetup_future.await else {
            continue;
        };

        let link = meetup.1.clone();

        let detail_future = async move {
            get_detail(link)
                .await
                .expect("No se pudo obtener el detalle del evento")
        };
        let detail = detail_future.await;

        let location = meetup.2.clone() + " - " + &detail.2.clone();

        msg.channel_id.say(&ctx.http, "Meetup:").await?;

        let embed = CreateEmbed::new()
            .title(meetup.0.clone())
            .description(detail.0.clone())
            .image(detail.1.clone())
            .url(meetup.1.clone())
            .footer(CreateEmbedFooter::new(location));

        let _ = msg
            .channel_id
            .send_message(&ctx.http, CreateMessage::new().embed(embed))
            .await;
    }

    Ok(())
}

async fn get_meetup(
    group: String,
) -> Result<(String, String, String), Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::new();
    let url = format!("https://www.meetup.com/es-ES/{}/events/", group);
    let res = client.get(&url).send().await?;
    let body = res.text().await?;
    let document = Html::parse_document(&body);

    let events_selector = Selector::parse("#event-card-e-1").unwrap();

    let mut events = document.select(&events_selector);
    let Some(event) = events.next() else {
        return Err(CommandError::from("No hay eventos próximos"));
    };
    let event = event.value();
    let event_title_selector =
        scraper::Selector::parse("#event-card-e-1 span.ds-font-title-3").unwrap();
    let event_title = document.select(&event_title_selector).next().unwrap();
    let event_title = event_title.text().collect::<Vec<_>>().join(" ");
    let event_link = event.attr("href").unwrap();
    let event_link = event_link.to_string();

    let event_location_selector = Selector::parse("#event-card-e-1 .d1hetqt0 .text-gray6").unwrap();
    let event_location = document.select(&event_location_selector).next().unwrap();
    let location = event_location.text().collect::<Vec<_>>().join(" ");

    Ok((event_title, event_link.to_string(), location))
}

async fn get_detail(link: String) -> Result<(String, String, String), Box<dyn std::error::Error>> {
    let client2 = Client::new();
    let res2 = client2.get(&link).send().await?;
    let body = res2.text().await?;
    let document2 = Html::parse_document(&body);

    let image_selector = scraper::Selector::parse("meta[property=\"og:image\"]").unwrap();
    let image = document2
        .select(&image_selector)
        .next()
        .unwrap()
        .value()
        .clone();
    let image = image.attr("content").unwrap();

    let description = document2
        .select(&scraper::Selector::parse("#event-details > div.break-words").unwrap())
        .next()
        .unwrap();
    let description = description.text().collect::<Vec<_>>().join(" ");
    let mut description = description[0..280].to_string();
    description.push('…');

    let location_div = document2
        .select(&scraper::Selector::parse("#event-info div.overflow-hidden div").unwrap())
        .next()
        .unwrap();

    let location = location_div.text().collect::<Vec<_>>().join(" ");

    Ok((description, image.to_string(), location))
}
