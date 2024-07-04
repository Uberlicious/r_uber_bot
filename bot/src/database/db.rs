use color_eyre::eyre::Report;
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
}
