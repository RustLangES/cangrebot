use crate::bot::{Context, Error};
use poise::{
    serenity_prelude::{GetMessages, MessageId, UserId},
    CreateReply,
};

/// Limpia hasta 100 mensajes
#[poise::command(
    slash_command,
    hide_in_help = true,
    default_member_permissions = "MANAGE_MESSAGES"
)]
pub async fn clear(
    ctx: Context<'_>,
    quantity: Option<u8>,
    user: Option<UserId>,
) -> Result<(), Error> {
    let quantity = quantity.unwrap_or(100);

    if quantity > 100 {
        let _ = ctx
            .send(
                CreateReply::default()
                    .content("Quantity must be less or equal to 100")
                    .ephemeral(true),
            )
            .await;
        return Ok(());
    }

    let channel = ctx.channel_id();

    if let Some(user) = user {
        let mut user_messages: Vec<MessageId> = vec![];

        while user_messages.len() < quantity as usize {
            let messages = if let Some(last_msg) = user_messages.last() {
                channel
                    .messages(
                        &ctx.http(),
                        GetMessages::new().limit(quantity).before(last_msg),
                    )
                    .await
            } else {
                channel
                    .messages(&ctx.http(), GetMessages::new().limit(quantity))
                    .await
            };
            let Ok(messages) = messages else {
                return Err("Couldn't get messages".into());
            };

            let mut messages = messages
                .iter()
                .filter_map(|message| {
                    if message.author.id == user {
                        Some(message.id)
                    } else {
                        None
                    }
                })
                .collect::<Vec<MessageId>>();
            let remaining_quantity = quantity as usize - user_messages.len();
            if messages.len() > remaining_quantity {
                user_messages.extend_from_slice(&messages[0..remaining_quantity]);
            } else {
                user_messages.append(&mut messages);
            }
        }

        if let Err(e) = channel.delete_messages(&ctx.http(), user_messages).await {
            return Err(e.into());
        };
    } else {
        let Ok(messages) = channel
            .messages(&ctx.http(), GetMessages::new().limit(quantity))
            .await
        else {
            return Err("Couldn't get messages".into());
        };

        let messages = messages.iter().map(|message| message.id);
        if let Err(e) = channel.delete_messages(&ctx.http(), messages).await {
            return Err(e.into());
        };
    }

    let _ = ctx
        .send(
            CreateReply::default()
                .content("Deleted messages!")
                .ephemeral(true),
        )
        .await;

    Ok(())
}
