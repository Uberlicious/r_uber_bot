use color_eyre::eyre::Report;
use serde_json::from_str;
use sqlx::PgPool;

use crate::database::models::Guild;

pub struct Database {
    pool: PgPool,
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

        let guild = sqlx::query_as!(
            Guild,
            r#"select * from guilds where guild_id = $1"#,
            guild_id
        )
        .fetch_optional(&pool)
        .await?;

        Ok(guild)
    }

    pub async fn create_guild(&self, guild_id: i64) -> Result<Guild, Report> {
        let pool = self.pool.clone();

        let newGuild = Guild {
            guild_id,
            prefix: None,
        };

        let result = sqlx::query_as!(
            Guild,
            "insert into guilds (guild_id, prefix) values ($1, $2)",
            newGuild.guild_id,
            newGuild.prefix
        )
        .execute(&pool)
        .await?;

        println!("{:?}", result);

        Ok(newGuild)
    }
}
