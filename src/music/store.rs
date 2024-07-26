use std::sync::Arc;

use lavalink_rs::client::LavalinkClient;
use serenity::prelude::TypeMapKey;
use tokio::sync::Mutex;

pub struct MusicStore;

impl TypeMapKey for MusicStore {
    type Value = Arc<Mutex<LavalinkClient>>;
}
