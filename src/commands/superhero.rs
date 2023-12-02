use poise::serenity_prelude::Color;

use crate::superhero_api::get_random_superhero;

use crate::superhero_api::model::Powerstats;
use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn get_superhero(ctx: Context<'_>) -> Result<(), Error> {
    let response = get_random_superhero().await?;

    let stats = &response.powerstats;

    let fields = vec![
        ("Intelligence", stats.intelligence.to_string(), true),
        ("Strength", stats.strength.to_string(), true),
        ("Speed", stats.speed.to_string(), true),
        ("Durability", stats.durability.to_string(), true),
        ("Power", stats.power.to_string(), true),
        ("Combat", stats.combat.to_string(), true),
    ];

    let mut color: Color = Color::DARK_GREEN;
    if response.biography.alignment == "bad" {
        color = Color::DARK_RED;
    }

    let _reply = ctx
        .send(|b| {
            b.embed(|b| {
                b.title(&response.name)
                    .color(color)
                    .fields(fields)
                    .field("", "---------------------------------------", false)
                    .field("Overall", Powerstats::overall(stats), true)
                    .image(&response.image.url)
            })
        })
        .await?;

    Ok(())
}
