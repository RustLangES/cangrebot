use once_cell::sync::Lazy;
use poise::serenity_prelude::{
    Channel, ChannelId, Context, GetMessages, Member, Message, Timestamp, UserId,
};
use std::time::Instant;
use tokio::sync::Mutex;

use crate::bot;

#[derive(Debug)]
pub struct MessageTracker {
    author_id: UserId,
    message_content: String,
    channel_ids: Vec<ChannelId>,
    last_message_time: Instant,
}

impl MessageTracker {
    pub fn builder() -> MessageTrackerBuilder {
        MessageTrackerBuilder::default()
    }
}

#[derive(Default)]
pub struct MessageTrackerBuilder {
    author_id: Option<UserId>,
    message_content: Option<String>,
    channel_ids: Option<Vec<ChannelId>>,
}

impl MessageTrackerBuilder {
    pub fn author_id(mut self, author_id: UserId) -> Self {
        self.author_id = Some(author_id);
        self
    }

    pub fn message_content(mut self, message_content: String) -> Self {
        self.message_content = Some(message_content);
        self
    }

    pub fn channel_ids(mut self, channel_ids: Vec<ChannelId>) -> Self {
        self.channel_ids = Some(channel_ids);
        self
    }

    pub fn build(self) -> Result<MessageTracker, &'static str> {
        Ok(MessageTracker {
            author_id: self.author_id.ok_or("Author id is missing")?,
            message_content: self.message_content.ok_or("Message content is missing")?,
            channel_ids: self.channel_ids.ok_or("Channel ids are missing")?,
            last_message_time: Instant::now(),
        })
    }
}

static MESSAGE_TRACKER: Lazy<Mutex<Vec<MessageTracker>>> = Lazy::new(|| Mutex::new(Vec::new()));

// pub fn extract_link(text: &str) -> Option<String> {
//     Regex::new(r"(https?://\S+)").map_or(None, |url_re| {
//         url_re.find(text).map(|m| m.as_str().to_string())
//     })
// }

pub async fn message(ctx: &Context, new_message: &Message) -> Result<bool, bot::Error> {
    let author_id = new_message.author.id;
    let mut member = new_message
        .guild_id
        .unwrap()
        .member(&ctx.http, new_message.author.id)
        .await?;
    let mut message_tracker = MESSAGE_TRACKER.lock().await;
    let time = 604800;
    let channel_id = new_message.channel_id;

    if let Some(last_message) = message_tracker.iter().last() {
        if last_message.author_id == author_id
            && last_message.message_content != new_message.content
        {
            message_tracker.clear();
        }
    }

    let message = if let Some(message) = message_tracker
        .iter_mut()
        .find(|m| m.author_id == author_id && m.message_content == new_message.content)
    {
        // Inicializa el tiempo del último mensaje
        message.last_message_time = Instant::now();

        // Si el mensaje existe y el canal no está en la lista de canales, añade el canal a la lista de canales
        if message.channel_ids.contains(&channel_id) {
            // Si el mensaje se repite en el mismo canal, borra el vector
            // Debug: println!("Message repeated in the same channel, clearing the vector");
            message_tracker.clear();

            return Ok(true);
        }

        message.channel_ids.push(channel_id);

        message
    } else {
        // Si el mensaje no existe, crea un nuevo rastreador de mensajes y añádelo a la lista
        let message = MessageTracker::builder()
            .author_id(author_id)
            .message_content(new_message.content.clone())
            .channel_ids(vec![channel_id])
            .build()?;

        message_tracker.push(message);
        message_tracker
            .last_mut()
            .ok_or("Failed to get the last message tracker")?
    };

    if message.channel_ids.len() >= 3 {
        apply_timeout(&mut member, ctx, time, new_message).await?;
        delete_spam_messages(message, ctx, author_id, &new_message.content).await?;

        // Limpia completamente el rastreador de mensajes para reiniciar el rastreo de mensajes
        message_tracker.retain(|m| m.author_id != author_id);
    }
    // Debug: println!("Tracker: {message_tracker:#?}");

    drop(message_tracker);

    Ok(false)
}

async fn delete_spam_messages(
    message: &MessageTracker,
    ctx: &Context,
    author_id: UserId,
    message_content: &String,
) -> Result<(), bot::Error> {
    // Borra cada mensaje individualmente
    for channel_id in &message.channel_ids {
        let channel = channel_id.to_channel(ctx).await?;
        let Channel::Guild(channel) = channel else {
            return Ok(());
        };

        let messages = channel.messages(&ctx.http, GetMessages::new()).await?;
        for message in messages {
            if message.author.id == author_id && &*message.content == message_content {
                message.delete(&ctx.http).await?;
            }
        }
    }

    Ok(())
}

/// Silencia al autor del mensaje y elimina el mensaje
pub async fn apply_timeout(
    member: &mut Member,
    ctx: &Context,
    time_out_timer: i64,
    message: &Message,
) -> Result<(), bot::Error> {
    let time = Timestamp::now().unix_timestamp() + time_out_timer;
    let time = Timestamp::from_unix_timestamp(time)?;
    member
        .disable_communication_until_datetime(&ctx.http, time)
        .await?;
    message.delete(&ctx.http).await?;

    Ok(())
}
