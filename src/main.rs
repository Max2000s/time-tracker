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
        .subcommand(
            SubCommand::with_name("report")
                .about("Generate a report of time spent on tasks")
                .arg(
                    Arg::with_name("period")
                        .short('p')
                        .long("period")
                        .takes_value(true)
                        .possible_values(&["day", "week", "month", "all"])
                        .default_value("all")
                        .help("Specify the period for the report"),
                )
                .arg(
                    Arg::with_name("export")
                        .short('e')
                        .long("export")
                        .takes_value(true)
                        .help("Specify a file to export the report"),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("start") {
        let task_name = matches.value_of("task_name").unwrap();
        commands::start_task::execute(task_name);
    } else if matches.subcommand_matches("stop").is_some() {
        commands::stop_task::execute();
    } else if matches.subcommand_matches("list").is_some() {
        commands::list::execute();
    } else if let Some(matches) = matches.subcommand_matches("delete") {
        let task_name = matches.value_of("task_name").unwrap();
        commands::delete_task::execute(task_name);
    } else if let Some(matches) = matches.subcommand_matches("report") {
        let period = matches.value_of("period").unwrap();
        let export_file = matches.value_of("export");
        commands::report::execute(period, export_file);
    }
}
