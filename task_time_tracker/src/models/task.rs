use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub name: String,
    pub start_time: Option<DateTime<Utc>>,
    pub accumulated_time: Duration,
    pub history: Vec<TimeEntry>,
}

#[derive(Serialize, Deserialize)]
pub struct TimeEntry {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}
