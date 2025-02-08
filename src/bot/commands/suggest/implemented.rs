use crate::bot;

/// Marca una sugerencia como implementada
#[poise::command(slash_command, prefix_command)]
pub async fn implementada(ctx: bot::Context<'_>) -> Result<(), bot::Error> {
    // "Sugerencia Marcada como **Implementada**".to_string()
    ctx.say("Esta caracteristica aun no se encuentra disponible".to_string())
        .await?;

    Ok(())
}
