use std::borrow::Cow;
use std::collections::HashSet;
use std::sync::{Arc, LazyLock};

use poise::serenity_prelude::futures::future::join_all;
use poise::serenity_prelude::{self as serenity, ChannelId, CreateEmbed, GuildId, Http, UserId};
use poise::CreateReply;
use regex::{Captures, Regex};
use reqwest::Client;
use songbird::input::HttpRequest;
use songbird::tracks::Track;
use songbird::Call;
use tokio::sync::Mutex;
use urlencoding::encode;
use uuid::Uuid;

use crate::bot;

pub mod begin;
pub mod end;
pub mod leave;
pub mod skip;
#[allow(clippy::module_inception)]
pub mod tts;

struct TtsTrackData {
    pub uuid: Uuid,
    pub author_id: UserId,
}

macro_rules! replace_patterns  {
    ($text:expr, [ $( ($re:expr, |$caps:ident| $body:expr) ),* $(,)? ]) => {{
        let mut result = $text.to_string();
        $({
            const REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new($re).expect("valid regex"));
            result = REGEX.replace_all(&result, |$caps: &Captures| -> Cow<str> {
                $body
            }).into_owned();
        })*
        result
    }};
}

async fn replace_mentions(
    guild_id: GuildId,
    http: Arc<Http>,
    raw_text: &str,
) -> Result<String, serenity::Error> {
    let mention_re = Regex::new(r"<@(\d+)>").unwrap();
    let mut resolved = String::with_capacity(raw_text.len());
    let mut last_end = 0;

    let matches: Vec<(std::ops::Range<usize>, u64)> = mention_re
        .captures_iter(raw_text)
        .filter_map(|caps| {
            let m = caps.get(0)?;
            let id_str = caps.get(1)?.as_str();
            let id = id_str.parse::<u64>().ok()?;
            Some((m.range(), id))
        })
        .collect();

    let futures = matches.iter().map(|(_, id)| {
        let http = http.clone();
        async move {
            let member = guild_id.member(&http, *id).await?;
            Ok::<_, serenity::Error>(member.display_name().to_string())
        }
    });

    let display_names = join_all(futures).await;

    for ((range, _), name_result) in matches.into_iter().zip(display_names) {
        let nick = name_result?;
        resolved.push_str(&raw_text[last_end..range.start]);
        resolved.push_str(&nick);
        last_end = range.end;
    }

    resolved.push_str(&raw_text[last_end..]);
    Ok(resolved)
}

fn split_text(s: &str, max_chars: usize) -> Vec<String> {
    s.split_whitespace().fold(Vec::new(), |mut acc, word| {
        if let Some(last) = acc.last_mut() {
            if last.len() + 1 + word.len() <= max_chars {
                last.push(' ');
                last.push_str(word);
            } else {
                acc.push(word.to_string());
            }
        } else {
            acc.push(word.to_string());
        }
        acc
    })
}

pub trait TtsStateExt {
    async fn active_channel(&self) -> Option<ChannelId>;
    async fn active_users(&self) -> usize;
    async fn begin(&self, user_id: UserId);
    async fn end(&self, user_id: &UserId) -> bool;
    async fn is_active_user(&self, user_id: &UserId) -> bool;
    async fn join(&self, channel: ChannelId);
    async fn leave(&self, ctx: &serenity::Context, guild_id: GuildId) -> Result<(), bot::Error>;
    async fn check_same_channel(&self, ctx: &bot::Context<'_>) -> Result<bool, bot::Error>;
}

impl TtsStateExt for Mutex<TtsState> {
    async fn active_channel(&self) -> Option<ChannelId> {
        self.lock().await.active_channel
    }

    async fn active_users(&self) -> usize {
        self.lock().await.active_users.len()
    }

    async fn begin(&self, user_id: UserId) {
        self.lock().await.begin(user_id);
    }

    async fn end(&self, user_id: &UserId) -> bool {
        self.lock().await.end(user_id)
    }

    async fn is_active_user(&self, user_id: &UserId) -> bool {
        self.lock().await.is_active_user(user_id)
    }

    async fn join(&self, channel: ChannelId) {
        self.lock().await.join(channel);
    }

    async fn leave(&self, ctx: &serenity::Context, guild_id: GuildId) -> Result<(), bot::Error> {
        self.lock().await.leave(ctx, guild_id).await
    }

