use crate::{Context, Error};
use poise::serenity_prelude::{self as serenity};
use rand::{seq::SliceRandom, Rng};
use giphy::v1::r#async::RunnableAsyncRequest;

/// Simulates rolling dice.
#[poise::command(slash_command)]
pub async fn roll_dice(
    ctx: Context<'_>,
    #[description = "The number of dice you want"] dice: u32,
    #[description = "The number of sides for each die"] sides: u32,
) -> Result<(), Error> {
    let results: Vec<String> = {
        let mut rng = rand::thread_rng();
        (0..dice)
            .map(|_| rng.gen_range(1..=sides).to_string())
            .collect()
    };
    ctx.say(results.join(", ")).await?;
    Ok(())
}

/// Split members in current voice chat into two random teams
#[poise::command(slash_command)]
pub async fn random_teams(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Must be in a guild")?;
    
    // Find the voice channel the user is in
    let channel_id = {
        let guild = guild_id.to_guild_cached(&ctx).ok_or("Guild not in cache")?;
        let voice_state = guild.voice_states.get(&ctx.author().id).ok_or("You must be in a voice channel")?;
        voice_state.channel_id.ok_or("You must be in a voice channel")?
    };

    let channel = channel_id
        .to_channel(&ctx)
        .await?
        .guild()
        .ok_or("Not a guild channel")?;
    
    let mut members = channel.members(&ctx)?;
    {
        let mut rng = rand::thread_rng();
        members.shuffle(&mut rng);
    }

    let half = members.len() / 2;
    let (team1, team2) = members.split_at(half);

    let team1_names: String = if team1.is_empty() { "null".to_string() } else { 
        team1.iter().map(|m| format!("<@{}>", m.user.id)).collect::<Vec<_>>().join("\n") 
    };
    let team2_names: String = if team2.is_empty() { "null".to_string() } else { 
        team2.iter().map(|m| format!("<@{}>", m.user.id)).collect::<Vec<_>>().join("\n") 
    };

    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::default()
            .title("Random Teams")
            .color(0x1777BF)
            .field("Team 1", team1_names, true)
            .field("\u{AD}", "\u{AD}", true)
            .field("Team 2", team2_names, true)
    ))
    .await?;

    Ok(())
}

/// 50/50 chance for yes or no answer.
#[poise::command(slash_command)]
pub async fn frank(ctx: Context<'_>) -> Result<(), Error> {
    let rand = rand::thread_rng().gen_range(0..=100);
    
    let query = if rand >= 51 {
        "Yes"
    } else if rand == 50 {
        "Maybe"
    } else {
        "No"
    };

    let response = giphy::v1::gifs::RandomRequest::new()
        .with_tag(query)
        .send_to(&ctx.data().giphy_api)
        .await?;

    ctx.say(format!("A coin flips through the air... {}", response.data.url)).await?;

    Ok(())
}
