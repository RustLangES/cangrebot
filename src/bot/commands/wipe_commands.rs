use std::fmt::Write;

use poise::serenity_prelude::{Color, CreateEmbed};
use poise::CreateReply;

use crate::bot;

fn format_progress_bar(completed: usize, total: usize) -> String {
    const WIDTH: usize = 10;
    const FULL_FILLED: &str = ":white_large_square:";
    const HALF_FILLED: &str = ":black_square_button:";
    const EMPTY_FILLED: &str = ":black_large_square:";

    let full_chars = completed * WIDTH / total;
    let empty_chars = WIDTH - full_chars;

    let full_chars = FULL_FILLED.repeat(full_chars);
    let empty_chars = EMPTY_FILLED.repeat(empty_chars);

    format!("{full_chars}{HALF_FILLED}{empty_chars}")
}

fn format_progress(completed: usize, total: usize) -> String {
    let progress_bar = format_progress_bar(completed, total);
    format!("Deleting {completed}/{total}\n{progress_bar}")
}

#[poise::command(
    slash_command,
    hide_in_help = true,
    default_member_permissions = "MANAGE_GUILD"
)]
pub async fn wipe_commands(ctx: bot::Context<'_>) -> Result<(), bot::Error> {
    let guild_id = ctx.guild_id().ok_or("Cannot get guild id")?;

    let reply = ctx
        .send(
            CreateReply::default().embed(
                CreateEmbed::default()
                    .title("Running")
                    .color(Color::BLURPLE),
            ),
        )
        .await?;

    let commands = guild_id.get_commands(ctx).await?;
    let n_commands = commands.len();
    let commands_list = commands.iter().fold(String::new(), |mut buf, command| {
        _ = writeln!(buf, "- {}", command.name);

        buf
    });

    reply
        .edit(
            ctx,
            CreateReply::default().embed(
                CreateEmbed::default()
                    .title("Running")
                    .description(format_progress(0, n_commands))
                    .color(Color::BLURPLE),
            ),
        )
        .await?;

    for (command_idx, command) in commands.into_iter().enumerate() {
        guild_id.delete_command(ctx, command.id).await?;

        reply
            .edit(
                ctx,
                CreateReply::default().embed(
                    CreateEmbed::default()
                        .title("Running")
                        .description(format_progress(command_idx, n_commands))
                        .color(Color::BLURPLE),
                ),
            )
            .await?;
    }

    reply
        .edit(
            ctx,
            CreateReply::default().embed(
                CreateEmbed::default()
                    .title(format!("{n_commands} commands deleted"))
                    .description(commands_list)
                    .color(Color::BLURPLE),
            ),
        )
        .await?;

    Ok(())
}
