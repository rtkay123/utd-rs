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
