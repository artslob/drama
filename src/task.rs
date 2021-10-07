use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
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
pub enum Task {
    CollectNewSubreddits {
        common: TaskCommon,
        limit: i32,
    },
    UpdateSubredditInfo {
        common: TaskCommon,
        subreddit_id: String,
    },
    CreateUser {
        common: TaskCommon,
        uid: Uuid,
    },
    CreateUserCron(TaskCommon),
}
