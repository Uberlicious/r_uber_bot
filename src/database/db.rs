use crate::Error;
use color_eyre::eyre::Report;
use sqlx::{SqlitePool, Row};

use crate::database::models::Guild;

use super::models::{CommandHistory, RoleAssign};

pub struct Database {
    pub pool: SqlitePool,
}

impl Database {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    #[allow(dead_code)]
    pub async fn get_guilds(&self) -> Result<Vec<Guild>, Report> {
        let pool = self.pool.clone();

        let guilds = sqlx::query_as::<_, Guild>("select guild_id, prefix, role_assign_id from guilds")
            .fetch_all(&pool)
            .await?;

        Ok(guilds)
    }

    pub async fn get_guild(&self, guild_id: i64) -> Result<Option<Guild>, Report> {
        let pool = self.pool.clone();

        let guild = sqlx::query_as::<_, Guild>("select guild_id, prefix, role_assign_id from guilds where guild_id = ?")
            .bind(guild_id)
            .fetch_optional(&pool)
            .await?;

        Ok(guild)
    }

    pub async fn create_guild(&self, guild_id: i64) -> Result<Guild, Report> {
        let pool = self.pool.clone();

        let new_guild = Guild::new(guild_id);

        sqlx::query("insert into guilds (guild_id, prefix) values (?, ?)")
            .bind(new_guild.guild_id)
            .bind(&new_guild.prefix)
            .execute(&pool)
            .await?;

        Ok(new_guild)
    }

    pub async fn update_guild(&self, guild: Guild) -> Result<(), Error> {
        let pool = self.pool.clone();

        sqlx::query("update guilds set prefix = ?, role_assign_id = ? where guild_id = ?")
            .bind(&guild.prefix)
            .bind(guild.role_assign_id)
            .bind(guild.guild_id)
            .execute(&pool)
            .await?;

        Ok(())
    }

    #[allow(dead_code)]
    pub async fn get_role_assign(&self, id: i64) -> Result<Option<RoleAssign>, Report> {
        let pool = self.pool.clone();

        let row = sqlx::query("select id, channel, roles from role_assign where id = ?")
            .bind(id)
            .fetch_optional(&pool)
            .await?;

        if let Some(row) = row {
            let id: i64 = row.get(0);
            let channel: i64 = row.get(1);
            let roles_str: Option<String> = row.get(2);
            let roles = roles_str.and_then(|s| serde_json::from_str(&s).ok());
            Ok(Some(RoleAssign {
                id: Some(id),
                channel,
                roles,
            }))
        } else {
            Ok(None)
        }
    }

    #[allow(dead_code)]
    pub async fn create_role_assign(
        &self,
        channel_id: i64,
        roles: Option<Vec<i64>>,
    ) -> Result<RoleAssign, Report> {
        let pool = self.pool.clone();
        let roles_str = roles.as_ref().map(|r| serde_json::to_string(r).unwrap());

        let result = sqlx::query("insert into role_assign (channel, roles) values (?, ?)")
            .bind(channel_id)
            .bind(roles_str)
            .execute(&pool)
            .await?;
        
        let id = result.last_insert_rowid();

        Ok(RoleAssign {
            id: Some(id),
            channel: channel_id,
            roles,
        })
    }

    #[allow(dead_code)]
    pub async fn add_managed_role(&self, guild_id: i64, role_id: i64, label: &str, emoji: Option<&str>) -> Result<(), Report> {
        let pool = self.pool.clone();
        sqlx::query("INSERT OR REPLACE INTO managed_roles (guild_id, role_id, label, emoji) VALUES (?, ?, ?, ?)")
            .bind(guild_id)
            .bind(role_id)
            .bind(label)
            .bind(emoji)
            .execute(&pool)
            .await?;
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn remove_managed_role(&self, guild_id: i64, role_id: i64) -> Result<(), Report> {
        let pool = self.pool.clone();
        sqlx::query("DELETE FROM managed_roles WHERE guild_id = ? AND role_id = ?")
            .bind(guild_id)
            .bind(role_id)
            .execute(&pool)
            .await?;
        Ok(())
    }

    pub async fn get_managed_roles(&self) -> Result<Vec<(i64, i64, String, Option<String>)>, Report> {
        let pool = self.pool.clone();
        let rows = sqlx::query("SELECT guild_id, role_id, label, emoji FROM managed_roles")
            .fetch_all(&pool)
            .await?;
        
        Ok(rows.into_iter().map(|r| (r.get(0), r.get(1), r.get(2), r.get(3))).collect())
    }

    pub async fn get_managed_roles_for_guild(&self, guild_id: i64) -> Result<Vec<(i64, String, Option<String>)>, Report> {
        let pool = self.pool.clone();
        let rows = sqlx::query("SELECT role_id, label, emoji FROM managed_roles WHERE guild_id = ?")
            .bind(guild_id)
            .fetch_all(&pool)
            .await?;
        
        Ok(rows.into_iter().map(|r| (r.get(0), r.get(1), r.get(2))).collect())
    }

    #[allow(dead_code)]
    pub async fn add_active_menu(&self, guild_id: i64, channel_id: i64, message_id: i64) -> Result<(), Report> {
        let pool = self.pool.clone();
        sqlx::query("INSERT INTO active_role_menus (guild_id, channel_id, message_id) VALUES (?, ?, ?)")
            .bind(guild_id)
            .bind(channel_id)
            .bind(message_id)
            .execute(&pool)
            .await?;
        Ok(())
    }

    pub async fn remove_active_menu(&self, message_id: i64) -> Result<(), Report> {
        let pool = self.pool.clone();
        sqlx::query("DELETE FROM active_role_menus WHERE message_id = ?")
            .bind(message_id)
            .execute(&pool)
            .await?;
        Ok(())
    }

    pub async fn is_menu_message(&self, message_id: i64) -> Result<bool, Report> {
        let pool = self.pool.clone();
        let row = sqlx::query("SELECT 1 FROM active_role_menus WHERE message_id = ?")
            .bind(message_id)
            .fetch_optional(&pool)
            .await?;
        Ok(row.is_some())
    }

    pub async fn get_active_menus_for_guild(&self, guild_id: i64) -> Result<Vec<(i64, i64)>, Report> {
        let pool = self.pool.clone();
        let rows = sqlx::query("SELECT channel_id, message_id FROM active_role_menus WHERE guild_id = ?")
            .bind(guild_id)
            .fetch_all(&pool)
            .await?;
        
        Ok(rows.into_iter().map(|r| (r.get(0), r.get(1))).collect())
    }

    pub async fn create_command_entry(&self, command: CommandHistory) -> Result<(), Report> {
        let pool = self.pool.clone();

        sqlx::query(
            "insert into command_history (user_id, guild_id, command_name, executed_at) values (?, ?, ?, ?)",
        )
        .bind(command.user_id)
        .bind(command.guild_id)
        .bind(&command.command_name)
        .bind(command.executed_at)
        .execute(&pool)
        .await?;

        Ok(())
    }
}
