use poise::serenity_prelude::Color;

use crate::superhero_api::get_random_superhero;

use crate::superhero_api::model::Powerstats;
use crate::{Context, Error};
use poise::serenity_prelude as serenity;

async fn get_superhero_embed(user: Option<serenity::User>) -> Result<serenity::CreateEmbed, Error> {
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

    let mut title = response.name;
    match user {
        Some(x) => title.push_str(format!(" | {}", x.name).as_str()),
        None => {}
    }

    let embed = serenity::CreateEmbed::new()
        .title(title)
        .color(color)
        .fields(fields)
        .field("", "---------------------------------------", false)
        .field("Overall", Powerstats::overall(stats).to_string(), true)
        .image(&response.image.url);

    Ok(embed)
}

#[poise::command(slash_command)]
pub async fn get_superhero(ctx: Context<'_>) -> Result<(), Error> {
    let reply = {
        let embed = get_superhero_embed(None).await?;

        poise::CreateReply::default().embed(embed)
    };

    ctx.send(reply).await?;

    Ok(())
}

#[poise::command(slash_command, reuse_response)]
pub async fn super_duel(
    ctx: Context<'_>,
    #[description = "Who do you want to challenge?"] competitor: Option<serenity::User>,
) -> Result<(), Error> {
    let uuid_duel = ctx.id();

    let response: String;

    match competitor {
        Some(ref c) => {
            response = format!("{} has challenged {} to a test of fates!", ctx.author(), c)
        }
        None => {
            response = format!(
                "{} has put forth a challenge to anyone brave enough!",
                ctx.author()
            )
        }
    }

    let mut duration: u64 = 5;

    let msg = ctx
        .send(duel_message(uuid_duel, &response, Some(duration)))
        .await?;

    loop {
        duration -= 1;

        let button = serenity::ComponentInteractionCollector::new(ctx)
            .author_id(ctx.author().id)
            .channel_id(ctx.channel_id())
            .timeout(std::time::Duration::from_secs(1))
            .filter(move |mci| mci.data.custom_id == uuid_duel.to_string())
            // .filter(move |mci| mci.user.id == competitor.id)
            .await;

        match button {
            Some(mci) => {
                msg.edit(ctx, duel_message(uuid_duel, &response, None))
                    .await?;

                mci.create_response(ctx, serenity::CreateInteractionResponse::Acknowledge)
                    .await?;

                let reply = {
                    let embed = get_superhero_embed(Some(ctx.author().clone())).await?;

                    poise::CreateReply::default().embed(embed)
                };

                ctx.send(reply).await?;

                let reply: poise::CreateReply;

                match competitor {
                    Some(c) => {
                        reply = {
                            let embed = get_superhero_embed(Some(c)).await?;
                            poise::CreateReply::default().embed(embed)
                        }
                    }
                    None => {
                        reply = {
                            let embed = get_superhero_embed(Some(mci.user)).await?;
                            poise::CreateReply::default().embed(embed)
                        }
                    }
                }

                ctx.send(reply).await?;

                break;
            }
            None => {}
        }

        if duration == 0 {
            let timeout_response: String;
            match competitor {
                Some(c) => {
                    timeout_response =
                        format!("Duel request from {} to {} has timed out.", ctx.author(), c);
                }
                None => {
                    timeout_response =
                        format!("Duel request from {} has timed out.", ctx.author(),);
                }
            }
            msg.edit(ctx, duel_message(uuid_duel, &timeout_response, None))
                .await?;

            break;
        }

        msg.edit(ctx, duel_message(uuid_duel, &response, Some(duration)))
            .await?
    }

    Ok(())
}

fn duel_message(uuid: u64, response: &String, time_left: Option<u64>) -> poise::CreateReply {
    match time_left {
        Some(t) => {
            let components = vec![serenity::CreateActionRow::Buttons(vec![
                serenity::CreateButton::new(format!("{uuid}"))
                    .emoji('⚔')
                    .label(format!("Accept Duel: {}", t))
                    .style(serenity::ButtonStyle::Primary),
            ])];

            return poise::CreateReply::default()
                .content(response)
                .components(components);
        }
        None => {
            return poise::CreateReply::default()
                .content(response)
                .components(vec![])
        }
    }
}
