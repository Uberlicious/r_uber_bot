use std::env;
use std::sync::atomic::AtomicU32;

use poise::serenity_prelude as serenity;

mod commands;
use commands::{age, event_handler, gardy_count};

pub struct Data {
    // giphy: giphy_api::Client,
    gardy_count: AtomicU32,
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");

    let options = poise::FrameworkOptions {
        event_handler: |_ctx, event, _framework, _data| {
            Box::pin(event_handler(_ctx, event, _framework, _data))
        },
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age(), gardy_count()],
            ..Default::default()
        })
        .token(env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
        .intents(
            serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT,
        )
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    // giphy: giphy_client,
                    gardy_count: AtomicU32::new(0),
                })
            })
        })
        .options(options);

    framework.run().await.unwrap();
}
