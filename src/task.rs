use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use uuid::Uuid;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskCommon {
    pub uid: Uuid,
    pub created_at: DateTime<Utc>,
    pub app_version: String,
    // can be added: user info
}

impl TaskCommon {
    pub fn new() -> Self {
        Self {
            uid: Uuid::new_v4(),
            created_at: Utc::now(),
            app_version: VERSION.to_string(),
        }
    }
}

impl Default for TaskCommon {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub common: TaskCommon,
    pub data: Data,
}

#[derive(Serialize, Deserialize, Debug, strum::IntoStaticStr)]
pub enum Data {
    Cron(Cron),
    CreateUser { uid: Uuid },
    UpdateUserSubreddits { user_id: String },
    UpdateUserInfo { user_id: String },
}

#[derive(Serialize, Deserialize, Debug, strum::IntoStaticStr, strum::EnumIter, Copy, Clone)]
pub enum Cron {
    CreateUserCron,
    UpdateUserSubredditsCron,
    UpdateUserInfoCron,
}

impl Cron {
    pub fn frequency(&self) -> Duration {
        let secs = match self {
            Cron::CreateUserCron => 10,
            Cron::UpdateUserSubredditsCron => 20,
            Cron::UpdateUserInfoCron => 30,
        };
        Duration::from_secs(secs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_user_task_name() {
        let task = Data::UpdateUserInfo {
            user_id: "id".to_string(),
        };
        let result: &'static str = task.into();
        assert_eq!(result, "UpdateUserInfo");
    }

    #[test]
    fn test_cron_task_name() {
        let result: &'static str = Data::Cron(Cron::CreateUserCron).into();
        // TODO make names for cron more verbose
        assert_eq!(result, "Cron");
    }
}
