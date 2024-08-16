use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdateToDoRequest {
    pub title: String,
    pub content: String,
    pub completed: Option<bool>,
}
