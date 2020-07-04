use serenity::client::Client;
use serenity::framework::standard::{
    macros::{command, group},
    Args, CommandResult, StandardFramework,
};
use serenity::model::{
    channel::Message,
    gateway::{Activity, Ready},
    id::GuildId,
    voice::VoiceState,
};
use serenity::prelude::{Context, EventHandler, TypeMapKey};

use std::{
    env,
    sync::{Arc, Mutex},
};

extern crate serde;
mod circle_buffer;
mod ext;
mod osu_api;

use circle_buffer::CircleBuffer;
use osu_api::Osu;

struct OsuContainer;
impl TypeMapKey for OsuContainer {
    type Value = Arc<Osu>;
}

struct CommandTime;
impl TypeMapKey for CommandTime {
    type Value = Arc<Mutex<CircleBuffer<u128>>>;
}

#[group]
#[commands(ping, get_user)]
struct General;

struct Handler;

impl EventHandler for Handler {
    fn voice_state_update(
        &self,
        _ctx: Context,
        _guild_id: Option<GuildId>,
        _old: Option<VoiceState>,
        new: VoiceState,
    ) {
        println!("{:?}", new);
    }

    fn ready(&self, ctx: Context, data_about_bot: Ready) {
        println!("Bot \"{}\" successefuly started!", data_about_bot.user.name);
        ctx.set_activity(Activity::playing("EEEE"));
    }
}

fn main() {
    let osu = Osu::new(env::var("OSU_TOKEN").expect("Unable to get osu token from env!"));

    let buffer = CircleBuffer::<u128>::new();

    let mut client = Client::new(
        env::var("DISCORD_TOKEN").expect("Unable to get token from env!"),
        Handler,
    )
    .expect("Unable to start client connection!");

    {
        let mut data = client.data.write();
        data.insert::<OsuContainer>(Arc::new(osu));
        data.insert::<CommandTime>(Arc::new(Mutex::new(buffer)));
    }

    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefix("!"))
            .group(&GENERAL_GROUP),
    );
    client.start().unwrap();
}

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read();
    let timer = data.get::<CommandTime>().unwrap().lock()?;
    let average_ping = if timer.current_item != 0 {
        (timer.iter().sum::<u128>() / timer.current_item as u128) as u128
    } else {
        timer.values[0]
    };
    msg.channel_id
        .say(
            &ctx.http,
            format!(
                "Info about bot:\nPing to osu API: {}:{}_square:!\nBot creator: GreenMine\nBot GitHub page: https://github.com/GreenMine/OsuDiscordBot",
                average_ping,
                match average_ping {
                    0..=100 => "green",
                    101..=600 => "yellow",
                    _ => "red"
    }
                ),
        )
        .unwrap();
    Ok(())
}

#[command]
fn get_user(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.read();
    let command_time = std::time::Instant::now();
    let osu_user = if let Some(user) = data.get::<OsuContainer>() {
        match user.get_user(&args.single::<String>().unwrap()) {
            Ok(user) => user,
            Err(error) => {
                match error {
                    osu_api::types::Error::Osu(osu_error) => {
                        msg.channel_id.send_message(&ctx.http, |m| {
                            m.embed(|e| e.color(0xFF0101).title(osu_error.error))
                        })?;
                    }
                    _ => {
                        msg.channel_id
                            .say(&ctx.http, format!("Error! {:?}", error))?;
                    }
                }
                return Ok(());
            }
        }
    } else {
        return Ok(());
    };
    let command_time = command_time.elapsed().as_millis();

    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            let pretty_level = {
                let current_level = osu_user.level.trunc();
                let mut pretty_level = current_level.to_string();
                let to_new_level = ((osu_user.level - current_level) * 10f32) as usize;
                pretty_level += &std::iter::repeat(":green_square:")
                    .take(to_new_level)
                    .collect::<String>();
                pretty_level += &std::iter::repeat(":black_large_square:")
                    .take(10 - to_new_level)
                    .collect::<String>();
                pretty_level += &(current_level + 1f32).to_string();
                pretty_level
            };

            e.description(format!(
                "Country: :flag_{}:",
                osu_user.country.to_lowercase()
            ))
            .color(ext::get_color_from_number(osu_user.user_id))
            .thumbnail(osu_user.get_avatar())
            .author(|a| {
                a.name(&osu_user.username)
                    .url(osu_user.get_profile())
                    .icon_url(osu_user.get_avatar())
            })
            .fields(vec![
                (
                    format!("World rank: {}", osu_user.pp_rank),
                    format!("Country rank: {}", osu_user.pp_country_rank),
                    false,
                ),
                (
                    format!("Ranked score: {}", osu_user.ranked_score),
                    format!("Accuracy: {:.2}%", osu_user.accuracy),
                    true,
                ),
                (
                    format!(
                        "Total time played: {}",
                        ext::pretty_time_print(osu_user.total_seconds_played)
                    ),
                    format!("PP: {}", osu_user.pp_raw.ceil()),
                    true,
                ),
                ("Level".to_string(), pretty_level, false),
            ])
        })
    })?;

    data.get::<CommandTime>()
        .unwrap()
        .lock()?
        .set_next(command_time);

    Ok(())
}
