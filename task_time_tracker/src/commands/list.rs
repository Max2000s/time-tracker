use crate::models::task::Task;
use crate::persistence::file_storage::load_tasks;
use chrono::{Duration, Utc};
use prettytable::{cell, row, Table};

pub fn execute() {
    let tasks = load_tasks();
    display_tasks(&tasks);
}

fn display_tasks(tasks: &[Task]) {
    let mut table = Table::new();
    table.add_row(row![
        "Name",
        "Active",
        "Current Duration",
        "Last Entry",
        "Total Duration"
    ]);

    for task in tasks {
        let current_duration = if task.active {
            task.start_time.map_or(Duration::seconds(0), |start_time| {
                Utc::now().signed_duration_since(start_time)
            })
        } else {
            Duration::seconds(0)
        };

        let last_entry = task.history.last().map_or("N/A".to_string(), |entry| {
            format!("{} - {}", entry.start, entry.end)
        });
        let total_duration = task
            .history
            .iter()
            .fold(Duration::seconds(0), |acc, entry| {
                acc + entry.end.signed_duration_since(entry.start)
            });

        table.add_row(row![
            task.name,
            if task.active { "Yes" } else { "--" },
            format_duration(current_duration),
            last_entry,
            format_duration(total_duration),
        ]);
    }

    table.printstd();
}

fn format_duration(duration: Duration) -> String {
    let seconds = duration.num_seconds();
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let seconds = seconds % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}
