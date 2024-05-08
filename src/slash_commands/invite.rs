use serenity::all::{CommandDataOption, CreateCommand};

pub fn run(_options: &[CommandDataOption]) -> String {
    "https://discord.gg/4ng5HgmaMg".to_string()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("invite").description("Retorna el link de invitación del servidor")
}
