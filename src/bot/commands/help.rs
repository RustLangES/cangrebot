use poise::samples::HelpConfiguration;

use crate::bot::{Context, Error};

/// Mostrar mensaje de ayuda
#[poise::command(slash_command, prefix_command, track_edits, category = "Utility")]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Comando a mirar"]
    #[rest]
    mut command: Option<String>,
) -> Result<(), Error> {
    // This makes it possible to just make `help` a subcommand of any command
    // `/fruit help` turns into `/help fruit`
    // `/fruit help apple` turns into `/help fruit apple`
    if ctx.invoked_command_name() != "help" {
        command = match command {
            Some(c) => Some(format!("{} {}", ctx.invoked_command_name(), c)),
            None => Some(ctx.invoked_command_name().to_string()),
        };
    }

    let prefix = ctx.prefix();

    let extra_text_at_bottom = &format!(
        "\
Type `{prefix}help command` for more info on a command.
You can edit your `{prefix}help` message to the bot and the bot will edit its response."
    );

    let config = HelpConfiguration {
        show_subcommands: true,
        show_context_menu_commands: true,
        ephemeral: true,
        extra_text_at_bottom,

        ..Default::default()
    };

    poise::builtins::help(ctx, command.as_deref(), config).await?;

    Ok(())
}
