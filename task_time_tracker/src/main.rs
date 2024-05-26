use clap::{App, Arg, SubCommand};

mod commands;
mod models;
mod persistence;

fn main() {
    let matches = App::new("Task Time Tracker")
        .version("1.0")
        .author("Maximilian Schoellkopf")
        .about("Tracks time spent on tasks")
        .subcommand(
            SubCommand::with_name("start")
                .about("Start a new task")
                .arg(Arg::with_name("task_name").required(true)),
        )
        .subcommand(SubCommand::with_name("stop").about("Stop the current task"))
        .subcommand(SubCommand::with_name("list").about("List all tasks with some information"))
        .subcommand(
            SubCommand::with_name("delete")
                .about("Deletes the task by its name")
                .arg(Arg::with_name("task_name").required(true)),
        )
        /*.subcommand(
            SubCommand::with_name("continue")
                .about("Continue a task")
                .arg(Arg::with_name("task_name").required(true)),
        )
         .subcommand(
            SubCommand::with_name("break")
                .about("Take a break")
                .arg(Arg::with_name("break_name").required(true)),
        )
        .subcommand(SubCommand::with_name("end_break").about("End the current break"))
        .subcommand(SubCommand::with_name("list").about("List all tasks"))
        .subcommand(
            SubCommand::with_name("summary")
                .about("Show task summary")
                .arg(Arg::with_name("task_name").required(false)),
        )
        .subcommand(
            SubCommand::with_name("export")
                .about("Export data")
                .arg(Arg::with_name("format").required(true)),
        )
        .subcommand(
            SubCommand::with_name("import")
                .about("Import data")
                .arg(Arg::with_name("file_path").required(true)),
        )
        .subcommand(
            SubCommand::with_name("filter")
                .about("Filter tasks by time period")
                .arg(Arg::with_name("period").required(true)),
        )*/
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("start") {
        let task_name = matches.value_of("task_name").unwrap();
        commands::start_task::execute(task_name);
    }

    if matches.subcommand_matches("stop").is_some() {
        commands::stop_task::execute();
    }

    if matches.subcommand_matches("list").is_some() {
        commands::list::execute();
    }

    if let Some(matches) = matches.subcommand_matches("delete") {
        let task_name = matches.value_of("task_name").unwrap();
        commands::delete_task::execute(task_name);
    }
}
