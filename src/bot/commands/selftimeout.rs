use crate::bot::{Context, Error};
use poise::{
    serenity_prelude::{CreateEmbed, Member, Mentionable, Timestamp},
    CreateReply,
};

/// Te aplica un timeout a ti mismo [USAR CON PRECAUCION]
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
            .color(0x00FF_0000);
        let reply = CreateReply::default();
        ctx.send(reply.embed(embed).ephemeral(true)).await?;
        return Ok(());
    }

    let time_seconds = u32::from(time) * 3600;

    apply_timeout(&mut member.into_owned(), &ctx, time_seconds).await?;

    let embed = CreateEmbed::new()
        .title("Selftimeout")
        .description(format!(
            "{} se aplico un timeout de `{}` horas",
            ctx.author().mention(),
            time
        ))
        .color(0x00FF_0000);

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
    time_out_timer: u32,
) -> Result<(), Error> {
    let time = Timestamp::now().unix_timestamp() + i64::from(time_out_timer);
    let time = Timestamp::from_unix_timestamp(time)?;
    member
        .disable_communication_until_datetime(ctx.http(), time)
        .await?;

    Ok(())
}
