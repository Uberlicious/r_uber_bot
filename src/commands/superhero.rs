use crate::superheroapi::get_num_ids;

use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn get_superhero(_ctx: Context<'_>) -> Result<(), Error> {
    let response = get_num_ids().await?;
    println!("{:?}", response);

    Ok(())
}
