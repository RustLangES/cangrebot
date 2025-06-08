use dotenvy::dotenv;
use tracing::warn;

#[derive(Clone, Debug)]
pub struct CangrebotSecrets {
    /// Closed key for API communication
    pub api_key: String,
    /// Channel id for Daily Challenges
    pub channel_daily: u64,
    /// Channel id for Suggest
    pub channel_suggest: u64,
    /// Waiting channel id for temporal voice chats
    pub temporal_wait: u64,
    /// Category id for temporal voice chats
    pub temporal_category: u64,
    /// Channel id for temporal voice chat logs
    pub temporal_logs: u64,
    /// Prefix for text commands. Defaults to "&"
    pub discord_prefix: String,
    /// Discord Bot Token
    pub discord_token: String,
    /// Server id
    pub guild_id: u64,
}

impl CangrebotSecrets {
    pub fn from<'a>(secrets: fn(&'a str) -> Result<String, std::env::VarError>) -> Self {
        dotenv().ok();
        Self {
            api_key: secrets("BOT_APIKEY").expect("'BOT_APIKEY' was not found"),
            channel_daily: secrets("CHANNEL_DAILY")
                .expect("'CHANNEL_DAILY' was not found")
                .parse()
                .expect("Cannot parse 'CHANNEL_DAILY'"),
            channel_suggest: secrets("CHANNEL_SUGGEST")
                .expect("'CHANNEL_SUGGEST' was not found")
                .parse()
                .expect("Cannot parse 'CHANNEL_SUGGEST'"),
            temporal_wait: secrets("TEMPORAL_WAIT")
                .expect("'TEMPORAL_WAIT' was not found")
                .parse()
                .expect("Cannot parse 'TEMPORAL_WAIT'"),
            temporal_category: secrets("TEMPORAL_CATEGORY")
                .expect("'TEMPORAL_CATEGORY' was not found")
                .parse()
                .expect("Cannot parse 'TEMPORAL_CATEGORY'"),
            temporal_logs: secrets("TEMPORAL_LOGS")
                .expect("'TEMPORAL_LOGS' was not found")
                .parse()
                .expect("Cannot parse 'TEMPORAL_LOGS'"),
            discord_prefix: secrets("DISCORD_PREFIX").unwrap_or_else(|_| {
                warn!("'DISCORD_PREFIX' was not found. Defaults to \"&\"");
                "&".to_owned()
            }),
            discord_token: secrets("DISCORD_TOKEN").expect("'DISCORD_TOKEN' was not found"),
            guild_id: secrets("GUILD_ID")
                .expect("'GUILD_ID' was not found")
                .parse()
                .expect("Cannot parse 'GUILD_ID'"),
        }
    }
}
