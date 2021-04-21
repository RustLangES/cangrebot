pub mod config;

use color_eyre::{eyre::{Result, WrapErr}, config::HookBuilder};
use tracing::{info, instrument};
use tracing_error::ErrorLayer;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[instrument]
pub fn install_tracing() {
    info!("Installing tracing");
    let fmt_layer = fmt::layer().with_target(false);
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .with(ErrorLayer::default())
        .init();
}

#[instrument]
pub fn install_color_eyre() -> Result<()> {
    info!("Installing color-eyre");
    HookBuilder::default()
    .panic_section("consider reporting the bug on github")
    .install().context("Error installing color-eyre")?;

    Ok(())
}
