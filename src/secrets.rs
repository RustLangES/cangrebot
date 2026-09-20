use dotenvy::dotenv;
use tracing::warn;

#[derive(Clone, Debug)]
pub struct CangrebotSecrets {
    pub features: FeatureFlags,
    /// Closed key for API communication
    pub api_key: Option<String>,
    /// Channel id for Daily Challenges
    pub channel_daily: Option<u64>,
    /// Channel id for Suggest
    pub channel_suggest: Option<u64>,
    /// Channel id for Showcase
    pub channel_showcase: Option<u64>,
    /// Path for showcase sync cache file
    pub showcase_cache_path: String,
    /// Waiting channel id for temporal voice chats
    pub temporal_wait: Option<u64>,
    /// Category id for temporal voice chats
    pub temporal_category: Option<u64>,
    /// Channel id for temporal voice chat logs
    pub temporal_logs: Option<u64>,
    /// Prefix for text commands. Defaults to "&"
    pub discord_prefix: String,
    /// Gemini key
    pub gemini_key: Option<String>,
    /// Discord Bot Token
    pub discord_token: String,
    /// Server id
    pub guild_id: u64,
}

#[derive(Clone, Debug)]
#[allow(clippy::struct_excessive_bools)]
pub struct FeatureFlags {
    pub api: bool,
    pub daily_challenges: bool,
    pub suggestions: bool,
    pub showcase: bool,
    pub temporal_channels: bool,
    pub ai: bool,
}

impl CangrebotSecrets {
    /// # Panics
    ///
    /// This function will panic if any of the secrets it's not found in the environment file
    pub fn from<'a>(secrets: fn(&'a str) -> Result<String, std::env::VarError>) -> Self {
        dotenv().ok();

        let api_key = secrets("BOT_APIKEY").ok();

        let channel_daily = secrets("CHANNEL_DAILY")
            .map_err(|_| warn!("'CHANNEL_DAILY' was not found. Daily challenge feature disabled"))
            .ok()
            .and_then(|s| s.parse().ok());

        let channel_suggest = secrets("CHANNEL_SUGGEST")
            .map_err(|_| warn!("'CHANNEL_SUGGEST' was not found. Suggestions feature disabled"))
            .ok()
            .and_then(|s| s.parse().ok());

        let channel_showcase = secrets("CHANNEL_SHOWCASE")
            .map_err(|_| warn!("'CHANNEL_SHOWCASE' was not found. Showcase feature disabled"))
            .ok()
            .and_then(|s| s.parse().ok());

        let showcase_cache_path = secrets("SHOWCASE_CACHE_PATH").unwrap_or_else(|_| {
            warn!("'SHOWCASE_CACHE_PATH' was not found. Defaults to \"showcase_cache.json\"");
            "showcase_cache.json".to_owned()
        });

        let temporal_wait = secrets("TEMPORAL_WAIT")
            .map_err(|_| warn!("'TEMPORAL_WAIT' was not found. Temporal channels feature disabled"))
            .ok()
            .and_then(|s| s.parse().ok());

        let temporal_category = secrets("TEMPORAL_CATEGORY")
            .map_err(|_| {
                warn!("'TEMPORAL_CATEGORY' was not found. Temporal channels feature disabled")
            })
            .ok()
            .and_then(|s| s.parse().ok());

        let temporal_logs = secrets("TEMPORAL_LOGS")
            .map_err(|_| warn!("'TEMPORAL_LOGS' was not found. Temporal channels feature disabled"))
            .ok()
            .and_then(|s| s.parse().ok());

        let discord_prefix = secrets("DISCORD_PREFIX").unwrap_or_else(|_| {
            warn!("'DISCORD_PREFIX' was not found. Defaults to \"&\"");
            "&".to_owned()
        });

        let gemini_key = secrets("GEMINI_KEY")
            .map_err(|_| warn!("'GEMINI_KEY' was not found. AI feature disabled"))
            .ok()
            .and_then(|s| s.parse().ok());

        let discord_token = secrets("DISCORD_TOKEN").expect("'DISCORD_TOKEN' was not found");

        let guild_id = secrets("GUILD_ID")
            .expect("'GUILD_ID' was not found")
            .parse()
            .expect("Cannot parse 'GUILD_ID'");

        let features = FeatureFlags {
            api: api_key.is_some(),
            daily_challenges: channel_daily.is_some(),
            suggestions: channel_suggest.is_some(),
            showcase: channel_showcase.is_some(),
            temporal_channels: temporal_category.is_some()
                && temporal_category.is_some()
                && temporal_logs.is_some(),
            ai: gemini_key.is_some(),
        };

        Self {
            features,
            api_key,
            channel_daily,
            channel_suggest,
            channel_showcase,
            showcase_cache_path,
            temporal_wait,
            temporal_category,
            temporal_logs,
            discord_prefix,
            gemini_key,
            discord_token,
            guild_id,
        }
    }
}
