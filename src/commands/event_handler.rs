use once_cell::sync::Lazy;
use rand::{seq::SliceRandom, Rng};
use regex::Regex;
use std::str::FromStr;
use std::time::Duration;

use giphy::v1::gifs::RandomRequest;
use giphy::v1::r#async::RunnableAsyncRequest;

use poise::serenity_prelude::{self as serenity, CreateAttachment};

use crate::{
    database::models::CommandHistory, Data, Error,
};
use crate::commands::role_assign::update_all_guild_role_menus;

/// Types of responses the bot can give for a meme trigger.
enum TriggerResponse {
    Giphy(&'static [&'static str]),
    Quote(&'static [&'static str]),
    File(&'static str),
    Url(&'static str),
    /// For the complex "shapes" logic which has nested probabilities.
    Shapes,
}

struct MemeTrigger {
    name: &'static str,
    /// The regex pattern to match.
    pattern: &'static str,
    response: TriggerResponse,
    /// Probability of trigger (1 in N).
    chance: u32,
}

/// Compiled regexes for the triggers. 
/// We keep these separate so we can iterate over them alongside the trigger metadata.
static COMPILED_TRIGGERS: Lazy<Vec<(Regex, &'static MemeTrigger)>> = Lazy::new(|| {
    let triggers = vec![
        MemeTrigger {
            name: "99_quotes",
            pattern: r"99!",
            response: TriggerResponse::Quote(&[
                "I'm the human form of the 💯 emoji.",
                "Bingpot!",
                "Cool. Cool cool cool cool cool cool cool, no doubt no doubt no doubt no doubt.",
            ]),
            chance: 1,
        },
        MemeTrigger {
            name: "gardy_time",
            pattern: r"gardy.?time|grady.?time|tobey.?time",
            response: TriggerResponse::Giphy(&["time"]),
            chance: 4,
        },
        MemeTrigger {
            name: "luxe_time",
            pattern: r"luxe.?time",
            response: TriggerResponse::Giphy(&["bathroom", "shower"]),
            chance: 4,
        },
        MemeTrigger {
            name: "shapes",
            pattern: r"shape|shapes",
            response: TriggerResponse::Shapes,
            chance: 1, // Logic handled internally
        },
        MemeTrigger {
            name: "antiquing",
            pattern: r"antiquing",
            response: TriggerResponse::Url("https://media2.giphy.com/media/xuDHhHcCR0rew/source.gif"),
            chance: 1,
        },
        MemeTrigger {
            name: "my_problem",
            pattern: r"i missed the part where that's my problem",
            response: TriggerResponse::File("resources/my_problem.mp4"),
            chance: 1,
        },
        MemeTrigger {
            name: "cranberries",
            pattern: r"get these cranberries",
            response: TriggerResponse::File("resources/cranberries.mp4"),
            chance: 1,
        },
    ];

    triggers
        .into_iter()
        .map(|t| (Regex::new(&format!("(?i){}", t.pattern)).unwrap(), Box::leak(Box::new(t)) as &'static MemeTrigger))
        .collect()
});

pub async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Message { new_message } => {
            if new_message.author.bot {
                return Ok(());
            }

            for (regex, trigger) in COMPILED_TRIGGERS.iter() {
                if regex.is_match(&new_message.content) {
                    
                    // Specific logic for Shapes
                    if let TriggerResponse::Shapes = trigger.response {
                        let (is_100, is_10) = {
                            let mut rng = rand::thread_rng();
                            (rng.gen_ratio(1, 100), rng.gen_ratio(1, 10))
                        };

                        if is_100 {
                            new_message.channel_id.say(&ctx.http, "https://media0.giphy.com/media/v1.Y2lkPTc5MGI3NjExMHNjbm13c3kwc2s4M3hkOWRxanQzZWhwY2MzbGhvNjA0Yzc1YWd0MCZlcD12MV9pbnRlcm5hbF9naWZfYnlfaWQmY3Q9Zw/P3ataVxTgFOimQeC0S/giphy.gif").await?;
                        } else if is_10 {
                            let response = RandomRequest::new()
                                .with_tag("shape-math-confused")
                                .send_to(&data.giphy_api)
                                .await?;
                            new_message.channel_id.say(&ctx.http, response.data.embed_url).await?;
                        }
                        continue;
                    }

                    // Standard chance check
                    let should_trigger = {
                        let mut rng = rand::thread_rng();
                        trigger.chance == 1 || rng.gen_ratio(1, trigger.chance)
                    };

                    if !should_trigger {
                        continue;
                    }

                    match trigger.response {
                        TriggerResponse::Giphy(tags) => {
                            let tag = {
                                let mut rng = rand::thread_rng();
                                tags.choose(&mut rng).unwrap().to_string()
                            };
                            let response = RandomRequest::new()
                                .with_tag(&tag)
                                .send_to(&data.giphy_api)
                                .await?;
                            new_message.channel_id.say(&ctx.http, response.data.embed_url).await?;
                        }
                        TriggerResponse::Quote(quotes) => {
                            let quote = {
                                let mut rng = rand::thread_rng();
                                *quotes.choose(&mut rng).unwrap()
                            };
                            new_message.channel_id.say(&ctx.http, quote).await?;
                        }
                        TriggerResponse::Url(url) => {
                            new_message.channel_id.say(&ctx.http, url).await?;
                        }
                        TriggerResponse::File(path) => {
                            let attachment = CreateAttachment::path(path).await?;
                            new_message.channel_id.send_message(&ctx.http, serenity::CreateMessage::default().add_file(attachment)).await?;
                        }
                        TriggerResponse::Shapes => unreachable!(),
                    }

                    // Log to database
                    let ch = CommandHistory {
                        id: None,
                        user_id: new_message.author.id.into(),
                        guild_id: new_message.guild_id.unwrap().into(),
                        command_name: trigger.name.to_string(),
                        executed_at: new_message.timestamp.to_utc(),
                    };
                    data.database.create_command_entry(ch).await?;
                }
            }
        }

        serenity::FullEvent::InteractionCreate { interaction } => {
            if let Some(component) = interaction.as_message_component() {
                let guild_id = component.guild_id.unwrap();
                
                match component.data.custom_id.as_str() {
                    "open_role_menu" => {
                        let managed = data.database.get_managed_roles_for_guild(guild_id.into()).await?;
                        let guild = guild_id.to_guild_cached(&ctx).unwrap().clone();
                        let member = guild_id.member(&ctx, component.user.id).await?;
                        
                        let mut options = Vec::new();
                        for (role_id, label, emoji) in managed {
                            let r_id = serenity::RoleId::new(role_id as u64);
                            // Count members with this role
                            let count = guild.members.values()
                                .filter(|m| m.roles.contains(&r_id))
                                .count();
                            
                            let mut option = serenity::CreateSelectMenuOption::new(
                                format!("{} ({} members)", label, count),
                                role_id.to_string(),
                            );

                            // Pre-select if the user already has this role
                            if member.roles.contains(&r_id) {
                                option = option.default_selection(true);
                            }

                            if let Some(e) = emoji {
                                let resolved = crate::commands::role_assign::resolve_emoji(ctx, guild_id, e).await;
                                if let Ok(reaction) = serenity::ReactionType::from_str(&resolved) {
                                    option = option.emoji(reaction);
                                }
                            }
                            
                            options.push(option);
                        }

                        if options.is_empty() {
                            component.create_response(&ctx, serenity::CreateInteractionResponse::Message(
                                serenity::CreateInteractionResponseMessage::new()
                                    .content("No roles are currently configured for management.")
                                    .ephemeral(true)
                            )).await?;
                            return Ok(());
                        }

                        let num_options = options.len();
                        let menu = serenity::CreateSelectMenu::new("role_selection", serenity::CreateSelectMenuKind::String { options })
                            .placeholder("Select roles to JOIN (deselect to leave)")
                            .min_values(0)
                            .max_values(num_options as u8);

                        component.create_response(&ctx, serenity::CreateInteractionResponse::Message(
                            serenity::CreateInteractionResponseMessage::new()
                                .content("Select the game channels you want to see:")
                                .components(vec![serenity::CreateActionRow::SelectMenu(menu)])
                                .ephemeral(true)
                        )).await?;
                    }
                    "role_selection" => {
                        if let serenity::ComponentInteractionDataKind::StringSelect { values } = &component.data.kind {
                            let managed = data.database.get_managed_roles_for_guild(guild_id.into()).await?;
                            let selected_ids: Vec<i64> = values.iter().map(|v| v.parse().unwrap()).collect();
                            
                            let mut member = guild_id.member(&ctx, component.user.id).await?;
                            let mut new_roles = member.roles.clone();

                            for (role_id, _, _) in managed {
                                let r_id = serenity::RoleId::new(role_id as u64);
                                if selected_ids.contains(&role_id) {
                                    if !new_roles.contains(&r_id) {
                                        new_roles.push(r_id);
                                    }
                                } else {
                                    new_roles.retain(|&r| r != r_id);
                                }
                            }

                            let result = member.edit(&ctx, serenity::EditMember::default().roles(new_roles)).await;

                            let response_text = match result {
                                Ok(_) => "✅ Successfully updated your roles! This message will disappear in 5 seconds.".to_string(),
                                Err(e) => format!("❌ Failed to update roles: {}. Ensure the bot's role is HIGHER than the managed roles!", e),
                            };

                            // Update the original ephemeral message to remove the menu and show success
                            component.create_response(&ctx, serenity::CreateInteractionResponse::UpdateMessage(
                                serenity::CreateInteractionResponseMessage::new()
                                    .content(response_text)
                                    .components(vec![]) // Remove the select menu
                            )).await?;

                            // Spawn a task to delete the message after 5 seconds
                            let http = ctx.http.clone();
                            let interaction_token = component.token.clone();
                            tokio::spawn(async move {
                                tokio::time::sleep(Duration::from_secs(5)).await;
                                let _ = http.delete_original_interaction_response(&interaction_token).await;
                            });
                        }
                    }
                    _ => {}
                }
            }
        }

        serenity::FullEvent::GuildMemberUpdate { old_if_available: _, new: _, event } => {
            tokio::time::sleep(Duration::from_millis(500)).await;
            let guild_id = event.guild_id;
            update_all_guild_role_menus(ctx, data, guild_id).await?;
        }

        serenity::FullEvent::MessageDelete { channel_id: _, guild_id: _, deleted_message_id } => {
            let m_id = i64::from(*deleted_message_id);
            if data.database.is_menu_message(m_id).await? {
                data.database.remove_active_menu(m_id).await?;
                println!("Cleaned up role menu {} from database.", m_id);
            }
        }

        serenity::FullEvent::CacheReady { guilds } => {
            for guild_id in guilds {
                let g_id = i64::from(*guild_id);
                if data.database.get_guild(g_id).await?.is_none() {
                    data.database.create_guild(g_id).await?;
                }
            }
        }

        _ => {}
    }
    Ok(())
}
