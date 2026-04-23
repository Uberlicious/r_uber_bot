// #![warn(clippy::str_to_string)]

use std::{
    env::{self},
    sync::Arc,
    time::Duration,
    str::FromStr,
};

use chrono::FixedOffset;
use database::db::Database;
use database::models::CommandHistory;
use log::info;

use color_eyre::{eyre::Report};

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

    let env = dotenvy::var("ENV").unwrap_or_else(|_| "prod".to_string());
    if env == "dev" {
        env::set_var("RUST_LOG", "debug")
    }

    pretty_env_logger::init();

    // giphy api
    let giphy_api_key = dotenvy::var("GIPHY_API_KEY").map_err(|e| Report::msg(format!("GIPHY_API_KEY must be set: {}", e)))?;
    let client_0_11 = reqwest_0_11::Client::new();
    let api = AsyncApi::new(giphy_api_key, client_0_11);

    // superhero api key
    let superhero_api_key =
        dotenvy::var("SUPERHERO_API_KEY").map_err(|e| Report::msg(format!("SUPERHERO_API_KEY must be set: {}", e)))?;
    let super_api = SuperheroApi::new(superhero_api_key);

    // database init
    let db_url = dotenvy::var("DATABASE_URL").map_err(|e| Report::msg(format!("DATABASE_URL must be set: {}", e)))?;
    let connection_options = sqlx::sqlite::SqliteConnectOptions::from_str(&db_url)?
        .create_if_missing(true);
    let pool = sqlx::SqlitePool::connect_with(connection_options).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let database = Database::new(pool);

    // FrameworkOptions contains all of poise's configuration option in one struct
    // Every option can be omitted to use its default value
    let options: poise::FrameworkOptions<Data, Error> = poise::FrameworkOptions {
        commands: vec![
            commands::management::sync(),
            commands::age::age(),
            commands::age::gardy_count(),
            commands::superhero::get_superhero(),
            commands::superhero::super_duel(),
            commands::role_assign::role_menu(),
            commands::role_assign::delete_role_menu(),
            commands::management::manage_roles(),
            commands::settings::set_prefix(),
            commands::misc::roll_dice(),
            commands::misc::random_teams(),
            commands::misc::frank(),
            commands::management::create_channel(),
        ],
        prefix_options: poise::PrefixFrameworkOptions {
            dynamic_prefix: Some(|ctx: PartialContext<'_>| Box::pin(commands::settings::get_prefix(ctx))),
            edit_tracker: Some(Arc::new(poise::EditTracker::for_timespan(
                Duration::from_secs(3600),
            ))),
            ..Default::default()
        },
        // The global error handler for all error cases that may occur
        on_error: |error: poise::FrameworkError<'_, Data, Error>| Box::pin(on_error(error)),
        // This code is run before every command
        pre_command: |ctx: Context<'_>| {
            Box::pin(async move {
                println!(
                    "command {} executed by {} at {}",
                    ctx.command().qualified_name,
                    ctx.author()
                        .global_name
                        .clone()
                        .unwrap_or(ctx.author().name.clone()),
                    ctx.created_at()
                        .naive_local()
                        .and_local_timezone(FixedOffset::west_opt(5 * 3600).unwrap())
                        .unwrap()
                        .to_rfc2822()
                );
            })
        },
        // This code is run after a command if it was successful (returned Ok)
        post_command: |ctx: Context<'_>| {
            Box::pin(async move {
                println!("{} command finished!", ctx.command().qualified_name);

                create_command_history_item(ctx).await;
            })
        },
        // Every command invocation must pass this check to continue execution
        command_check: Some(|ctx: Context<'_>| {
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
        event_handler: |ctx: &serenity::Context, event: &serenity::FullEvent, _framework: poise::FrameworkContext<'_, Data, Error>, data: &Data| {
            Box::pin(commands::event_handler::event_handler(
                ctx, event, _framework, data,
            ))
        },
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                println!("Logged in as {}", _ready.user.name);
                info!("logged in as {}", _ready.user.name);
                
                // If DEV_GUILD_ID is set in .env, register commands to that guild instantly.
                // Otherwise, register them globally (takes up to 1 hour).
                if let Ok(guild_id_str) = dotenvy::var("DEV_GUILD_ID") {
                    let guild_id = serenity::GuildId::new(guild_id_str.parse()?);
                    println!("Registering commands to guild {}...", guild_id);
                    poise::builtins::register_in_guild(ctx, &framework.options().commands, guild_id).await?;
                } else {
                    println!("Registering commands globally...");
                    poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                }

                Ok(Data {
                    giphy_api: api,
                    superhero_api: super_api,
                    database,
                })
            })
        })
        .options(options)
        .build();

    let token = dotenvy::var("DISCORD_TOKEN").map_err(|e| Report::msg(format!("DISCORD_TOKEN must be set: {}", e)))?;
    let intents =
        serenity::GatewayIntents::non_privileged() 
        | serenity::GatewayIntents::MESSAGE_CONTENT
        | serenity::GatewayIntents::GUILD_MEMBERS;

    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await
        .map_err(|e| Report::msg(format!("Failed to create client: {}", e)))?;

    client.start().await.map_err(|e| Report::msg(format!("Failed to start client: {}", e)))?;
    Ok(())
}

pub async fn create_command_history_item(ctx: Context<'_>) {
    let ch = CommandHistory {
        id: None,
        user_id: ctx.author().id.into(),
        guild_id: ctx.guild_id().unwrap().into(),
        command_name: ctx.command().qualified_name.clone(),
        executed_at: ctx.created_at().to_utc(),
    };

    ctx.data()
        .database
        .create_command_entry(ch)
        .await
        .expect("error creating db command history item");
}
