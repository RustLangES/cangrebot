use std::sync::Arc;

use axum::async_trait;
use songbird::events::{Event, EventContext, EventHandler as VoiceEventHandler};
use songbird::Call;
use tokio::sync::Mutex;

use crate::music::MusicStore;

pub struct MusicEventEndHandler {
    pub call: Arc<Mutex<Call>>,
    pub store: Arc<Mutex<MusicStore>>,
}

#[async_trait]
impl VoiceEventHandler for MusicEventEndHandler {
    async fn act(&self, _: &EventContext<'_>) -> Option<Event> {
        let mut store = self.store.lock().await;
        let next_song = store.playlist.pop_front();

        if let Some(next_song) = next_song {
            let mut call = self.call.lock().await;

            let track = next_song.source.volume(0.4); // TODO: Get volume from config
            let track_handle = call.play_only(track);

            drop(call);
            drop(store);
            track_handle.add_event(
                Event::Track(songbird::TrackEvent::End),
                MusicEventEndHandler {
                    call: self.call.clone(),
                    store: self.store.clone(),
                },
            );
        }

        None
    }
}
