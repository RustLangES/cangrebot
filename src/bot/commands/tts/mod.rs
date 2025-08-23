use poise::serenity_prelude::{ChannelId, CreateEmbed};
use poise::CreateReply;
use tokio::sync::Mutex;

use crate::bot;

pub mod join;
pub mod leave;
pub mod stop;
#[allow(clippy::module_inception)]
pub mod tts;

pub trait TtsStateExt {
    async fn reset(&self);
    async fn join(&self, channel: ChannelId);
    async fn check_same_channel(&self, ctx: &bot::Context<'_>) -> Result<bool, bot::Error>;
}

impl TtsStateExt for Mutex<TtsState> {
    async fn reset(&self) {
        self.lock().await.reset();
    }

    async fn join(&self, channel: ChannelId) {
        self.lock().await.join(channel);
    }

    async fn check_same_channel(&self, ctx: &bot::Context<'_>) -> Result<bool, bot::Error> {
        self.lock().await.check_same_channel(ctx).await
    }
}

#[derive(Default)]
pub struct TtsState {
    active_channel: Option<ChannelId>,
}

impl TtsState {
    pub fn reset(&mut self) {
        _ = self.active_channel.take();
    }

    pub fn join(&mut self, channel: ChannelId) {
        _ = self.active_channel.replace(channel)
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

        let is_on_same_vc = ctx
            .guild()
            .ok_or("No se pudo obtener el guild")?
            .voice_states
            .get(&ctx.author().id)
            .and_then(|vc| vc.channel_id)
            .is_some_and(|channel_id| channel_id == ctx.channel_id());

        if is_on_same_vc {
            ctx.reply("Tienes que estar conectado en vc para utilizar este comando.")
                .await?;

            return Ok(true);
        }

        Ok(false)
    }
}
