use super::{priority::Priority, status::Status};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub priority: Priority,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

impl Task {
    pub fn new(title: String, priority: Priority) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            description: None,
            priority,
            status: Status::ToDo,
            created_at: Utc::now(),
        }
    }
    pub fn toggle(&mut self) {
        self.status = match self.status {
            Status::ToDo => Status::Done,
            Status::Done => Status::ToDo,
        }
    }

    pub fn set_description(&mut self, desc: String) {
        self.description = Some(desc);
    }
}
