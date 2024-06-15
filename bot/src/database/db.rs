use color_eyre::eyre::Report;
use sqlx::{PgPool, Pool, Postgres};

pub struct Database {
    pool: PgPool,
}

impl Database {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_guilds(&self) -> Result<(), Report> {
        let pool = self.pool.clone();

        let guilds = sqlx::query!("SELECT * FROM guilds")
            .fetch_all(&pool)
            .await?;

        println!("{:?}", guilds);
        Ok(())
    }
}
