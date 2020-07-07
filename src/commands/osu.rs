use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::{ext, osu_api, CommandTime, OsuContainer};

#[command]
pub fn get_user(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
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
