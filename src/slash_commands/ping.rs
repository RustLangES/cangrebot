use serenity::builder::CreateCommand;

pub fn run() -> String {
    "Hey, I'm alive!".to_string()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("ping").description("A ping command")
}