use poise::serenity_prelude::{
    ChannelId, Context, CreateEmbed, GuildId, Member, Message, RoleId, UserId, VoiceState,
};
use poise::CreateReply;

use crate::bot;
use crate::bot::commands::{TtsState, TtsStateExt};

const DEFAULT_TTS_ROLE: RoleId = RoleId::new(1410740555385802784);

pub async fn message(ctx: &Context, msg: &Message, data: &bot::Data) -> Result<bool, bot::Error> {
    let Some(call_channel) = data.tts.active_channel().await else {
        return Ok(false);
    };

    if msg.channel_id != call_channel {
        return Ok(false);
    }

    if !data.tts.is_active_user(&msg.author.id).await {
        return Ok(false);
    }

    let guild_id = msg.guild_id.ok_or("No se pudo obtener el guild")?;

    let guild_channel = msg
        .channel(ctx)
        .await?
        .guild()
        .ok_or("Not a guild channel")?;

    let member = msg.member(ctx).await?;

    let perms = msg
        .guild(ctx.as_ref())
        .ok_or("Not in a guild")?
        .user_permissions_in(&guild_channel, &member);

    if !perms.speak() {
        return Ok(false);
    }

    let manager = songbird::get(ctx)
        .await
        .ok_or("No se pudo obtener el manager de voz")?
        .clone();

    let Some(handler_lock) = manager.get(guild_id) else {
        msg.channel_id
            .send_message(
                ctx,
                CreateReply::default()
                    .embed(
                        CreateEmbed::new()
                            .title("Error")
                            .description("No estoy en ningún canal de voz. Usa /join primero.")
                            .color(0x00FF_0000),
                    )
                    .to_prefix(msg.into()),
            )
            .await?;

        return Ok(true);
    };

    let is_unique_speaker = data.tts.active_users().await == 1;
    let is_consecutive_speaker = data.tts.is_last_user(&msg.author.id).await;

    let raw_text = if is_unique_speaker || is_consecutive_speaker {
        &msg.content
    } else {
        &format!("{} dice: {}", msg.author.display_name(), &msg.content)
    };

    data.tts.set_last_user(msg.author.id).await;

    TtsState::send_tts(
        guild_id,
        ctx.http.clone(),
        &handler_lock,
        msg.author.id,
        raw_text,
    )
    .await?;

    Ok(false)
}

pub async fn moved(
    ctx: &Context,
    channel_id: ChannelId,
    data: &bot::Data,
) -> Result<(), bot::Error> {
    let mut tts = data.tts.lock().await;

    if tts
        .active_channel()
        .is_some_and(|active_channel| active_channel == &channel_id)
    {
        return Ok(());
    }

    tts.reset();
    tts.join(channel_id);

    let tts_members = channel_id
        .to_channel(ctx)
        .await?
        .guild()
        .expect("This bot is for guilds")
        .members(ctx)?
        .into_iter()
        .filter(|member| !member.user.bot && member.roles.contains(&DEFAULT_TTS_ROLE))
        .map(|member| member.user.id);

    for member in tts_members {
        tts.begin(member);
    }

    Ok(())
}

pub async fn join(
    ctx: &Context,
    member: &Member,
    guild_id: &GuildId,
    channel_id: ChannelId,
    data: &bot::Data,
) -> Result<(), bot::Error> {
    match data.tts.active_channel().await {
        Some(id) if id == channel_id && member.roles.contains(&DEFAULT_TTS_ROLE) => {
            // Bot is already in vc
            data.tts.begin(member.user.id).await;
        }
        Some(_) => { /* In another vc or new user doesn't require tts */ }
        None => {
            // Could join bot

            let vc_members = channel_id
                .to_channel(ctx)
                .await?
                .guild()
                .expect("This bot is for guilds")
                .members(ctx)?;

            let (non_tts_members, tts_members) = vc_members
                .into_iter()
                .filter(|member| !member.user.bot)
                .map(|member| (member.roles.contains(&DEFAULT_TTS_ROLE), member.user.id))
                .fold(
                    (0, Vec::new()),
                    |(mut non_tts, mut tts), (is_tts, member_id)| {
                        if is_tts {
                            tts.push(member_id);
                        } else {
                            non_tts += 1;
                        }

                        (non_tts, tts)
                    },
                );

            let total_members = non_tts_members + tts_members.len();
            let should_join = !tts_members.is_empty() && total_members > 1;

            if !should_join {
                return Ok(());
            }

            if !TtsState::join_vc(ctx, *guild_id, channel_id).await? {
                return Ok(());
            }

            let mut tts = data.tts.lock().await;

            tts.join(channel_id);

            for member in tts_members {
                tts.begin(member);
            }
        }
    }

    Ok(())
}

pub async fn quit(
    ctx: &Context,
    bot_id: UserId,
    guild_id: &GuildId,
    state: &VoiceState,
    data: &bot::Data,
) -> Result<(), bot::Error> {
    if state.user_id == bot_id {
        data.tts.reset().await;

        return Ok(());
    }

    if state.channel_id != data.tts.active_channel().await {
        return Ok(());
    }

    if !data.tts.end(&state.user_id).await {
        return Ok(());
    }

    if data.tts.active_users().await == 0 {
        data.tts.leave(ctx, *guild_id).await?;

        let channel_id = state.channel_id.unwrap();

        channel_id
            .say(ctx, "Me retiro, veo que ya no me necesitan\n-# Aún me puedo unir si haces `/tts begin` o `/tts play`")
            .await?;

        return Ok(());
    }

    // Maybe we can move this to another module
    let channel_members = state
        .channel_id
        .unwrap()
        .to_channel(ctx)
        .await?
        .guild()
        .ok_or("Cannot get guild channel")?
        .members(ctx)?
        .len();

    // I'm alone, very alone, just alone, my alone, our alone
    if channel_members <= 1 {
        data.tts.leave(ctx, *guild_id).await?;

        let channel_id = state.channel_id.unwrap();

        channel_id
            .say(ctx, "No sé, me quede solo, supongo que me iré..")
            .await?;
    }

    Ok(())
}
