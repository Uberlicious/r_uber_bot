use rand::{rngs::StdRng, Rng, SeedableRng};
use regex::RegexSet;
use serde_json::json;
use std::sync::atomic::Ordering;

use poise::serenity_prelude::{self as serenity, Attachment, MessageBuilder};

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

                println!("gardy time: {}", data.gardy_count.load(Ordering::SeqCst));
                // if !rng.gen_ratio(1, 4) {
                //     break 'early;
                // }

                let count = data.gardy_count.load(Ordering::SeqCst) + 1;
                data.gardy_count.store(count, Ordering::SeqCst);

                let _channel = match new_message.channel_id.to_channel(&ctx).await {
                    Ok(channel) => channel,
                    Err(why) => {
                        println!("Error getting channel: {:?}", why);

                        break 'early;
                    }
                };

                let client =
                    giphy_api::Client::new(String::from("WlUS2Sd7uP61mfO02n3SUS8oUISZOF2b"));

                let giphy_response = client.gifs().random("time", "").await?;

                let gif = giphy_response.body.data.expect("no data");

                if let Err(why) = new_message.channel_id.say(&ctx.http, gif.url).await {
                    println!("Error sending message: {:?}", why)
                };

                break 'early;
            }
        }
        _ => {}
    }
    Ok(())
}
