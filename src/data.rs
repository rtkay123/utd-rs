use serde::{Deserialize, Serialize};

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
}

impl Task {
    pub fn new(name: &str, tags: &str) -> Self {
        Self {
            id: 0,
            name: name.trim().to_string(),
            tags: tags.to_string(),
            is_task: false,
            is_done: false,
            timestamp: "hello".to_string(),
            in_progress: false,
        }
    }
}
