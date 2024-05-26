use crate::models::task::Task;
use serde_json;
use std::fs::File;
use std::io::{Read, Write};

const FILE_PATH: &str = "tasks.json";

pub fn save_task(tasks: &[Task]) {
    let json = serde_json::to_string(tasks).unwrap();
    let mut file = File::create(FILE_PATH).unwrap();
    file.write_all(json.as_bytes()).unwrap();
}

pub fn save_tasks(tasks: &[Task]) -> Result<(), std::io::Error> {
    let json = serde_json::to_string(tasks).unwrap();
    let mut file = File::create(FILE_PATH)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

pub fn load_tasks() -> Vec<Task> {
    let mut file = match File::open(FILE_PATH) {
        Ok(file) => file,
        Err(_) => return Vec::new(), // Return an empty vector if file does not exist
    };

    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    if content.is_empty() {
        Vec::new()
    } else {
        serde_json::from_str(&content).unwrap()
    }
}
