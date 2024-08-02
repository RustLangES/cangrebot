use std::collections::HashMap;
use regex::{Captures, Regex};
use reqwest::get;
use serenity::all::{Context, CreateMessage, EventHandler, Message};
use serenity::async_trait;
use std::option::Option;
use lazy_static::lazy_static;
use serenity::constants::MESSAGE_CODE_LIMIT;

pub struct ReadGithubLinkHandler;

lazy_static! {
    static ref COMMENT_TEMPLATES: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("c", "// {}");          
        m.insert("cpp", "// {}");        
        m.insert("cs", "// {}");         
        m.insert("java", "// {}");       
        m.insert("js", "// {}");         
        m.insert("go", "// {}");         
        m.insert("kt", "// {}");         
        m.insert("swift", "// {}");      
        m.insert("rs", "// {}");         
        m.insert("scala", "// {}");      
        m.insert("py", "# {}");          
        m.insert("sh", "# {}");          
        m.insert("pl", "# {}");          
        m.insert("rb", "# {}");          
        m.insert("r", "# {}");           
        m.insert("ps1", "# {}");         
        m.insert("php", "// {}");        
        m.insert("sql", "-- {}");        
        m.insert("html", "<!-- {} -->"); 
        m.insert("xml", "<!-- {} -->");  
        m.insert("css", "/* {} */");     
        m.insert("lisp", "; {}");        
        m.insert("scm", "; {}");         
        m.insert("hs", "-- {}");         
        m.insert("m", "% {}");           
        m.insert("asm", "; {}");         
        m.insert("pro", "% {}");         
        m.insert("vim", "\" {}");        
        m.insert("ini", "; {}");         
        m.insert("jl", "# {}");          
        m.insert("erl", "% {}");         
        m.insert("ex", "# {}");          
        m.insert("lua", "-- {}");        
        m.insert("tcl", "# {}");         
        m.insert("yml", "# {}");         
        m.insert("md", "[comment]: # ({})");
        m.insert("lhs", "-- {}");        
        m
    };
}

pub enum RangeOrIndex {
    Language(String),
    Index(String, i32),
    Range(String, i32, i32)
}

pub fn parse_url(url: &str) -> Option<RangeOrIndex> {
    let extension_regex = Regex::new(r"\.([^./?#]+)(#|$)").unwrap();

    let range_regex
        = Regex::new(r"(?:\.(?<language>[^#]+))?(?:#L(?<start>\d+)?(?:-L(?<end>\d+))?)?$")
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

fn trim_message(lang: String, content: String) -> String {
    if content.len() > MESSAGE_CODE_LIMIT {
        content[0..MESSAGE_CODE_LIMIT - 200].to_string()
            + &*format!(
                "\n{}",
                COMMENT_TEMPLATES
                    .get(&*lang)
                    .unwrap_or(&"// {}")
                    .replace("{}", "El mensaje fue cortado por limite de caracteres.")
            )
    } else {
        content
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
                        "Mostrando <{link}>\n```{}\n{}\n```",
                        language.to_owned(),
                        trim_message(language, text)
                    )),
                    RangeOrIndex::Index(language, index)
                    => {
                        if index < subtext.len() as i32 {
                            Some(format!(
                                "Mostrando linea {} de <{link}>\n```{}\n{}\n```",
                                index + 1,
                                language.to_owned(),
                                trim_message(language, subtext[index as usize].to_string()))
                            )
                        } else {
                            None
                        }
                    }
                    RangeOrIndex::Range(language, start, end)
                    => {
                        if start < subtext.len() as i32 && end <= subtext.len() as i32 {
                            Some(format!(
                                "Mostrando desde la linea {} hasta la linea {end} de <{link}>\n```{}\n{}\n```",
                                start + 1,
                                language.to_owned(),
                                trim_message(language, subtext[start as usize..end as usize].join("\n"))
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
        if msg.author.bot {
            return;
        }

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
                if let Some(reference) = &msg.message_reference {
                    msg.channel_id.send_message(
                        &ctx,
                        CreateMessage::new()
                            .content(content)
                            .reference_message(reference.clone())
                    ).await.unwrap();
                } else {
                    msg.reply(&ctx, content).await.unwrap();
                }
            }

            dup.push(link);
        }
    }
}
