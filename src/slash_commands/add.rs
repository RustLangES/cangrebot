use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
};

use reqwest::Client as HttpClient;
use tracing::warn;

pub fn register() -> CreateCommand {
    CreateCommand::new("add")
        .description("Reproduce musica en un canal de voz. Puedes escribir una busqueda o una url")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "url_search",
                "URL del video de Youtube / Busqueda",
            )
            .kind(CommandOptionType::String)
            .required(true),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "primero",
                "La cancion sera añadida el principio de la cola",
            )
            .kind(CommandOptionType::Boolean)
            .required(false),
        )
}

pub async fn run(_http_client: &HttpClient, _ctx: &Context, _cmd: &CommandInteraction) -> String {
    String::from("TODO")
    // let Some(url) = cmd.data.options.iter().find(|opt| opt.name == "url_search") else {
    //     return String::from("Falta la URL o busqueda");
    // };
    //
    // let first = cmd
    //     .data
    //     .options
    //     .iter()
    //     .find(|opt| opt.name == "primero")
    //     .map(|opt| &opt.value)
    //     .map(CommandDataOptionValue::as_bool)
    //     .flatten()
    //     .unwrap_or(false);
    //
    // if let Err(err) = music::join(ctx, cmd).await {
    //     return err;
    // };
    //
    // let url = url.value.as_str().unwrap().to_owned();
    //
    // let is_search = !url.starts_with("http");
    //
    // let guild_id = cmd.guild_id.unwrap();
    //
    // let manager = songbird::get(ctx)
    //     .await
    //     .expect("Songbird Voice client placed in at initialisation.")
    //     .clone();
    //
    // if let Some(handler_lock) = manager.get(guild_id) {
    //     let mut src = if is_search {
    //         YoutubeDl::new_search(http_client.clone(), url.clone())
    //     } else {
    //         YoutubeDl::new(http_client.clone(), url.clone())
    //     };
    //
    //     let data = CreateInteractionResponseMessage::new().content("Procesando cancion");
    //     let builder = CreateInteractionResponse::Defer(data);
    //     if let Err(why) = cmd.create_response(&ctx.http, builder).await {
    //         warn!("Cannot respond to slash command: {}", why);
    //     }
    //
    //     let metadata = src.aux_metadata().await;
    //
    //     let metadata = match metadata {
    //         Ok(m) => m,
    //         Err(err) => {
    //             let builder = EditInteractionResponse::new()
    //                 .content(format!("No se pudo obtener la cancion: {err}"));
    //
    //             if let Err(why) = cmd.edit_response(&ctx.http, builder).await {
    //                 warn!("Cannot respond to slash command: {}", why);
    //             }
    //
    //             return String::new();
    //         }
    //     };
    //
    //     let song_name = metadata
    //         .title
    //         .clone()
    //         .unwrap_or(String::from("[object Object]"));
    //
    //     let data = ctx.data.write().await;
    //     let store = data.get::<MusicStore>().unwrap();
    //     let mut store = store.lock().await;
    //
    //     let playlist_item =
    //         music::models::MusicPlaylistItem::from_metadata(metadata, src.into(), url);
    //
    //     if store.queue.is_empty() && store.playing.is_none() {
    //         info!(" Reproduciendo: {}", song_name);
    //
    //         store.play_item(
    //             handler_lock.clone(),
    //             ctx.data.read().await.get::<MusicStore>().unwrap().clone(),
    //             playlist_item,
    //         );
    //     } else {
    //         if first {
    //             store.queue.push_front(playlist_item);
    //         } else {
    //             store.queue.push_back(playlist_item);
    //         }
    //
    //         info!(" Cola: {}", store.queue.len());
    //     }
    //
    //     let builder =
    //         EditInteractionResponse::new().content(format!("\"{song_name}\" puesta en cola"));
    //
    //     if let Err(why) = cmd.edit_response(&ctx.http, builder).await {
    //         warn!("Cannot respond to slash command: {}", why);
    //     }
    //
    //     String::new()
    // } else {
    //     String::from("El bot no se encuentra en un canal de voz")
    // }
}
