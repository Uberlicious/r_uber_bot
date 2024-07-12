use crate::{Context, Error};
use poise::serenity_prelude as serenity;

#[poise::command(slash_command, guild_only = true)]
pub async fn create_role_assign(
    ctx: Context<'_>,
    #[description = "What channel do you want the role assigner in?"]
    #[channel_types("Text")]
    channel: serenity::Channel,
) -> Result<(), Error> {
    let guild = ctx
        .data()
        .database
        .get_guild(ctx.guild_id().unwrap().into())
        .await?;

    if let None = guild {
        ctx.say("Must be in a server!").await?;
    }

    // let g = guild.unwrap().with_role_assign_id(channel.id().into());
    // ctx.data().database.update_guild(g).await?;
    ctx.data()
        .database
        .create_role_assign(channel.id().into(), None)
        .await?;

    ctx.say(format!("Role assign channel set: {}", channel.to_string()))
        .await?;
    Ok(())
}
