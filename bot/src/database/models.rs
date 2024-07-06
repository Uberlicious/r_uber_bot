#[derive(Debug)]
pub struct Guild {
    pub guild_id: i64,
    pub prefix: Option<String>,
}

impl Guild {
    pub fn new(guild_id: i64) -> Guild {
        Guild {
            guild_id,
            prefix: None,
        }
    }
}
