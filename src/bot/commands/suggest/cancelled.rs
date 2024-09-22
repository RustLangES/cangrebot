use crate::bot;

/// Marca una sugerencia como cancelada o que no se realizara
#[poise::command(slash_command, prefix_command)]
pub async fn cancelada(ctx: bot::Context<'_>) -> Result<(), bot::Error> {
    // "Sugerencia Marcada como **Cancelada**".to_string()
    ctx.say("Esta caracteristica aun no se encuentra disponible")
        .await?;

    Ok(())
}
