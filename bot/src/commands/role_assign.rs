use std::str::FromStr;

use color_eyre::eyre::{eyre, Report};
use poise::serenity_prelude::{
    self as serenity, ButtonStyle, CreateSelectMenuKind, CreateSelectMenuOption,
};

use crate::{Context, Error};

pub enum RoleAssignmentButton {
    AssignRole,
    AddRole,
    Unknown,
}

impl From<String> for RoleAssignmentButton {
    fn from(value: String) -> Self {
        match value.as_str() {
            "assign_role" => Self::AssignRole,
            "add_role" => Self::AddRole,
            _ => Self::Unknown,
        }
    }
}

impl From<RoleAssignmentButton> for String {
    fn from(value: RoleAssignmentButton) -> Self {
        match value {
            RoleAssignmentButton::AssignRole => String::from("assign_role"),
            RoleAssignmentButton::AddRole => String::from("add_role"),
            RoleAssignmentButton::Unknown => String::from(""),
        }
    }
}

#[poise::command(slash_command, guild_only = true)]
pub async fn create_role_assign(
    ctx: Context<'_>,
    #[description = "What channel do you want the role assigner in?"]
    #[channel_types("Text")]
    channel: serenity::Channel,
) -> Result<(), Error> {
    let guild = ctx
        .data()
        .database
        .get_guild(ctx.guild_id().unwrap().into())
        .await?
        .unwrap();

    // if let Some(r_id) = guild.role_assign_id {
    //     let role_assign = ctx.data().database.get_role_assign(r_id).await?;

    //     if role_assign.clone().is_none() {
    //         println!("no role assign table item found");
    //     }

    //     let c = ctx.guild().unwrap().channels.iter().find_map(|(&k, v)| {
    //         if role_assign.clone().unwrap().channel == i64::from(k) {
    //             Some(v.clone())
    //         } else {
    //             None
    //         }
    //     });

    //     ctx.say(format!("Role assignment already exists in {}", c.unwrap()))
    //         .await?;

    //     return Ok(());
    // }

    let embed = serenity::CreateEmbed::new().title("Role Assign".to_string());

    serenity::ButtonStyle::Unknown(255);

    let components = vec![
        serenity::CreateActionRow::Buttons(vec![serenity::CreateButton::new(
            RoleAssignmentButton::AssignRole,
        )
        .label("Assign Role")
        .style(serenity::ButtonStyle::Primary)]),
        serenity::CreateActionRow::Buttons(vec![serenity::CreateButton::new(
            RoleAssignmentButton::AddRole,
        )
        .label("Add Role")
        .style(serenity::ButtonStyle::Danger)]),
    ];

    let reply = poise::CreateReply::default()
        .embed(embed.clone())
        .components(components);

    ctx.send(reply).await?;

    // let role_assignment = ctx
    //     .data()
    //     .database
    //     .create_role_assign(channel.id().into(), None)
    //     .await?;

    // let update_guild = guild.with_role_assign_id(role_assignment.id.unwrap());
    // ctx.data().database.update_guild(update_guild).await?;
    Ok(())
}
