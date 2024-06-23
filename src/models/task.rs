use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub completed: bool,
    pub created_at: String,
    pub update_at: String,
}
