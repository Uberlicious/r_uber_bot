use poise::serenity_prelude::model::guild;
use poise::serenity_prelude::{Color, Emoji, EmojiId, Guild, ReactionType};

use crate::superhero_api::get_random_superhero;

use crate::superhero_api::model::Powerstats;
use crate::{Context, Error};
use poise::serenity_prelude as serenity;

fn get_superhero_embed() -> Result<poise::Embed, Error> {
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

    let embed = serenity::CreateEmbed::new()
        .title(&response.name)
        .color(color)
        .fields(fields)
        .field("", "---------------------------------------", false)
        .field("Overall", Powerstats::overall(stats).to_string(), true)
        .image(&response.image.url);
}

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

    let reply = {
        let embed = serenity::CreateEmbed::new()
            .title(&response.name)
            .color(color)
            .fields(fields)
            .field("", "---------------------------------------", false)
            .field("Overall", Powerstats::overall(stats).to_string(), true)
            .image(&response.image.url);

        poise::CreateReply::default().embed(embed)
    };

    ctx.send(reply).await?;

    Ok(())
}

#[poise::command(slash_command, reuse_response)]
pub async fn super_duel(
    ctx: Context<'_>,
    #[description = "Who do you want to challenge?"] competitor: serenity::User,
) -> Result<(), Error> {
    let guild = ctx.partial_guild().await.unwrap();
    let response = format!(
        "{} has challenged {} to a test of fates!",
        ctx.author(),
        competitor
    );

    let swords = guild
        .emojis
        .values()
        .for_each(|e| println!("name: {}", e.name));

    let reply = {
        let components = vec![serenity::CreateActionRow::Buttons(vec![
            serenity::CreateButton::new("duel")
                .emoji('⚔')
                .label("Accept Duel")
                .style(serenity::ButtonStyle::Primary),
        ])];

        poise::CreateReply::default()
            .content(response)
            .components(components)
    };

    ctx.send(reply).await?;
    Ok(())
}
