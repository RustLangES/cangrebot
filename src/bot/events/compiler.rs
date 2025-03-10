use poise::serenity_prelude::{Context, Message};
use super::godbolt::parse_args::{DiscordCompilerOutput, DiscordCompilerCommand};

pub async fn message(ctx: &Context, msg: &Message) -> Result<bool, String> {
    if msg.author.bot || !msg.content.starts_with("&code") {
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
        DiscordCompilerOutput::Raw(text) => text,
        DiscordCompilerOutput::Compiler(output) => output.as_discord_message()
    };

    msg.reply(ctx, compile_output)
        .await
        .ok();

    Ok(true)
}
