// src/commands/stop.rs
use crate::{
    models::time_entry::TimeEntry,
    persistence::file_storage::{load_tasks, save_task},
};
use chrono::Utc;

pub fn execute() {
    let mut tasks = load_tasks();
    if let Some(task) = tasks.iter_mut().find(|t| t.active == true) {
        println!("Stopped task: {}", task.name);

        let entry: TimeEntry = TimeEntry {
            start: task.start_time.unwrap(),
            end: Utc::now(),
        };

        task.history.push(entry);
        task.start_time = None;
        task.active = false;
        save_task(&tasks);
    } else {
        println!("No task is currently running.");
    }
}
