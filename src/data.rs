use serde::{Deserialize, Serialize};

use crate::args::PriorityLevel;

pub type Tasks = Vec<Task>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: i64,
    pub name: String,
    pub tags: String,
    #[serde(rename = "is_task")]
    pub is_task: bool,
    #[serde(rename = "is_done")]
    pub is_done: bool,
    pub timestamp: String,
    #[serde(rename = "in_progress")]
    pub in_progress: bool,
    pub priority: String,
}

impl Task {
    pub fn new(
        name: &str,
        tags: &str,
        is_task: bool,
        id: i64,
        priority: PriorityLevel,
        timestamp: u128,
    ) -> Self {
        Self {
            id,
            name: name.trim().replace("  ", ""),
            tags: tags.to_string(),
            is_task,
            is_done: false,
            timestamp: timestamp.to_string(),
            in_progress: false,
            priority: priority.to_string(),
        }
    }
}
