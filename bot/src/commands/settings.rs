use crate::{Context, Error};

use crate::database::Guild as dbGuild;

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
