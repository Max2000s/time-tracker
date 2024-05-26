use crate::models::time_entry::TimeEntry;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Task {
    pub id: u32,
    pub name: String,
    pub active: bool,
    pub start_time: Option<DateTime<Utc>>,
    pub history: Vec<TimeEntry>,
}
