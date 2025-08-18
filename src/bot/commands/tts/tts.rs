use crate::bot;
use poise::serenity_prelude::futures::future::join_all;
use poise::serenity_prelude::{self, CreateEmbed, GuildId, Http};
use poise::CreateReply;
use regex::{Captures, Regex};
use reqwest::Client;
use songbird::input::HttpRequest;
use songbird::tracks::Track;
use std::borrow::Cow;
use std::sync::Arc;
use urlencoding::encode;

macro_rules! replace_patterns  {
    ($text:expr, [ $( ($re:expr, |$caps:ident| $body:expr) ),* $(,)? ]) => {{
        let mut result = $text.to_string();
        $(
            let re = Regex::new($re).expect("regex inválido");
            result = re.replace_all(&result, |$caps: &Captures| -> Cow<str> {
                $body
            }).into_owned();
        )*
        result
    }};
}

pub async fn replace_mentions(
    guild_id: GuildId,
    http: Arc<Http>,
    raw_text: &str,
) -> Result<String, serenity_prelude::Error> {
    let mention_re = Regex::new(r"<@(\d+)>").unwrap();
    let mut resolved = String::with_capacity(raw_text.len());
    let mut last_end = 0;

    let matches: Vec<(std::ops::Range<usize>, u64)> = mention_re
        .captures_iter(raw_text)
        .filter_map(|caps| {
            let m = caps.get(0)?;
            let id_str = caps.get(1)?.as_str();
            let id = id_str.parse::<u64>().ok()?;
            Some((m.range(), id))
        })
        .collect();

    let futures = matches.iter().map(|(_, id)| {
        let http = http.clone();
        async move {
            let member = guild_id.member(&http, *id).await?;
            Ok::<_, serenity_prelude::Error>(member.display_name().to_string())
        }
    });

    let display_names = join_all(futures).await;

    for ((range, _), name_result) in matches.into_iter().zip(display_names) {
        let nick = name_result?;
        resolved.push_str(&raw_text[last_end..range.start]);
        resolved.push_str(&nick);
        last_end = range.end;
    }

    resolved.push_str(&raw_text[last_end..]);
    Ok(resolved)
}

pub fn split_text(s: &str, max_chars: usize) -> Vec<String> {
    s.split_whitespace().fold(Vec::new(), |mut acc, word| {
        if let Some(last) = acc.last_mut() {
            if last.len() + 1 + word.len() <= max_chars {
                last.push(' ');
                last.push_str(word);
            } else {
                acc.push(word.to_string());
            }
        } else {
            acc.push(word.to_string());
        }
        acc
    })
}

#[poise::command(slash_command, prefix_command, guild_only)]
pub async fn tts(ctx: bot::Context<'_>, #[rest] text: String) -> Result<(), bot::Error> {
    let guild_id = ctx.guild_id().ok_or(".")?;
    let http = ctx.serenity_context().http.clone();

    let manager = songbird::get(ctx.serenity_context())
        .await
        .ok_or("No se pudo obtener el manager de voz")?
        .clone();

    let Some(handler_lock) = manager.get(guild_id) else {
        ctx.send(
            CreateReply::default().embed(
                CreateEmbed::new()
                    .title("Error")
                    .description("No estoy en ningún canal de voz. Usa /join primero.")
                    .color(0x00FF_0000),
            ),
        )
        .await?;
        return Ok(());
    };

    let raw_text = format!("{} dice: {}", ctx.author().display_name(), &text);

    let resolved = replace_mentions(guild_id, http.clone(), &raw_text).await?;

    let cleaned = replace_patterns!(
        resolved,
        [
            (
                r"https?://(?:www\.)?[-a-zA-Z0-9@%._+~#=]{2,256}\.[a-z]{2,6}(?:[-a-zA-Z0-9@:%_+.~#?&/=]*)?",
                |_caps| Cow::Borrowed("enlace")
            ),
            (r"<:([a-zA-Z0-9_]+):\d+>", |caps| Cow::Owned(
                (caps[1]).to_string()
            )),
        ]
    );

    let client = Client::new();
    for c in split_text(&cleaned, 200) {
        let url = format!(
            "https://translate.google.com/translate_tts?client=tw-ob&tl=es&q={}",
            encode(&c)
        );
        let data = HttpRequest::new(client.clone(), url).clone();
        let t: Track = Track::new_with_data(data.into(), Arc::new(ctx.author().id));
        handler_lock.lock().await.enqueue(t).await;
    }

    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::new()
                .title("TTS")
                .description(format!("Reproduciendo: {text}"))
                .color(0x0000_FF00),
        ),
    )
    .await?;

    Ok(())
}
