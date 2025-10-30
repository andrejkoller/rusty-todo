use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub done: bool,
}

impl Task {
    pub fn new(id: u32, description: String) -> Self {
        Task {
            id,
            description,
            done: false,
        }
    }

    pub fn mark_done(&mut self) {
        self.done = true;
    }
}
