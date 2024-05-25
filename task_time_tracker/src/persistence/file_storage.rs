use crate::models::task::Task;
use serde_json;
use std::fs::File;
use std::io::{Read, Write};

const FILE_PATH: &str = "tasks.json";

pub fn save_task(task: &Task) {
    let mut tasks = load_tasks();
    tasks.push(task.clone());
    let json = serde_json::to_string(&tasks).unwrap();
    let mut file = File::create(FILE_PATH).unwrap();
    file.write_all(json.as_bytes()).unwrap();
}

pub fn load_tasks() -> Vec<Task> {
    let mut file = File::open(FILE_PATH).unwrap_or(File::create(FILE_PATH).unwrap());
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    if content.is_empty() {
        return Vec::new();
    }
    serde_json::from_str(&content).unwrap()
}
