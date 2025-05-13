use reqwest::Client;
use serde::Serialize;
use std::error::Error;
use serde_json::Value;


/// Estructura que contiene las estadísticas del servidor
#[derive(Serialize, Clone)]
pub struct ServerStats {
    pub guild_id: String,
    pub guild_name: String,
    pub total_members: usize,
    pub active_members: usize,
    pub total_channels: usize,
    pub total_messages: u64,
    pub daily_messages: u64,
    pub monthly_messages: u64,
    pub latest_messages: Option<Vec<Value>>,
    pub new_members: Option<Vec<Value>>,
}

/// Función para enviar las estadísticas a la API externa
pub async fn send_stats_to_api(stats: ServerStats) -> Result<(), Box<dyn Error>> {
    let api_url = "https://webhook.site/7b724fc1-c713-45cd-8b86-fade7386a693";
    let client = Client::new();

    let response = client.post(api_url).json(&stats).send().await?;

    if response.status().is_success() {
        println!("✅ Stats enviadas correctamente");
    } else {
        println!("⚠️ Error al enviar stats: {:?}", response.status());
    }

    Ok(())
}
