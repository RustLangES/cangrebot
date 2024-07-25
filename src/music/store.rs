use std::collections::VecDeque;
use std::sync::Arc;

use serenity::prelude::TypeMapKey;
use songbird::Call;
use tokio::sync::Mutex;

use super::events;
use super::models::{MusicConfig, MusicPlaylistItem};

pub struct MusicStore {
    pub playing: Option<MusicPlaylistItem>,
    pub config: MusicConfig,
    pub queue: VecDeque<MusicPlaylistItem>,
}

impl MusicStore {
    pub fn new() -> MusicStore {
        MusicStore {
            playing: None,
            config: MusicConfig::new(),
            queue: VecDeque::new(),
        }
    }

    pub async fn play_item(
        store_lock: Arc<Mutex<Self>>,
        call_lock: Arc<Mutex<Call>>,
        mut item: MusicPlaylistItem,
    ) {
        let track = item.source.take().expect("execute with unplayed item");
        let track_handle = call_lock.lock().await.play_only(track);

        _ = track_handle.add_event(
            songbird::Event::Track(songbird::TrackEvent::End),
            events::MusicEventEndHandler {
                call: call_lock,
                store: store_lock,
            },
        );
    }
}

impl TypeMapKey for MusicStore {
    type Value = Arc<Mutex<MusicStore>>;
}
