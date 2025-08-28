use crate::bot::util::send_multiple;
use poise::serenity_prelude::{Context, EmojiId, Message, ReactionType};

use super::godbolt::parse_args::{DiscordCompilerCommand, DiscordCompilerOutput};

const RUST_CLAP: u64 = 796209434388987928;
const BANEADITTO: u64 = 1312259673587712021;
const APROBADITTO: u64 = 1311155327219007508;

pub async fn message(ctx: &Context, msg: &Message, prefix: &str) -> Result<bool, String> {
    if msg.author.bot || !msg.content.starts_with(format!("{prefix}code").as_str()) {
        return Ok(false);
    }

    let typing = ctx.http.start_typing(msg.channel_id);

    let rust_clap_emoji = ReactionType::Custom {
        animated: true,
        id: EmojiId::new(RUST_CLAP),
        name: Some("rust_clap".into()),
    };

    let baneaditto_emoji = ReactionType::Custom {
        animated: false,
        id: EmojiId::new(BANEADITTO),
        name: Some("baneaditto".into()),
    };

    let aprobaditto_emoji = ReactionType::Custom {
        animated: false,
        id: EmojiId::new(APROBADITTO),
        name: Some("aprobaditto".into()),
    };

    let reaction = msg.react(ctx, rust_clap_emoji).await;

    let compile_result = DiscordCompilerCommand::run(&msg.content)
        .await
        .map_err(|err| format!("{err:#}"));

    typing.stop();

    let compile_result = match compile_result {
        Ok(result) => result,
        Err(err) => {
            msg.reply(ctx, format!("**Error:** {err}")).await.ok();

            _ = msg.react(ctx, baneaditto_emoji).await;

            if let Ok(reaction) = reaction {
                _ = reaction.delete(ctx).await;
            }

            return Err(err);
        }
    };

    let compile_output = match compile_result {
        DiscordCompilerOutput::Raw(text) => text
            .split("<sp>")
            .map(std::string::ToString::to_string)
            .collect(),
        DiscordCompilerOutput::Compiler(output) => {
            if output.is_success() {
                _ = msg.react(ctx, aprobaditto_emoji).await;
            } else {
                _ = msg.react(ctx, baneaditto_emoji).await;
            }

            vec![output.as_discord_message()]
        }
    };

    if let Ok(reaction) = reaction {
        _ = reaction.delete(ctx).await;
    }

    send_multiple(ctx, msg, compile_output, true).await;

    Ok(true)
}
