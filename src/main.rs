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
    //    osu.get_beatmaps(2).await;

    //    println!("{:?}", osu.get_user("Cookiezi").await?);

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
        user.get_user(&args.single::<String>().unwrap()).unwrap()
    } else {
        return Ok(());
    };

    msg.channel_id.say(&ctx.http, format!("{:?}", osu_user))?;

    Ok(())
}
