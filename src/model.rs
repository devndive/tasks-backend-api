use chrono::{prelude::*, Duration};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: String,
    pub name: String,
    pub created_by: String,
    pub created_on: DateTime<Utc>,
    pub due_date: DateTime<Utc>,
    pub assigned_to: String,
    pub is_completed: bool,
    pub is_over_due: bool,
}

#[derive(Serialize, Deserialize)]
pub struct TaskAdd {
    pub name: String,
    pub created_by: String,
    pub due_date: DateTime<Utc>,
    pub assigned_to: String,
}

#[derive(Serialize, Deserialize)]
pub struct TaskUpdate {
    pub name: Option<String>,
    pub due_date: Option<DateTime<Utc>>,
    pub assigned_to: Option<String>,
    pub is_completed: Option<bool>,
}

pub type DB = Arc<Mutex<Vec<Task>>>;

pub fn task_db() -> DB {
    let tasks: Vec<Task> = (0..10).map(|n| {
        let now = Utc::now();

        Task {
            id: Uuid::new_v4().to_string(),
            name: format!("Task number {n}"),
            created_by: "yann.duval@microsoft.com".to_string(),
            created_on: now,
            due_date: now.checked_add_signed(Duration::days(1)).unwrap(),
            assigned_to: "Someone elese".to_string(),
            is_completed: false,
            is_over_due: false
        }
    }).collect();

    Arc::new(Mutex::new(tasks))
}
