use poise::serenity_prelude::{Context, Message};
use crate::bot::util::send_multiple;

use super::godbolt::parse_args::{DiscordCompilerOutput, DiscordCompilerCommand};

pub async fn message(ctx: &Context, msg: &Message, prefix: &str) -> Result<bool, String> {
    if msg.author.bot || !msg.content.starts_with(format!("{}code", prefix).as_str()) {
        return Ok(false);
    }

    let compile_result = DiscordCompilerCommand::run(&msg.content)
        .await
        .map_err(|err| format!("{err:#}"));

    let compile_result = match compile_result {
        Ok(result) => result,
        Err(err) => {
            msg.reply(ctx, format!("**Error:** {err}"))
                .await
                .ok();

            return Err(err);
        }
    };

    let compile_output = match compile_result {
        DiscordCompilerOutput::Raw(text) => text.split("<sp>").map(|t| t.to_string()).collect(),
        DiscordCompilerOutput::Compiler(output) => vec![output.as_discord_message()]
    };

    send_multiple(ctx, msg, compile_output, true)
        .await;

    Ok(true)
}
