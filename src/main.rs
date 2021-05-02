//! # CangreBot

mod startup;
mod bot;
mod commands;
mod handlers;

use std::sync::Arc;

use crate::startup::config;

use color_eyre::eyre::{Result, WrapErr};
use serenity::{
    client::bridge::gateway::ShardManager,
    prelude::{Mutex, TypeMapKey},
};

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

#[tokio::main]
async fn main() -> Result<()> {
    // Subscribirse al administrador de registros de Rust (debug mode).
    // Para ver los mensajes usa `RUST_LOG=debug` en tu archivo `.env`.
    startup::install_tracing();

    //Instalar color-eyre para obtner reportes de erorr con formato de colores
    startup::install_color_eyre()?;

    //Obtener configuración
    let config = config::Config::from_env()?;

    // Generar el client de Serenity para Discord usando el token de `DISCORD_TOKEN`.
    let mut client =
        bot::get_client(config.discord_token.as_str(), config.bot_prefix.as_str()).await?;

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    let shard_manager = client.shard_manager.clone();

    // Detectar la combinación de teclas `CTRL + C` y parar el bot.
    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not register ctrl+c handler.");
        shard_manager.lock().await.shutdown_all().await;
    });

    // Iniciar el bot, si hay un error se mostrará en los registros.
    client.start().await.context("Error starting client")?;

    Ok(())
}
