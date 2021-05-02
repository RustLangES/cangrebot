use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};
use tracing::info;

#[command]
pub async fn ping(ctx: &Context, message: &Message, _: Args) -> CommandResult {
    info!("Ping response");
    message.channel_id.say(&ctx.http, "Pong!").await?;

    Ok(())
}
