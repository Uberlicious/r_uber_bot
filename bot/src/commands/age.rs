use std::sync::atomic::Ordering;

use crate::{Context, Error};
use poise::serenity_prelude as serenity;

// check when a user was created
#[poise::command(prefix_command, slash_command, track_edits)]
pub async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

// check how many times gardy time has occurred
#[poise::command(slash_command, ephemeral)]
pub async fn gardy_count(ctx: Context<'_>) -> Result<(), Error> {
    let count = ctx.data().gardy_count.load(Ordering::SeqCst);
    let response = format!("Gardy time has been triggered {count} times");
    ctx.say(response).await?;
    Ok(())
}
