use crate::bot::{Context, Error};
use poise::{
    serenity_prelude::{CreateEmbed, Member, Timestamp},
    CreateReply,
};
#[poise::command(slash_command, prefix_command, help_text_fn = "help")]
pub async fn selftimeout(
    ctx: Context<'_>,
    #[description = "Tiempo del timeout en horas"] time: Option<u8>,
) -> Result<(), Error> {
    let Some(member) = ctx.author_member().await else {
        return Err(String::from("Failed to get author of command").into());
    };

    let time = time.unwrap_or(1u8);

    if time < 1 {
        let embed = CreateEmbed::new()
            .title("Selftimeout")
            .description("El timeout debe ser al menos de 1 hora")
            .color(0xFF0000);
        let reply = CreateReply::default();
        ctx.send(reply.embed(embed).ephemeral(true)).await?;
        return Ok(());
    }

    let time_seconds = time as u64 * 3600;

    apply_timeout(&mut member.into_owned(), &ctx, time_seconds).await?;

    let embed = CreateEmbed::new()
        .title("Selftimeout")
        .description(format!(
            "<@{}> se aplico un timeout de `{}` horas",
            ctx.author().id,
            time
        ))
        .color(0xFF0000);

    let reply = CreateReply::default();
    ctx.send(reply.embed(embed)).await?;
    Ok(())
}

fn help() -> String {
    String::from("El timeout no se puede eliminar, por favor usar el comando con precaucion")
}

pub async fn apply_timeout(
    member: &mut Member,
    ctx: &Context<'_>,
    time_out_timer: u64,
) -> Result<(), Error> {
    let time = Timestamp::now().unix_timestamp() + time_out_timer as i64;
    let time = Timestamp::from_unix_timestamp(time)?;
    member
        .disable_communication_until_datetime(ctx.http(), time)
        .await?;

    Ok(())
}
