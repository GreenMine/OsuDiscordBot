use serenity::client::Client;
use serenity::framework::standard::{macros::group, StandardFramework};
use serenity::model::{
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
mod commands;
mod ext;
mod osu_api;

use circle_buffer::CircleBuffer;
use osu_api::Osu;

use commands::{info::*, osu::*};

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
        ctx.set_activity(Activity::playing("Welcome to osu!"));
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
