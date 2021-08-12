use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: Option<u32>,
    pub content: String,
    pub done: bool,
}