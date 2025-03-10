use poise::serenity_prelude::{Context, CreateMessage, Message};

const MASKS: [(&str, &str); 4] = [
    ("github.com/", "github:"),
    ("gitlab.com/", "gitlab:"),
    ("docs.rs/", "docs.rs:"),
    ("crates.io/crates/", "crates:"),
];

pub fn mask_url(url: String) -> String {
    let mut masked = url.replace("https://", "").replace("http://", "");

    for (mask, repl) in &MASKS {
        if masked.starts_with(mask) {
            masked = masked.replace(mask, repl);
            break;
        }
    }

    format!("[{masked}]({url})")
}

pub async fn send_multiple(
    ctx: &Context,
    caller: &Message,
    msgs: Vec<String>,
    mut reply: bool
) {
    for msg in msgs {
        if reply {
            caller
                .reply(ctx, msg)
                .await
                .ok();

            reply = false;
        } else {
            caller
                .channel_id
                .send_message(
                    ctx,
                    CreateMessage::new()
                        .content(msg)
                )
                .await
                .ok();
        }
    }
}
