//! # CangreBot

mod bot;
mod commands;
mod handler;

use std::sync::Arc;

use dotenv::dotenv;

use serenity::{
    client::{bridge::gateway::ShardManager},
    prelude::{Mutex, TypeMapKey},
};

use tracing::error;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

#[tokio::main]
async fn main() {
    // Cargar las variables de entorno desde el archivo `.env`.
    // Mira el archivo `.env.example` para saber cómo configurarlo.
    dotenv().expect("Failed to load `.env` file.");

    // Subscribirse al administrador de registros de Rust (debug mode).
    // Para ver los mensajes usa `RUST_LOG=debug` en tu archivo `.env`.
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to start the logger");

    // Generar el client de Serenity para Discord usando el token de `DISCORD_TOKEN`.
    let mut client = bot::get_client().await;

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
    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
