use crate::Error;
use color_eyre::eyre::Report;
use sqlx::{postgres::PgRow, PgPool};

use crate::database::models::Guild;

use super::models::RoleAssign;

pub struct Database {
    pub pool: PgPool,
}

impl Database {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_guilds(&self) -> Result<Vec<Guild>, Report> {
        let pool = self.pool.clone();

        let guilds = sqlx::query_as!(Guild, "select * from guilds")
            .fetch_all(&pool)
            .await?;

        Ok(guilds)
    }

    pub async fn get_guild(&self, guild_id: i64) -> Result<Option<Guild>, Report> {
        let pool = self.pool.clone();

        let guild = sqlx::query_as!(Guild, "select * from guilds where guild_id = $1", guild_id)
            .fetch_optional(&pool)
            .await?;

        Ok(guild)
    }

    pub async fn create_guild(&self, guild_id: i64) -> Result<Guild, Report> {
        let pool = self.pool.clone();

        let new_guild = Guild::new(guild_id);

        let result = sqlx::query_as!(
            Guild,
            "insert into guilds (guild_id, prefix) values ($1, $2)",
            new_guild.guild_id,
            new_guild.prefix
        )
        .execute(&pool)
        .await?;

        Ok(new_guild)
    }

    pub async fn update_guild(&self, guild: Guild) -> Result<(), Error> {
        let pool = self.pool.clone();

        sqlx::query_as!(
            Guild,
            "update guilds set prefix = $2, role_assign_id = $3 where guild_id = $1",
            guild.guild_id,
            guild.prefix,
            guild.role_assign_id,
        )
        .execute(&pool)
        .await?;

        Ok(())
    }

    pub async fn create_role_assign(
        &self,
        channel_id: i64,
        roles: Option<Vec<i64>>,
    ) -> Result<RoleAssign, Report> {
        let pool = self.pool.clone();

        let result = sqlx::query_as!(
            RoleAssign,
            "insert into role_assign (channel, roles) values ($1, $2) returning id, channel, roles",
            channel_id,
            roles.as_deref(),
        )
        .fetch_one(&pool)
        .await?;

        Ok(result)
    }
}
