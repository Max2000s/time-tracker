use crate::models::task::Task;
use crate::persistence::file_storage::{load_tasks, save_tasks};

pub fn execute(task_name: &str) {
    let mut tasks = load_tasks();

    if let Some(pos) = find_task_position_by_name(&mut tasks, task_name) {
        tasks.remove(pos);
        println!("The task '{}' was removed from the task list.", task_name);
    } else {
        println!("The given task name '{}' was not found.", task_name)
    }

    save_tasks(&tasks).unwrap();
}

fn find_task_position_by_name<'a>(tasks: &'a mut Vec<Task>, task_name: &str) -> Option<usize> {
    tasks.iter().position(|task| task.name == task_name)
}
