use rand::{rngs::StdRng, Rng, SeedableRng};
use regex::RegexSet;
use std::sync::atomic::Ordering;

use poise::serenity_prelude::{self as serenity, MessageBuilder};

use crate::{Data, Error};
use poise::Event;

fn gardy_check(message: String) -> bool {
    let set = RegexSet::new(&[r"gardy.?time", r"tobey.?time", r"grady.?time"]).unwrap();

    let matches: Vec<_> = set.matches(message.as_str()).into_iter().collect();

    matches.len() > 0
}

pub async fn event_handler(
    ctx: &serenity::Context,
    event: &Event<'_>,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        Event::Ready { data_about_bot } => {
            println!("Logged in as {}", data_about_bot.user.name);
        }

        Event::Message { new_message } => 'early: {
            if new_message.author.bot {
                break 'early;
            }

            if gardy_check(new_message.content.to_lowercase()) {
                let mut rng = StdRng::from_entropy();

                if !rng.gen_ratio(1, 4) {
                    break 'early;
                }

                let _channel = match new_message.channel_id.to_channel(&ctx).await {
                    Ok(channel) => channel,
                    Err(why) => {
                        println!("Error getting channel: {:?}", why);

                        break 'early;
                    }
                };

                let content = MessageBuilder::new().push("user p'd").build();

                if let Err(why) = new_message.channel_id.say(&ctx.http, content).await {
                    println!("Error sending message: {:?}", why)
                };

                break 'early;
            }
        }
        _ => {}
    }
    Ok(())
}
