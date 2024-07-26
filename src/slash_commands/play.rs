use lavalink_rs::model::search::SearchEngines;
use lavalink_rs::model::track::TrackLoadData;
use lavalink_rs::player_context::TrackInQueue;
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    CreateInteractionResponse, CreateInteractionResponseMessage, EditInteractionResponse,
};

use reqwest::Client as HttpClient;
use tracing::{error, warn};

use crate::music::{self, MusicStore};

pub fn register() -> CreateCommand {
    CreateCommand::new("play")
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
}

pub async fn run(http_client: &HttpClient, ctx: &Context, cmd: &CommandInteraction) -> String {
    let Some(url) = cmd.data.options.iter().find(|opt| opt.name == "url_search") else {
        return String::from("Falta la URL o busqueda");
    };

    if let Err(err) = music::join(ctx, cmd).await {
        return err;
    };

    let url = url.value.as_str().unwrap().to_owned();

    let is_search = !url.starts_with("http");

    let guild_id = cmd.guild_id.unwrap();

    let data = ctx.data.read().await;
    let client = data.get::<MusicStore>().unwrap().lock().await;
    let Some(player) = client.get_player_context(cmd.guild_id.unwrap()) else {
        let builder =
            EditInteractionResponse::new().content("El bot no se encuentra en un canal de voz");

        if let Err(why) = cmd.edit_response(&ctx.http, builder).await {
            warn!("Cannot respond to slash command: {}", why);
        }

        return String::new();
    };

    let data = CreateInteractionResponseMessage::new().content("Procesando cancion");
    let builder = CreateInteractionResponse::Defer(data);
    if let Err(why) = cmd.create_response(&ctx.http, builder).await {
        warn!("Cannot respond to slash command: {}", why);
    }

    let Ok(track) = client
        .load_tracks(cmd.guild_id.unwrap(), &url)
        .await
        .inspect_err(|err| error!("Error fetching tracks: {err}"))
    else {
        let builder = EditInteractionResponse::new().content(format!(
            "Me arreglas? :sob: <https://github.com/RustLangES/cangrebot>"
        ));

        if let Err(why) = cmd.edit_response(&ctx.http, builder).await {
            warn!("Cannot respond to slash command: {}", why);
        }
        return String::new();
    };

    let mut tracks: Vec<TrackInQueue> = match track.data {
        Some(TrackLoadData::Track(x)) => vec![x.into()],
        Some(TrackLoadData::Search(x)) => vec![x[0].clone().into()],
        Some(TrackLoadData::Playlist(x)) => {
            // playlist_info = Some(x.info);
            x.tracks.iter().map(|x| x.clone().into()).collect()
        }

        _ => {
            let builder = EditInteractionResponse::new().content(format!(
                "Me arreglas? :sob: {track:#?}\n<https://github.com/RustLangES/cangrebot>"
            ));

            if let Err(why) = cmd.edit_response(&ctx.http, builder).await {
                warn!("Cannot respond to slash command: {}", why);
            }
            return String::new();
        }
    };

    let queue = player.get_queue();
    _ = queue
        .append(tracks.into())
        .inspect_err(|err| error!("Append: {err}"));
    let queue_len = queue
        .get_count()
        .await
        .inspect_err(|err| error!("Queue count: {err}"))
        .unwrap_or(0);

    let builder = EditInteractionResponse::new()
        .content(format!("Canciones puestas en cola: {:#?}", queue_len));

    if let Err(why) = cmd.edit_response(&ctx.http, builder).await {
        warn!("Cannot respond to slash command: {}", why);
    }

    if let Ok(player_data) = player.get_player().await {
        if player_data.track.is_none() && queue.get_track(0).await.is_ok_and(|x| x.is_some()) {
            _ = player.skip().inspect_err(|err| error!("Skip: {err}"));
        }
    }

    String::new()
}
