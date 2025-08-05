use std::{collections::HashMap, sync::Arc};

use crate::bot::{Context, Error};
use poise::{
    serenity_prelude::{prelude::TypeMapKey, CreateAttachment, CreateWebhook, ExecuteWebhook},
    CreateReply,
};
use reqwest::header::CONTENT_TYPE;
use tokio::sync::Mutex;

struct AiStore;
impl TypeMapKey for AiStore {
    type Value = Arc<Mutex<Option<String>>>;
}

pub async fn setup_gemini(ctx: &poise::serenity_prelude::client::Context, secret: Option<String>) {
    let secret = Arc::new(Mutex::new(secret));
    let mut data = { ctx.data.write().await };
    data.insert::<AiStore>(secret);
}

const GEMINI_URL: &str =
    "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent";
const SYSTEM_PROMPT: &str =
    "Eres Ferris-chan, la ayudante de IA de la comunidad RustLangES, una comunidad orientada al lenguaje
    de programacion Rust en Español, te comunicas a traves de Discord y puedes formatear tus mensajes con el Markdown
    habilitado en Discord, se amigable y paciente con los usuarios de la comunidad, usa emojis de cangrejos con :crab: y :janky_crab:, manten tus respuestas bajo los 1000 caracteres";
const URLS: [&str; 6] = [
    "https://rustlang-es.org/ - pagina web principal",
    "https://book.rustlang-es.org/ - el libro de rust traducido al español",
    "https://rustlang-es.org/comunidades - nuestras comunidades aliadas",
    "https://rustlang-es.org/colaboradores - nuestros colaboradores",
    "https://blog.rustlang-es.org/ - nuestro blog",
    "https://github.com/RustLangES - nuestro github",
];

/// Haz preguntas a Ferris-chan :3
#[poise::command(slash_command, prefix_command)]
pub async fn ask(ctx: Context<'_>, query: String) -> Result<(), Error> {
    let mut gemini_key: Option<String> = None;
    let store_mutex = {
        let data = { ctx.serenity_context().data.read().await };
        data.get::<AiStore>().cloned()
    };

    if let Some(store) = store_mutex {
        let store = store.lock().await;
        gemini_key.clone_from(&*store);
    }

    let Some(key) = gemini_key else {
        ctx.send(
            CreateReply::default()
                .content("La funcion de IA no esta habilitada")
                .ephemeral(true),
        )
        .await?;
        return Ok(());
    };

    let thinking = ctx
        .send(CreateReply::default().content("Pensando..."))
        .await?;
    let channel = ctx.channel_id();

    let client = reqwest::Client::new();
    let prompt = parse_data(query);

    let response = client
        .post(GEMINI_URL)
        .header("x-goog-api-key", key)
        .header(CONTENT_TYPE, "application/json")
        .json(&prompt)
        .send()
        .await?
        .text()
        .await?;

    let response = serde_json::from_str::<serde_json::Value>(&response)?;

    let text = response
        .get("candidates")
        .and_then(|value| value.get(0))
        .and_then(|value| value.get("content"))
        .and_then(|value| value.get("parts"))
        .and_then(|value| value.get(0))
        .and_then(|value| value.get("text"))
        .and_then(|value| value.as_str())
        .unwrap_or("Ferris-chan esta confundida! :face_with_spiral_eyes:");

    let webhook = channel
        .create_webhook(
            ctx.http(),
            CreateWebhook::new("&Ferris-chan").avatar(&CreateAttachment::bytes(
                include_bytes!("../../../static/ferris_chan.png"),
                "ferris_chan",
            )),
        )
        .await?;

    webhook
        .execute(ctx.http(), true, ExecuteWebhook::new().content(text))
        .await?;
    webhook.delete(ctx.http()).await?;
    thinking.delete(ctx).await?;
    Ok(())
}

pub fn parse_data(prompt: String) -> HashMap<String, HashMap<String, HashMap<String, String>>> {
    let mut root_map: HashMap<String, HashMap<String, HashMap<String, String>>> = HashMap::new();
    let mut parts: HashMap<String, HashMap<String, String>> = HashMap::new();
    let mut text: HashMap<String, String> = HashMap::new();
    let mut sys_parts: HashMap<String, HashMap<String, String>> = HashMap::new();
    let mut sys_text: HashMap<String, String> = HashMap::new();

    text.insert("text".to_string(), prompt);
    parts.insert("parts".to_string(), text);

    let mut system = SYSTEM_PROMPT.to_string();
    system.push_str(
        format!(
            "Asegurate de referirte a nuestros sitios web oficiales si es necesario {}",
            URLS.join("\n")
        )
        .as_str(),
    );

    sys_text.insert("text".to_string(), system);
    sys_parts.insert("parts".to_string(), sys_text);
    root_map.insert("system_instruction".to_string(), sys_parts);

    root_map.insert("contents".to_string(), parts);

    root_map
}
