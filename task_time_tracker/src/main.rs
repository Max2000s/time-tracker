
use clap::{Arg, App, SubCommand};
use chrono::Utc;

mod commands;
mod models;
mod persistence;

fn main() {
    let matches = App::new("Task Time Tracker")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Tracks time spent on tasks")
        .subcommand(SubCommand::with_name("start")
            .about("Start a new task")
            .arg(Arg::with_name("task_name").required(true)))
        .subcommand(SubCommand::with_name("stop").about("Stop the current task"))
        .subcommand(SubCommand::with_name("continue")
            .about("Continue a task")
            .arg(Arg::with_name("task_name").required(true)))
        .subcommand(SubCommand::with_name("break")
            .about("Take a break")
            .arg(Arg::with_name("break_name").required(true)))
        .subcommand(SubCommand::with_name("end_break").about("End the current break"))
        .subcommand(SubCommand::with_name("list").about("List all tasks"))
        .subcommand(SubCommand::with_name("summary")
            .about("Show task summary")
            .arg(Arg::with_name("task_name").required(false)))
        .subcommand(SubCommand::with_name("export")
            .about("Export data")
            .arg(Arg::with_name("format").required(true)))
        .subcommand(SubCommand::with_name("import")
            .about("Import data")
            .arg(Arg::with_name("file_path").required(true)))
        .subcommand(SubCommand::with_name("filter")
            .about("Filter tasks by time period")
            .arg(Arg::with_name("period").required(true)))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("start") {
        let task_name = matches.value_of("task_name").unwrap();
        commands::start::execute(task_name);
    }
    // Handle other commands similarly...
}
