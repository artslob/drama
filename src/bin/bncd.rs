use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
struct TaskCommon {
    uid: Uuid,
    // can be added:
    // user info
    // creation time
    // app version
}

impl TaskCommon {
    fn new() -> Self {
        Self {
            uid: Uuid::new_v4(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
enum Task {
    CollectNewSubreddits {
        common: TaskCommon,
        limit: i32,
    },
    UpdateSubredditInfo {
        common: TaskCommon,
        subreddit_id: String,
    },
}

#[tokio::main]
async fn main() -> drama::Result<()> {
    let task = Task::UpdateSubredditInfo {
        common: TaskCommon::new(),
        subreddit_id: "123".to_string(),
    };
    println!("{:?}", task);
    let bytes = bincode::serialize(&task)?;
    println!("{:?}", bytes);
    println!("{}", bytes.len());
    let decoded: Task = bincode::deserialize(&bytes)?;
    println!("{:?}", decoded);
    Ok(())
}
