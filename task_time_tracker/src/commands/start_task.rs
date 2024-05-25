use crate::models::task::Task;
use crate::persistence::file_storage::save_task;
use chrono::Utc;

pub fn execute(task_name: &str) {
    let task = Task {
        id: 1,
        name: task_name.to_string(),
        start_time: Some(Utc::now()),
        accumulated_time: chrono::Duration::seconds(0),
        history: Vec::new(),
    };
    save_task(&task);
    println!("Started task: {}", task_name);
}
