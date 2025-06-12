use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub id: i64,
    pub description: String,
    pub done: bool,
}
