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

use std::{env, sync::Arc};

extern crate serde;
mod ext;
mod osu_api;

use osu_api::Osu;

struct OsuContainer;

impl TypeMapKey for OsuContainer {
    type Value = Arc<Osu>;
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

    let mut client = Client::new(
        env::var("DISCORD_TOKEN").expect("Unable to get token from env!"),
        Handler,
    )
    .expect("Unable to start client connection!");

    {
        let mut data = client.data.write();
        data.insert::<OsuContainer>(Arc::new(osu));
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
    msg.channel_id
        .say(
            &ctx.http,
            format!("Pong! {}#{}", msg.author.name, msg.author.discriminator),
        )
        .unwrap();
    Ok(())
}

#[command]
fn get_user(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.read();
    let osu_user = if let Some(user) = data.get::<OsuContainer>() {
            match user.get_user(&args.single::<String>().unwrap()) {
                Ok(user) => user,
                Err(error) => {
                    match error {
                        osu_api::types::Error::Osu(osu_error) => {msg.channel_id.send_message(&ctx.http, |m| m.embed(|e| e.color(0xFF0101).title(osu_error.error)))?;},
                        _ => {msg.channel_id.say(&ctx.http, format!("Error! {:?}", error))?;}
                    }
                    return Ok(());
                }
            }
    } else {
        return Ok(());
    };

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

    Ok(())
}
