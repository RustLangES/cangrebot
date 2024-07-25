use songbird::input::AuxMetadata;
use songbird::tracks::Track;
use std::time::Duration;

pub struct MusicPlaylistItem {
    pub url: String,
    pub title: String,
    pub channel: String,
    pub duration: Duration,
    pub thumbnail: String,

    pub source: Option<Track>,
    pub source_url: String,
}

impl MusicPlaylistItem {
    pub fn from_metadata(value: AuxMetadata, source: Track, source_url: String) -> Self {
        MusicPlaylistItem {
            url: value.source_url.unwrap_or_default(),
            title: value.title.unwrap_or(String::from("[object Object]")),
            channel: value.channel.unwrap_or_default(),
            duration: value.duration.unwrap_or_default(),
            thumbnail: value.thumbnail.unwrap_or_default(),

            source: Some(source),
            source_url,
        }
    }
}
