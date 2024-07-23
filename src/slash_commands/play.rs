use axum::http::request;
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
};

use songbird::input::YoutubeDl;

use reqwest::Client as HttpClient;

pub fn register() -> CreateCommand {
    CreateCommand::new("play")
        .description("Play to voice channel")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "url_search",
                "URL to a video or audio / Keywords to search",
            )
            .kind(CommandOptionType::String)
            .required(true),
        )
}

pub async fn run(http_client: &HttpClient, ctx: &Context, cmd: &CommandInteraction) -> String {
    let Some(url) = cmd.data.options.iter().find(|opt| opt.name == "url_search") else {
        return String::from("Must provide a URL to a video or audio");
    };
    let url = url.value.as_str().unwrap().to_owned();

    let do_search = !url.starts_with("http");

    let guild_id = cmd.guild_id.unwrap();

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        let mut src = if do_search {
            YoutubeDl::new_search(http_client.clone(), url)
        } else {
            YoutubeDl::new(http_client.clone(), url)
        };
        let _ = handler.play_input(src.clone().into());

        String::from("Playing song")
    } else {
        String::from("Not in a voice channel to play in")
    }
}
