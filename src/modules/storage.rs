use crate::modules::task::Task;
use serde_json;
use std::fs;

const FILE_PATH: &str = "tasks.json";

pub fn save_tasks(tasks: &Vec<Task>) {
    let json = serde_json::to_string_pretty(tasks).unwrap();
    fs::write(FILE_PATH, json).unwrap();
}

pub fn load_tasks() -> Vec<Task> {
    if let Ok(data) = fs::read_to_string(FILE_PATH) {
        serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
    } else {
        Vec::new()
    }
}
