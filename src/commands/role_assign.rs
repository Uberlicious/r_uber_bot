use crate::{Context, Error, Data};
use poise::serenity_prelude::{self as serenity};
use color_eyre::eyre::Report;

/// Resolves a string into a Discord emoji format.
pub async fn resolve_emoji(ctx: &serenity::Context, guild_id: serenity::GuildId, emoji_str: String) -> String {
    // 1. If it's already a full mention, return it as is
    if emoji_str.starts_with('<') && emoji_str.ends_with('>') {
        return emoji_str;
    }

    // 2. Clean the name (handle :name:, name, and name~1)
    let base_name = emoji_str.trim_matches(':')
        .split('~') // Handle the ~1, ~2 suffixes Discord adds
        .next()
        .unwrap_or("")
        .to_lowercase();
    
    if base_name.is_empty() { return emoji_str; }

    // Helper to search a list of emojis
    let find_in_list = |emojis: &std::collections::HashMap<serenity::EmojiId, serenity::Emoji>| {
        emojis.values().find(|e| e.name.to_lowercase() == base_name).map(|e| {
            let prefix = if e.animated { "a" } else { "" };
            format!("<{}:{}:{}>", prefix, e.name, e.id)
        })
    };

    // Check current guild first
    if let Some(guild) = ctx.cache.guild(guild_id) {
        if let Some(found) = find_in_list(&guild.emojis) {
            return found;
        }
    }

    // Check all other guilds in cache
    for g_id in ctx.cache.guilds() {
        if let Some(guild) = ctx.cache.guild(g_id) {
            if let Some(found) = find_in_list(&guild.emojis) {
                return found;
            }
        }
    }

    // 3. Last resort: Try fetching from the API for the local guild
    if let Ok(emojis) = ctx.http.get_emojis(guild_id).await {
        if let Some(e) = emojis.iter().find(|e| e.name.to_lowercase() == base_name) {
            let prefix = if e.animated { "a" } else { "" };
            return format!("<{}:{}:{}>", prefix, e.name, e.id);
        }
    }

    emoji_str
}

/// Builds the status embed for role management.
pub async fn build_role_menu_embed(ctx: &serenity::Context, data: &Data, guild_id: serenity::GuildId) -> Result<serenity::CreateEmbed, Report> {
    let managed = data.database.get_managed_roles_for_guild(guild_id.into()).await?;
    
    // Fetch all members from the API to ensure accurate counts
    let members = guild_id.members(ctx, Some(1000), None).await
        .map_err(|e| Report::msg(format!("Failed to fetch members: {}", e)))?;

    let mut role_column = String::new();
    let mut count_column = String::new();

    if managed.is_empty() {
        role_column = "_No channels configured._".to_string();
        count_column = "-".to_string();
    } else {
        for (role_id, label, emoji) in managed {
            let r_id = serenity::RoleId::new(role_id as u64);
            
            // Count members
            let count = members.iter()
                .filter(|m| m.roles.contains(&r_id))
                .count();
            
            let emote = match emoji {
                Some(e) => resolve_emoji(ctx, guild_id, e).await,
                None => "🔹".to_string(),
            };
                
            role_column.push_str(&format!("{} **{}**\n", emote, label));
            count_column.push_str(&format!("`{} members`\n", count));
        }
    }

    Ok(serenity::CreateEmbed::default()
        .title("Game Channel Management")
        .description("Click the button below to join or leave game channels!")
        .field("Game Channel", role_column, true)
        .field("Activity", count_column, true)
        .color(0x1777BF))
}

/// Updates all active role menu messages in a guild.
pub async fn update_all_guild_role_menus(ctx: &serenity::Context, data: &Data, guild_id: serenity::GuildId) -> Result<(), Error> {
    let menus = data.database.get_active_menus_for_guild(guild_id.into()).await?;
    if menus.is_empty() { return Ok(()); }

    let embed = build_role_menu_embed(ctx, data, guild_id).await?;

    for (channel_id, message_id) in menus {
        let channel = serenity::ChannelId::new(channel_id as u64);
        let message = serenity::MessageId::new(message_id as u64);
        
        let _ = channel.edit_message(ctx, message, serenity::EditMessage::default()
            .content("") 
            .embed(embed.clone())
        ).await;
    }

    Ok(())
}

/// Setup the role assignment menu in this channel.
#[poise::command(slash_command, guild_only = true, default_member_permissions = "ADMINISTRATOR")]
pub async fn role_menu(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let embed = build_role_menu_embed(ctx.serenity_context(), ctx.data(), guild_id).await?;

    let components = vec![
        serenity::CreateActionRow::Buttons(vec![
            serenity::CreateButton::new("open_role_menu")
                .label("Join / Leave Channels")
                .style(serenity::ButtonStyle::Primary),
        ])
    ];

    // Send as a standalone message instead of an interaction reply
    // to remove the "User used /role_menu" header.
    let msg = ctx.channel_id().send_message(ctx, 
        serenity::CreateMessage::default()
            .embed(embed)
            .components(components)
    ).await?;

    // Acknowledge the slash command ephemerally
    ctx.send(poise::CreateReply::default()
        .content("✅ Role menu has been posted!")
        .ephemeral(true)
    ).await?;

    ctx.data().database.add_active_menu(
        guild_id.into(),
        msg.channel_id.into(),
        msg.id.into()
    ).await?;

    Ok(())
}

/// Delete this role menu and cleanup the database.
#[poise::command(context_menu_command = "Delete Role Menu", guild_only = true, default_member_permissions = "ADMINISTRATOR")]
pub async fn delete_role_menu(ctx: Context<'_>, msg: serenity::Message) -> Result<(), Error> {
    let is_menu = ctx.data().database.is_menu_message(msg.id.into()).await?;
    
    if is_menu {
        msg.delete(&ctx).await?;
        ctx.data().database.remove_active_menu(msg.id.into()).await?;
        ctx.say("Role menu deleted and database cleaned up.").await?;
    } else {
        ctx.say("This message is not a registered role menu.").await?;
    }

    Ok(())
}

#[allow(dead_code)]
pub enum RoleAssignmentInteraction {
    OpenMenu,
    RoleSelect,
    Unknown,
}

impl From<String> for RoleAssignmentInteraction {
    fn from(s: String) -> Self {
        match s.as_str() {
            "open_role_menu" => Self::OpenMenu,
            "role_selection" => Self::RoleSelect,
            _ => Self::Unknown,
        }
    }
}
