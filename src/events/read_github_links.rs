use regex::{Captures, Regex};
use reqwest::get;
use serenity::all::{Context, EventHandler, Message};
use serenity::async_trait;
use std::option::Option;

pub struct ReadGithubLinkHandler;

pub enum RangeOrIndex {
    Language(String),
    Index(String, i32),
    Range(String, i32, i32)
}

pub fn parse_url(url: &str) -> Option<RangeOrIndex> {
    let extension_regex = Regex::new(r"\.([^./?#]+)(#|$)").unwrap();

    let range_regex
        = Regex::new(r"(?:\.(?<language>[^#]+))?#L(?<start>\d+)?(?:-L(?<end>\d+))?$")
        .unwrap();

    let language = extension_regex
        .captures(url)
        .and_then(|caps| caps.get(1))
        .map(|m| m.as_str().to_string())
        .unwrap_or_default();

    if let Some(caps) = range_regex.captures(url) {
        let start = caps.name("start").and_then(|m| m.as_str().parse::<i32>().ok());
        let end = caps.name("end").and_then(|m| m.as_str().parse::<i32>().ok());

        if end < start {
            return None;
        }

        match (start, end) {
            (Some(start), Some(end)) => Some(RangeOrIndex::Range(
                language,
                start - 1,
                end
            )),
            (Some(start), None) => Some(RangeOrIndex::Index(
                language,
                start - 1
            )),
            (None, None) => {
                Some(RangeOrIndex::Language(language))
            }
            _ => None
        }
    } else {
        None
    }
}


async fn read_message(link: String) -> Option<String> {
    if let Ok(result) = get(&link).await {
        if result.status() == 200 {
            if let Ok(text) = result.text().await {
                let parsed = parse_url(&link)?;

                let subtext: Vec<&str> = text.split('\n').collect();

                return match parsed {
                    RangeOrIndex::Language(language)
                    => Some(format!(
                        "Mostrando <{link}>\n```{language}\n{text}\n```"
                    )),
                    RangeOrIndex::Index(language, index)
                    => {
                        if index < subtext.len() as i32 {
                            Some(format!(
                                "Mostrando linea {} de <{link}>\n```{language}\n{}\n```",
                                index + 1,
                                subtext[index as usize].to_string())
                            )
                        } else {
                            None
                        }
                    }
                    RangeOrIndex::Range(language, start, end)
                    => {
                        if start  < subtext.len() as i32 && end <= subtext.len() as i32 {
                            Some(format!(
                                "Mostrando desde la linea {} hasta la linea {end} de <{link}>\n```{language}\n{}\n```",
                                start + 1,
                                subtext[start as usize..end as usize].join("\n")
                            ))
                        } else {
                            None
                        }
                    }
                };
            }
        }
    }
    None
}

#[async_trait]
impl EventHandler for ReadGithubLinkHandler {
    async fn message(&self, ctx: Context, msg: Message) {
        let repo_regex
            = Regex::new(r"(https://github\.com/(?:[^/]+/){2})blob/(.*)")
            .unwrap();
        let hidden_link_regex
            = Regex::new(r"[<>]")
            .unwrap();
        let split_message_regex
            = Regex::new(r"[\n ]")
            .unwrap();

        let replaced = if repo_regex.is_match(&msg.content) {
            repo_regex.replace_all(&msg.content, |captures: &Captures| {
                captures[1].to_string() + &captures[2]
            })
        } else {
            return;
        }.replace("https://github.com/", "https://raw.githubusercontent.com/");

        let without_hidden = hidden_link_regex
            .replace_all(&replaced, "");

        let without_spaces = split_message_regex
            .split(&without_hidden);

        let links = without_spaces
            .filter(|s| s.starts_with("https://raw.githubusercontent.com/"));

        let mut dup: Vec<&str> = Vec::new();
        for link in links {
            if dup.contains(&link) {
                continue;
            }

            if let Some(content) = read_message(link.to_string()).await {
                msg.reply(&ctx, content).await.unwrap();
            }

            dup.push(link);
        }
    }
}