use crate::{Context, Error};
use poise::serenity_prelude::{self as serenity};
use crate::commands::role_assign::{update_all_guild_role_menus, resolve_emoji};

/// Command group for managing roles
#[poise::command(slash_command, guild_only = true, subcommands("add", "remove"), default_member_permissions = "ADMINISTRATOR")]
pub async fn manage_roles(_: Context<'_>) -> Result<(), Error> { Ok(()) }

/// Manually add a role to the managed list
#[poise::command(slash_command)]
pub async fn add(
    ctx: Context<'_>,
    #[description = "The role to manage"] role: serenity::Role,
    #[description = "The label to show in the menu"] label: String,
    #[description = "The emoji to display (Static emojis only!)"] emoji: Option<String>,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    
    // Auto-detect if the user accidentally put the emoji in the label field
    let mut final_label = label.clone();
    let mut search_emoji = emoji;

    if search_emoji.is_none() && final_label.contains(':') {
        // If label looks like an emoji and emoji is none, swap them
        search_emoji = Some(final_label.clone());
        final_label = role.name.clone(); // Fallback label to role name
    }

    if let Some(ref e) = search_emoji {
        if e.starts_with("<a:") {
            ctx.say("❌ Error: Animated emojis are not supported for role menus. Please use a static emoji.").await?;
            return Ok(());
        }
    }

    // Resolve emoji before saving
    let resolved_emoji = match search_emoji {
        Some(e) => Some(resolve_emoji(ctx.serenity_context(), guild_id, e).await),
        None => None,
    };

    ctx.data().database.add_managed_role(
        guild_id.into(),
        role.id.into(),
        &final_label,
        resolved_emoji.as_deref()
    ).await?;

    update_all_guild_role_menus(ctx.serenity_context(), ctx.data(), guild_id).await?;

    ctx.say(format!("Successfully added role **{}** to the managed list.", final_label)).await?;
    Ok(())
}

/// Remove a role from the managed list
#[poise::command(slash_command)]
pub async fn remove(
    ctx: Context<'_>,
    #[description = "The role to remove"] role: serenity::Role,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    ctx.data().database.remove_managed_role(guild_id.into(), role.id.into()).await?;

    update_all_guild_role_menus(ctx.serenity_context(), ctx.data(), guild_id).await?;

    ctx.say(format!("Successfully removed role **{}** from the managed list.", role.name)).await?;
    Ok(())
}

/// Create Game Channel.
#[poise::command(slash_command)]
pub async fn create_channel(
    ctx: Context<'_>,
    #[description = "What do you want this new channel to be named"] channel_name: String,
    #[description = "What emoji do you want set for the icon? (Static emojis only!)"] icon: String,
) -> Result<(), Error> {
    let guild = ctx.guild().ok_or("Must be in a guild")?.clone();
    let guild_id = guild.id;

    if icon.starts_with("<a:") {
        ctx.say("❌ Error: Animated emojis are not supported for channel icons. Please use a static emoji.").await?;
        return Ok(());
    }

    // Check if anything already exists
    let existing_channel = guild.channels.values().find(|c| c.name == channel_name);
    if existing_channel.is_some() {
        ctx.say(format!("Channel {} already exists", channel_name)).await?;
        return Ok(());
    }

    let existing_role = guild.roles.values().find(|r| r.name == channel_name);
    if existing_role.is_some() {
        ctx.say(format!("Role {} already exists", channel_name)).await?;
        return Ok(());
    }

    let perms = serenity::Permissions::MANAGE_ROLES 
        | serenity::Permissions::CREATE_INSTANT_INVITE
        | serenity::Permissions::CHANGE_NICKNAME
        | serenity::Permissions::READ_MESSAGE_HISTORY
        | serenity::Permissions::SEND_MESSAGES
        | serenity::Permissions::EMBED_LINKS
        | serenity::Permissions::ATTACH_FILES
        | serenity::Permissions::MENTION_EVERYONE
        | serenity::Permissions::USE_EXTERNAL_EMOJIS
        | serenity::Permissions::CONNECT
        | serenity::Permissions::SPEAK
        | serenity::Permissions::USE_VAD;

    let new_role = guild.create_role(&ctx, serenity::EditRole::default()
        .name(&channel_name)
        .permissions(perms)
        .mentionable(true)
    ).await?;

    let bots_role = guild.roles.values().find(|r| r.name == "Bots");

    let mut overwrites = vec![
        serenity::PermissionOverwrite {
            allow: serenity::Permissions::empty(),
            deny: serenity::Permissions::VIEW_CHANNEL,
            kind: serenity::PermissionOverwriteType::Role(guild.id.everyone_role()),
        },
        serenity::PermissionOverwrite {
            allow: serenity::Permissions::VIEW_CHANNEL,
            deny: serenity::Permissions::empty(),
            kind: serenity::PermissionOverwriteType::Role(new_role.id),
        },
    ];

    if let Some(bots) = bots_role {
        overwrites.push(serenity::PermissionOverwrite {
            allow: serenity::Permissions::VIEW_CHANNEL,
            deny: serenity::Permissions::empty(),
            kind: serenity::PermissionOverwriteType::Role(bots.id),
        });
    }

    let category = guild.create_channel(&ctx, serenity::CreateChannel::new(&channel_name)
        .kind(serenity::ChannelType::Category)
        .permissions(overwrites.clone())
    ).await?;

    let lower_cn = channel_name.to_lowercase();
    
    guild.create_channel(&ctx, serenity::CreateChannel::new(format!("general-chat-{}", lower_cn))
        .kind(serenity::ChannelType::Text)
        .category(category.id)
    ).await?;

    guild.create_channel(&ctx, serenity::CreateChannel::new(format!("General-{}", lower_cn))
        .kind(serenity::ChannelType::Voice)
        .category(category.id)
    ).await?;

    // Resolve icon before saving
    let resolved_emoji = resolve_emoji(ctx.serenity_context(), guild_id, icon.clone()).await;

    ctx.data().database.add_managed_role(
        guild.id.into(),
        new_role.id.into(),
        &channel_name,
        Some(&resolved_emoji)
    ).await?;

    update_all_guild_role_menus(ctx.serenity_context(), ctx.data(), guild_id).await?;

    ctx.say(format!("Successfully created channel and role! Emote: {} <@&{}>", resolved_emoji, new_role.id)).await?;

    Ok(())
}

/// Register application commands globally or in this guild.
#[poise::command(prefix_command, hide_in_help)]
pub async fn sync(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}
