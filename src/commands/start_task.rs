use crate::models::task::Task;
use crate::persistence::file_storage::{load_tasks, save_tasks};
use chrono::Utc;

pub fn execute(task_name: &str) {
    let mut tasks = load_tasks();

    if check_for_active_task(&mut tasks) {
        println!("There is already a active task! Use stop to deactivate it and start the new one again.");
        return;
    }

    if let Some(task) = find_task_by_name(&mut tasks, task_name) {
        print!("{:?}", task);
        if task.active == false {
            update_existing_task(task);
            println!("Started inactive existing task: {}", task_name);
        } else {
            println!("The task {} is already running.", task_name);
            // dialog for resetting the time would be possible
        }
    } else {
        create_new_task(&mut tasks, task_name);
        println!("Started new task: {}", task_name);
    }

    save_tasks(&tasks).unwrap();
}

fn check_for_active_task(tasks: &mut Vec<Task>) -> bool {
    tasks.iter_mut().any(|t| t.active)
}

fn find_task_by_name<'a>(tasks: &'a mut Vec<Task>, task_name: &str) -> Option<&'a mut Task> {
    tasks.iter_mut().find(|task| task.name == task_name)
}

fn update_existing_task(task: &mut Task) {
    task.start_time = Some(Utc::now());
    task.active = true;
}

fn create_new_task(tasks: &mut Vec<Task>, task_name: &str) {
    let task = Task {
        id: tasks.len() as u32 + 1,
        name: task_name.to_string(),
        active: true,
        start_time: Some(Utc::now()),
        history: Vec::new(),
    };
    tasks.push(task);
}
