use shuttle_secrets::SecretStore;

pub mod config;
pub mod general_commands;
pub mod slash_commands;
use config::setup::setup;

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    let Ok(_) = color_eyre::install() else {
        panic!("Failed to install color_eyre");
    };

    let client = setup(secret_store).await?;

    Ok(client.into())
}
