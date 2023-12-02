use crate::superhero_api::get_random_superhero;
use poise::serenity_prelude as serenity;

use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn get_superhero(ctx: Context<'_>) -> Result<(), Error> {
    let response = get_random_superhero().await?;
    println!("{:?}", response);

    let stats = &response.powerstats;
    // let overall = usize::try_from(stats.intelligence).unwrap();
    let overall = "200".to_string();
    let blank = "".to_string();

    let fields = vec![
        ("Hero", &response.name, true),
        ("Intelligence", &stats.intelligence, true),
        ("Strength", &stats.strength, true),
        ("Speed", &stats.speed, true),
        ("Durability", &stats.durability, true),
        ("Power", &stats.power, true),
        ("Combat", &stats.combat, true),
        ("", &blank, true),
        ("Overall", &overall, true),
    ];

    // let title =

    let _reply = ctx
        .send(|b| {
            b.embed(|b| {
                b.title(&ctx.author().name)
                    .fields(fields)
                    .image(&response.image.url)
            })
        })
        .await?;

    Ok(())
}