    async fn check_same_channel(&self, ctx: &bot::Context<'_>) -> Result<bool, bot::Error> {
        self.lock().await.check_same_channel(ctx).await
    }
}

#[derive(Default)]
pub struct TtsState {
    active_channel: Option<ChannelId>,
    active_users: HashSet<UserId>,
}

impl TtsState {
    pub fn reset(&mut self) {
        self.active_users.clear();
        _ = self.active_channel.take();
    }

    pub fn join(&mut self, channel: ChannelId) {
        _ = self.active_channel.replace(channel)
    }

    async fn leave(
        &mut self,
        ctx: &serenity::Context,
        guild_id: GuildId,
    ) -> Result<(), bot::Error> {
        songbird::get(ctx)
            .await
            .ok_or("Cannot get songbird manager")?
            .leave(guild_id)
            .await?;

        self.reset();

        Ok(())
    }

    pub fn begin(&mut self, user_id: UserId) {
        _ = self.active_users.insert(user_id)
    }

    pub fn end(&mut self, user_id: &UserId) -> bool {
        self.active_users.remove(user_id)
    }

    pub fn is_active_user(&self, user_id: &UserId) -> bool {
        self.active_users.contains(user_id)
    }

    // Returns true if needs to early return
    pub async fn check_same_channel(&self, ctx: &bot::Context<'_>) -> Result<bool, bot::Error> {
        let Some(call_channel) = self.active_channel else {
            ctx.send(
                CreateReply::default().embed(
                    CreateEmbed::new()
                        .description("¿Estoy conectado?")
                        .color(0x00FF_0000),
                ),
            )
            .await?;

            return Ok(true);
        };

        if ctx.channel_id() != call_channel {
            ctx.reply("Permitido solo en el canal de voz donde me encuentro.")
                .await?;

            return Ok(true);
        }

        let member = ctx.author_member().await.ok_or("Failed to get member")?;
        let guild_channel = ctx
            .guild_channel()
            .await
            .ok_or("Failed to get guild channel")?;

        let perms = ctx
            .guild()
            .ok_or("Not in a guild")?
            .user_permissions_in(&guild_channel, member.as_ref());

        // Moderators bypass
        if !perms.manage_messages() {
            return Ok(false);
        }

        let is_on_same_vc = ctx
            .guild()
            .ok_or("No se pudo obtener el guild")?
            .voice_states
            .get(&ctx.author().id)
            .and_then(|vc| vc.channel_id)
            .is_some_and(|channel_id| channel_id == ctx.channel_id());

        if !is_on_same_vc {
            ctx.reply("Tienes que estar conectado en vc para utilizar este comando.")
                .await?;

            return Ok(true);
        }

        Ok(false)
    }

    pub async fn join_vc(
        ctx: &poise::serenity_prelude::Context,
        guild_id: GuildId,
        channel_id: ChannelId,
    ) -> Result<bool, bot::Error> {
        let manager = songbird::get(ctx)
            .await
            .ok_or("No se pudo obtener el manager de voz")?
            .clone();

        Ok(manager
            .join(guild_id, channel_id)
            .await
            .inspect_err(|err| eprintln!("[TTS] Join error: {err}"))
            .is_ok())
    }

    pub async fn send_tts(
        guild_id: GuildId,
        http: Arc<Http>,
        handler: &Mutex<Call>,
        author_id: UserId,
        raw_text: &str,
    ) -> Result<(), bot::Error> {
        let resolved = replace_mentions(guild_id, http.clone(), raw_text).await?;

        let cleaned = replace_patterns!(
            resolved,
            [
                (
                    r"https?://(?:www\.)?[-a-zA-Z0-9@%._+~#=]{2,256}\.[a-z]{2,6}(?:[-a-zA-Z0-9@:%_+.~#?&/=]*)?",
                    |_caps| Cow::Borrowed("enlace")
                ),
                (r"<a?:([a-zA-Z0-9_]+):\d+>", |caps| Cow::Owned(
                    caps[1].to_string()
                )),
            ]
        );

        let client = Client::new();
        let uuid = Uuid::new_v4();
        for c in split_text(&cleaned, 200) {
            let url = format!(
                "https://translate.google.com/translate_tts?client=tw-ob&tl=es&q={}",
                encode(&c)
            );
            let data = HttpRequest::new(client.clone(), url).clone();
            let t: Track =
                Track::new_with_data(data.into(), Arc::new(TtsTrackData { uuid, author_id }));
            handler.lock().await.enqueue(t).await;
        }

        Ok(())
    }
}
