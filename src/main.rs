use std::env;
use std::sync::atomic::AtomicU32;

use giphy::v1::r#async::*;

use poise::serenity_prelude as serenity;

mod commands;
use commands::{age, event_handler, gardy_count};

pub struct Data {
    gardy_count: AtomicU32,
    giphy_api: AsyncApi,
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");

    let api_key = env::var("GIPHY_API_KEY")
        .unwrap_or_else(|e| panic!("error retrieving env variable: {:?}", e));
    let client = reqwest::Client::new();
    let api = AsyncApi::new(api_key, client);

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age(), gardy_count()],
            event_handler: |_ctx, event, _framework, _data| {
                Box::pin(event_handler(_ctx, event, _framework, _data))
            },
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
                    giphy_api: api,
                    gardy_count: AtomicU32::new(0),
                })
            })
        });

    framework.run().await.unwrap();
}
