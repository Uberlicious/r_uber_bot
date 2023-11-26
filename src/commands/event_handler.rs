use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};
use regex::RegexSet;

use std::sync::atomic::{AtomicU32, Ordering};

use giphy::v1::gifs::RandomRequest;
use giphy::v1::r#async::RunnableAsyncRequest;

use poise::serenity_prelude::{self as serenity, Message};

use crate::{Data, Error};
use poise::Event;

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
    count: Option<&AtomicU32>,
) -> Result<(), Error> {
    if !trigger_check(msg.content.to_lowercase(), set) {
        return Ok(());
    }
    let mut rng = StdRng::from_entropy();

    if !rng.gen_ratio(1, 4) {
        return Ok(());
    }

    if let Some(count) = count {
        let new_count = count.load(Ordering::SeqCst) + 1;
        count.store(new_count, Ordering::SeqCst);
    }

    let response = RandomRequest::new()
        .with_tag(tags.choose(&mut rng).expect("no option"))
        .send_to(&data.giphy_api)
        .await?;

    if let Err(why) = msg.channel_id.say(&ctx.http, response.data.embed_url).await {
        println!("Error sending message: {:?}", why);
    };

    Ok(())
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

            // Gardy
            friend_trigger(
                ctx,
                new_message,
                data,
                &RegexSet::new(&[r"gardy.?time", r"grady.?time", r"tobey.?time"]).unwrap(),
                vec!["time"],
                Some(&data.gardy_count),
            )
            .await?;

            // Luxe
            friend_trigger(
                ctx,
                new_message,
                data,
                &RegexSet::new(&[r"luxe.?time"]).unwrap(),
                vec!["bathroom", "shower"],
                Some(&data.luxe_count),
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

        _ => {}
    }
    Ok(())
}
