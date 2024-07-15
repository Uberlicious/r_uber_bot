// #![warn(clippy::str_to_string)]

use std::{
    env::var,
    sync::{atomic::AtomicU32, Arc},
    time::Duration,
};

use database::{db::Database, models::CommandHistory};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use color_eyre::{eyre::Report, Section};

use giphy::v1::r#async::*;

use poise::serenity_prelude::{self as serenity};
use superhero_api::superhero::SuperheroApi;

mod commands;
mod database;
mod superhero_api;

pub struct Data {
    giphy_api: AsyncApi,
    superhero_api: SuperheroApi,
    database: Database,
    gardy_count: AtomicU32,
    luxe_count: AtomicU32,
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
type PartialContext<'a> = poise::PartialContext<'a, Data, Error>;

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx, .. } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error,);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {}", e)
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Report> {
    color_eyre::install()?;
    dotenvy::dotenv().ok();

    // giphy api
    let giphy_api_key = dotenvy::var("GIPHY_API_KEY").section("GIPHY_API_KEY must be set")?;
    let client = reqwest::Client::new();
    let api = AsyncApi::new(giphy_api_key, client);

    // superhero api key
    let superhero_api_key =
        dotenvy::var("SUPERHERO_API_KEY").section("SUPERHERO_API_KEY must be set")?;
    let super_api = SuperheroApi::new(superhero_api_key);

    // database init
    // let db_user = dotenvy::var("POSTGRES_USER").section("POSTGRES_USER must be set")?;
    // let db_password = dotenvy::var("POSTGRES_PASSWORD").section("POSTGRES_PASSWORD must be set")?;
    // let db = dotenvy::var("POSTGRES_DB").section("POSTGRES_DB must be set")?;
    let db_url = dotenvy::var("DATABASE_URL").section("DATABASE_URL must be set")?;
    println!("{db_url}");
    let pool: Pool<Postgres> = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    // sqlx::migrate!("./migrations").run(&pool).await?;

    println!("migration run");

    let database = Database::new(pool);

    // FrameworkOptions contains all of poise's configuration option in one struct
    // Every option can be omitted to use its default value
    let options = poise::FrameworkOptions {
        commands: vec![
            commands::age::age(),
            commands::age::gardy_count(),
            commands::superhero::get_superhero(),
            commands::superhero::super_duel(),
            commands::role_assign::create_role_assign(),
            commands::settings::set_prefix(),
        ],
        prefix_options: poise::PrefixFrameworkOptions {
            dynamic_prefix: Some(|ctx| Box::pin(commands::settings::get_prefix(ctx))),
            edit_tracker: Some(Arc::new(poise::EditTracker::for_timespan(
                Duration::from_secs(3600),
            ))),
            ..Default::default()
        },
        // The global error handler for all error cases that may occur
        on_error: |error| Box::pin(on_error(error)),
        // This code is run before every command
        pre_command: |ctx| {
            Box::pin(async move {
                println!("Executing command {}...", ctx.command().qualified_name);
            })
        },
        // This code is run after a command if it was successful (returned Ok)
        post_command: |ctx| {
            Box::pin(async move {
                println!(
                    "command {} executed by {} at {}",
                    ctx.command().qualified_name,
                    ctx.author().id,
                    ctx.created_at().timestamp()
                );

                println!("{} command finished!", ctx.command().qualified_name);

                create_command_history_item(ctx).await;
            })
        },
        // Every command invocation must pass this check to continue execution
        command_check: Some(|ctx| {
            Box::pin(async move {
                if ctx.author().id == 123456789 {
                    return Ok(false);
                }
                Ok(true)
            })
        }),
        // Enforce command checks even for owners (enforced by default)
        // Set to true to bypass checks, which is useful for testing
        skip_checks_for_owners: false,
        event_handler: |_ctx, event, _framework, _data| {
            Box::pin(commands::event_handler::event_handler(
                _ctx, event, _framework, _data,
            ))
        },
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                println!("Logged in as {}", _ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    giphy_api: api,
                    superhero_api: super_api,
                    database: database,
                    gardy_count: AtomicU32::new(0),
                    luxe_count: AtomicU32::new(0),
                })
            })
        })
        .options(options)
        .build();

    let token = var("DISCORD_TOKEN").expect("Missing `DISCORD_TOKEN` env var");
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();
    Ok(())
}

pub async fn create_command_history_item(ctx: Context<'_>) {
    println!("ctx: {:?}", ctx.command().qualified_name);

    let ch = CommandHistory {
        id: None,
        user_id: ctx.author().id.into(),
        command_name: ctx.command().qualified_name.clone(),
        executed_at: ctx.created_at().to_utc(),
    };

    ctx.data()
        .database
        .create_command_entry(ch)
        .await
        .expect("error creating db command history item");
}
