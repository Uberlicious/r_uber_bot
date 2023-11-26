use rand::{rngs::StdRng, Rng, SeedableRng};
use regex::RegexSet;
use serde_json::json;
use std::{sync::atomic::Ordering, env};

use giphy::v1::r#async::*;
use giphy::v1::gifs::RandomRequest;

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

                let api_key = env::var("GIPHY_API_KEY").unwrap_or_else(|e| panic!("error retrieving env variable: {:?}", e));
                let client = reqwest::Client::new();
                let api = AsyncApi::new(api_key, client);

                let response = RandomRequest::new()
                    .with_tag("time")
                    .send_to(&api)
                    .await?;        

                if let Err(why) = new_message.channel_id.say(&ctx.http, response.data.embed_url).await {
                    println!("Error sending message: {:?}", why);
                };

                break 'early;
            }
        }
        _ => {}
    }
    Ok(())
}
