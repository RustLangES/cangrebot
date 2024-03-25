use std::path::PathBuf;
use shuttle_runtime::SecretStore;

pub mod config;
pub mod events;
pub mod general_commands;
pub mod slash_commands;
use config::setup::setup;

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_runtime::Secrets] secret_store: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    let Ok(_) = color_eyre::install() else {
        panic!("Failed to install color_eyre");
    };
    let public_folder = PathBuf::from("static");

    let client = setup(secret_store,public_folder).await?;

    Ok(client.into())
}
