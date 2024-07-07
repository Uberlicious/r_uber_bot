use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
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

    pub fn with_prefix(mut self, prefix: String) -> Guild {
        self.prefix = Some(prefix);
        self
    }
}
