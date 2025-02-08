use shuttle_runtime::SecretStore;
use tracing::warn;

#[derive(Clone, Debug)]
pub struct CangrebotSecrets {
    /// Closed key for API communication
    pub api_key: String,
    /// Channel id for Daily Challenges
    pub channel_daily: u64,
    /// Channel id for Suggest
    pub channel_suggest: u64,
    /// Prefix for text commands. Defaults to "&"
    pub discord_prefix: String,
    /// Discord Bot Token
    pub discord_token: String,
    /// Server id
    pub guild_id: u64,
}

impl From<SecretStore> for CangrebotSecrets {
    fn from(secrets: SecretStore) -> Self {
        Self {
            api_key: secrets
                .get("BOT_APIKEY")
                .expect("'BOT_APIKEY' was not found"),
            channel_daily: secrets
                .get("CHANNEL_DAILY")
                .expect("'CHANNEL_DAILY' was not found")
                .parse()
                .expect("Cannot parse 'CHANNEL_DAILY'"),
            channel_suggest: secrets
                .get("CHANNEL_SUGGEST")
                .expect("'CHANNEL_SUGGEST' was not found")
                .parse()
                .expect("Cannot parse 'CHANNEL_SUGGEST'"),
            discord_prefix: secrets.get("DISCORD_PREFIX").unwrap_or_else(|| {
                warn!("'DISCORD_PREFIX' was not found. Defaults to \"&\"");
                "&".to_owned()
            }),
            discord_token: secrets
                .get("DISCORD_TOKEN")
                .expect("'DISCORD_TOKEN' was not found"),
            guild_id: secrets
                .get("GUILD_ID")
                .expect("'GUILD_ID' was not found")
                .parse()
                .expect("Cannot parse 'GUILD_ID'"),
        }
    }
}
