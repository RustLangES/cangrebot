use serenity::{
    client::Context,
    framework::{
        standard::macros::group,
        standard::{macros::command, CommandResult},
    },
    model::channel::Message,
};

#[command]
async fn ping(ctx: &Context, message: &Message) -> CommandResult {
    message.channel_id.say(&ctx.http, "Pong!").await?;

    Ok(())
}

#[group]
#[commands(ping)]
struct General;
