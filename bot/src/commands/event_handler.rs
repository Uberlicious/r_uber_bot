use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};
use regex::RegexSet;

use std::sync::atomic::{AtomicU32, Ordering};

use giphy::v1::gifs::RandomRequest;
use giphy::v1::r#async::RunnableAsyncRequest;

use poise::serenity_prelude::{self as serenity, Message};

use crate::{database::models::CommandHistory, Data, Error};

fn trigger_check(message: String, set: &RegexSet) -> bool {
    let matches: Vec<_> = set.matches(message.as_str()).into_iter().collect();

    matches.len() > 0
}

async fn friend_trigger(
    ctx: &serenity::Context,
    msg: &Message,
    data: &Data,
    set: &RegexSet,
    tags: Vec<&str>,
    trigger_name: &str,
) -> Result<(), Error> {
    if !trigger_check(msg.content.to_lowercase(), set) {
        return Ok(());
    }
    let mut rng = StdRng::from_entropy();

    if !rng.gen_ratio(1, 4) {
        return Ok(());
    }

    let response = RandomRequest::new()
        .with_tag(tags.choose(&mut rng).expect("no option"))
        .send_to(&data.giphy_api)
        .await?;

    if let Err(why) = msg.channel_id.say(&ctx.http, response.data.embed_url).await {
        println!("Error sending message: {:?}", why);
    };

    let ch = CommandHistory {
        id: None,
        user_id: msg.author.id.into(),
        guild_id: msg.guild_id.unwrap().into(),
        command_name: String::from(trigger_name),
        executed_at: msg.timestamp.to_utc(),
    };

    data.database.create_command_entry(ch).await?;

    Ok(())
}

pub async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Message { new_message } => 'early: {
            if new_message.author.bot {
                break 'early;
            }

            // Gardy
            friend_trigger(
                ctx,
                new_message,
                data,
                &RegexSet::new([r"gardy.?time", r"grady.?time", r"tobey.?time"]).unwrap(),
                vec!["time"],
                "gardy_time",
            )
            .await?;

            // Luxe
            friend_trigger(
                ctx,
                new_message,
                data,
                &RegexSet::new([r"luxe.?time"]).unwrap(),
                vec!["bathroom", "shower"],
                "luxe_time",
            )
            .await?;

            if new_message.content.to_lowercase().contains("antiquing") {
                if let Err(why) = new_message
                    .channel_id
                    .say(
                        &ctx.http,
                        "https://media2.giphy.com/media/xuDHhHcCR0rew/source.gif",
                    )
                    .await
                {
                    println!("Error sending message: {:?}", why);
                };

                return Ok(());
            }
        }

        serenity::FullEvent::Ready { data_about_bot: _ } => {
            // println!("guilds: {:?}", data_about_bot);
        }

        serenity::FullEvent::CacheReady { guilds } => {
            for guild_id in guilds {
                let g_id = i64::from(guild_id.clone());
                let g = data.database.get_guild(g_id).await?;

                if g.is_none() {
                    data.database.create_guild(g_id).await?;
                }
            }
        }

        _ => {}
    }
    Ok(())
}
