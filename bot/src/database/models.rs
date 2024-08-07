use poise::serenity_prelude::Timestamp;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Guild {
    pub guild_id: i64,
    pub prefix: Option<String>,
    pub role_assign_id: Option<i64>,
}

impl Guild {
    pub fn new(guild_id: i64) -> Guild {
        Guild {
            guild_id,
            prefix: None,
            role_assign_id: None,
        }
    }

    pub fn with_prefix(mut self, prefix: &String) -> Guild {
        self.prefix = Some(prefix.clone());
        self
    }

    pub fn with_role_assign_id(mut self, role_assign_id: i64) -> Guild {
        self.role_assign_id = Some(role_assign_id);
        self
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RoleAssign {
    pub id: Option<i64>,
    pub channel: i64,
    pub roles: Option<Vec<i64>>,
}

impl RoleAssign {
    pub fn new(channel_id: i64) -> RoleAssign {
        RoleAssign {
            id: None,
            channel: channel_id,
            roles: None,
        }
    }

    pub fn with_roles(mut self, roles: Vec<i64>) -> RoleAssign {
        self.roles = Some(roles);
        self
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CommandHistory {
    pub id: Option<i64>,
    pub user_id: i64,
    pub guild_id: i64,
    pub command_name: String,
    pub executed_at: chrono::DateTime<chrono::Utc>,
}
