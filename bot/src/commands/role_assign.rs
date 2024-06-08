use crate::{Context, Error};
use poise::serenity_prelude as serenity;

#[poise::command(slash_command, guild_only = true)]
pub async fn create_role_assign(
    ctx: Context<'_>,
    #[description = "What channel do you want the role assigner in?"]
    #[channel_types("Text")]
    channel: serenity::Channel,
) -> Result<(), Error> {
    ctx.say(format!("channel: {}", channel.to_string())).await?;
    Ok(())
}
