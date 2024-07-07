use std::path::Prefix;

use crate::{Context, Error, PartialContext};

pub async fn get_prefix(ctx: PartialContext<'_>) -> Result<Option<String>, Error> {
    let guild_id = ctx.guild_id;

    if guild_id.is_none() {
        return Ok(Some("u!".into()));
    }

    let guild = ctx
        .data
        .database
        .get_guild(i64::from(guild_id.unwrap()))
        .await?;

    if let Some(guild) = guild.clone() {
        if guild.prefix.is_some() {
            return Ok(guild.prefix);
        }
    }

    Ok(Some("u!".into()))
}

#[poise::command(slash_command, guild_only = true)]
pub async fn set_prefix(
    ctx: Context<'_>,
    #[description = "What would you like the new bot prefix to be?:"] prefix: String,
) -> Result<(), Error> {
    // guild only command, should always unwrap
    let guild_id = i64::from(ctx.guild().unwrap().id);

    println!("update guild prefix: {:?}", prefix);

    let guild = ctx.data().database.get_guild(guild_id).await?;

    println!("guild: {:?}", guild);

    if let Some(guild) = guild {
        let updated = guild.clone().with_prefix(&prefix);

        println!("updated guild: {:?}", updated);
        ctx.data().database.update_guild(updated).await?;

        match guild.prefix {
            Some(p) => {
                ctx.say(format!("Prefix changed from {} to {}", p, prefix))
                    .await?;
            }
            None => {
                ctx.say(format!("Prefix changed from u! to {}", prefix))
                    .await?;
            }
        }
    }

    // let db = ctx.data().database.pool.clone();
    // let guild = sqlx::query_as!(
    //     dbGuild,
    //     "select * from guilds where guild_id = $1",
    //     i64::from(ctx.guild().unwrap().id)
    // )
    // .fetch_optional(&db)
    // .await?;

    // println!("Guild: {:?}", guild);

    // if let Some(g) = Option::as_ref(&localGuild) {
    //     let updated = g.clone().with_prefix(prefix);

    //     println!("updated {:?}", updated)
    // }

    // data.database.update_guild(Guild, prefix)
    // Guild::new()

    Ok(())
}
