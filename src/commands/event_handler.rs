use std::sync::atomic::Ordering;

use poise::serenity_prelude::{self as serenity, MessageBuilder};

use crate::{Data, Error};
use poise::Event;

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
            println!("new message received: {:?}", new_message.content);
            if new_message.content.to_lowercase().contains("reply poise") {
                let mentions = data.poise_mentions.load(Ordering::SeqCst) + 1;
                data.poise_mentions.store(mentions, Ordering::SeqCst);
                new_message
                    .reply(
                        ctx,
                        format!("This command has been been mentioned {} times", mentions),
                    )
                    .await?;

                break 'early;
            }

            if new_message.content.to_lowercase().contains("poise") {
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
