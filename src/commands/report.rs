use crate::{
    models::task::Task, models::time_entry::TimeEntry, persistence::file_storage::load_tasks,
};
use chrono::{Datelike, Duration, Utc};
use prettytable::{cell, row, Table};
use std::fs::File;
use std::io::Write;

pub fn execute(period: &str, export_file: Option<&str>) {
    let tasks = load_tasks();
    let report_data = generate_report(&tasks, period);
    display_report(&report_data);

    if let Some(file) = export_file {
        export_report(&report_data, file);
    }
}

fn generate_report(tasks: &[Task], period: &str) -> Vec<(String, Duration)> {
    let now = Utc::now();
    tasks
        .iter()
        .map(|task| {
            let filtered_history: Vec<&TimeEntry> = match period {
                "day" => task
                    .history
                    .iter()
                    .filter(|entry| entry.start.date_naive() == now.date_naive())
                    .collect(),
                "week" => task
                    .history
                    .iter()
                    .filter(|entry| now.signed_duration_since(entry.start).num_weeks() == 0)
                    .collect(),
                "month" => task
                    .history
                    .iter()
                    .filter(|entry| {
                        entry.start.month() == now.month() && entry.start.year() == now.year()
                    })
                    .collect(),
                _ => task.history.iter().collect(),
            };
            let total_duration = filtered_history
                .iter()
                .fold(Duration::seconds(0), |acc, entry| {
                    acc + entry.end.signed_duration_since(entry.start)
                });
            (task.name.clone(), total_duration)
        })
        .collect()
}

fn display_report(report_data: &[(String, Duration)]) {
    let mut table = Table::new();
    table.add_row(row!["Task Name", "Time Spent"]);

    for (task_name, duration) in report_data {
        table.add_row(row![task_name, format_duration(*duration)]);
    }

    table.printstd();
}

fn export_report(report_data: &[(String, Duration)], file: &str) {
    let mut file = File::create(file).expect("Unable to create file");
    writeln!(file, "Task Name,Time Spent").expect("Unable to write to file");

    for (task_name, duration) in report_data {
        writeln!(file, "{},{}", task_name, format_duration(*duration))
            .expect("Unable to write to file");
    }
}

fn format_duration(duration: Duration) -> String {
    let seconds = duration.num_seconds();
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let seconds = seconds % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}
