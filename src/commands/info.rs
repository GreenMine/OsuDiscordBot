use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command
};

use crate::CommandTime;

#[command]
pub fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
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
                }),
        )
        .unwrap();
    Ok(())
}
