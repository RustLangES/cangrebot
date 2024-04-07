use anyhow::Result;
use serenity::all::{CreateAttachment, CreateMessage};
use serenity::{model::prelude::*, prelude::*};
use std::convert::TryFrom;

const WELCOME_MESSAGE: &str = r#"¡Bienvenidx a la Comunidad de RustLangES!

Nos alegra que hayas decidido unirte a nuestra comunidad. Aquí encontrarás varios canales dedicados a diferentes aspectos de nuestra comunidad:

- [#anuncios-de-la-comunidad](<https://discord.com/channels/778674594856960012/1159719259287597087>): Este es el lugar donde compartimos las últimas novedades y eventos de nuestra comunidad. ¡Mantente al tanto de lo que está sucediendo!
- [#show-case](<https://discord.com/channels/778674594856960012/1144727580323369000>): ¿Has creado algo increíble con Rust? ¡Este es el canal perfecto para compartirlo con el resto de la comunidad!
- [#proyectos-comunitarios](<https://discord.com/channels/778674594856960012/1140802416170770463>): Aquí se discuten los proyectos que estamos desarrollando como comunidad, como nuestra página web, blog y bot. ¡Participa y ayúdanos a mejorar!
- [#retos-diarios](<https://discord.com/channels/778674594856960012/1219703076944871616>): ¿Quieres poner a prueba tus habilidades de programación? ¡Únete a los retos diarios y comparte tus soluciones!
- [#principiantes](<https://discord.com/channels/778674594856960012/795836875872141362>): Si estas empezando en Rust, este es el lugar perfecto para encontrar ayuda y recursos para empezar.

Recuerda revisar los mensajes fijados en cada canal para obtener más información.

> **Nota:** Es posible que para acceder a algunos canales necesites de un rol especifico
> por lo que te recomendamos que te asignes los roles que te interesen

¡No olvides seguirnos en nuestras redes sociales y visitar nuestras webs para mantenerte al día con todo lo que sucede en nuestra comunidad!

> **Web:** https://rustlang-es.org
> **Blog:** <https://rustlang-es.org/blog>
> **Recursos para aprender Rust:** https://rustlang-es.org/aprende
> **GitHub:** <https://github.com/RustLangES>
> **Linkedin:** <https://www.linkedin.com/company/rustlanges>

¡Bienvenidx una vez más y disfruta de tu estancia en nuestro servidor!"#;

pub async fn guild_member_addition(ctx: &Context, guild_id: &GuildId, member: &Member) {
    if let Err(e) = _guild_member_addition(ctx, guild_id, member).await {
        tracing::error!("Failed to handle welcome guild_member_addition: {}", e);
    }
}

#[tracing::instrument(skip(ctx))]
async fn _guild_member_addition(ctx: &Context, guild_id: &GuildId, member: &Member) -> Result<()> {
    let join_msg = "Bienvenid@ <mention> a <server>! Pásala lindo!".to_string();

    let msg_channel = ChannelId::new(778674893851983932_u64);

    let join_msg_replaced = join_msg
        .replace("<mention>", &member.user.mention().to_string())
        .replace("<username>", &member.user.name)
        .replace("<server>", &guild_id.name(ctx).unwrap_or_else(|| "".into()));

    // Download the user's avatar and create a welcome image
    let avatar_url = member
        .user
        .avatar_url()
        .unwrap_or_else(|| member.user.default_avatar_url());
    let response = reqwest::get(avatar_url).await?;
    let avatar = response.bytes().await?;

    let output_path = format!("/tmp/{}_welcome.png", member.user.name);

    gen_welcome::generate(
        "./static/welcome_background.png",
        &avatar,
        &member.user.global_name.clone().unwrap_or(member.user.name.clone()),
        None,
        include_bytes!("../../static/fonts/WorkSans-Bold.ttf"),
        include_bytes!("../../static/fonts/WorkSans-Regular.ttf"),
        &output_path,
    )
    .expect("Cannot generate welcome image");

    let attachment = CreateAttachment::path(output_path.as_str()).await?;

    let msg = msg_channel
        .send_files(
            &ctx,
            vec![attachment],
            CreateMessage::new().content(&join_msg_replaced),
        )
        .await?;

    // Remove the file after sending the message
    std::fs::remove_file(&output_path)?;

    // Convert string emoji to ReactionType to allow custom emojis
    let reaction = ReactionType::try_from("👋")?;
    msg.react(&ctx, reaction).await?;

    // Send DM with guides
    member
        .user
        .dm(ctx, CreateMessage::new().content(WELCOME_MESSAGE))
        .await?;

    Ok(())
}
