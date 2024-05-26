use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TimeEntry {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}
