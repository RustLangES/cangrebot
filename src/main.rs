pub use poise::serenity_prelude as serenity;
pub use secrets::CangrebotSecrets;

mod api;
mod bot;
mod secrets;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    color_eyre::install().expect("Failed to install color_eyre");

    let secrets = CangrebotSecrets::from(std::env::var);

    let mut discord_bot = bot::setup(&secrets).await?;
    let router = api::build_router(&secrets, discord_bot.http.clone());

    let port = format!(
        "{}:{}",
        std::env::var("BOT_API_ADDR")
            .as_deref()
            .unwrap_or("0.0.0.0"),
        std::env::var("BOT_API_PORT").as_deref().unwrap_or("8080")
    );
    let listener = tokio::net::TcpListener::bind(port).await?;
    let serve_router = async move {
        axum::serve(listener, router.into_make_service())
            .await
            .unwrap();
    };

    tokio::select! {
        _ = discord_bot.start_autosharded() => {},
        _ = serve_router => {},
    };
    Ok(())
}
