use crate::{
    response::{TaskListResponse, SingleTaskResponse, TaskData},
    model::{Task, DB, TaskAdd, TaskUpdate}
};

use axum::{Json, extract::{State, Path}, response::IntoResponse, http::StatusCode};
use serde_json::{Value, json};

use chrono::prelude::*;
use uuid::Uuid;

pub async fn health_check_handler() -> Json<Value> {
    const MESSAGE: &str = "Healthy, wooot?";

    Json(json!({
        "status": "success".to_string(),
        "message": MESSAGE.to_string(),
    }))
}

pub async fn tasks_list_handler(
    State(state): State<DB>
) -> Json<TaskListResponse> {
    let tasks = state.lock().await;

    let tasks: Vec<Task> = tasks
        .clone()
        .into_iter()
        .collect();

    let response = TaskListResponse {
        status: "success".to_string(),
        results: tasks.len(),
        tasks
    };

    Json(response)
}

pub async fn create_task_handler(
    State(state): State<DB>,
    Json(payload): Json<TaskAdd>,
) -> (StatusCode, Json<SingleTaskResponse>) {
    let mut tasks = state.lock().await;

    let now = Utc::now();

    let new_task = Task {
        id: Uuid::new_v4().to_string(),
        name: payload.name,
        created_by: payload.created_by,
        created_on: now,
        due_date: payload.due_date,
        is_over_due: false,
        is_completed: false,
        assigned_to: payload.assigned_to,
    };

    tasks.push(new_task.clone());

    let response = SingleTaskResponse {
        status: "success".to_string(),
        data: TaskData { task: new_task.clone() }
    };

    (StatusCode::CREATED, Json(response))
}

pub async fn edit_task_handler(
    Path(task_id): Path<String>,
    State(state): State<DB>,
    Json(payload): Json<TaskUpdate>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut tasks = state.lock().await;

    let task = tasks.iter_mut().find(|t| {
        t.id.eq(&task_id)
    });

    if task.is_none() {
        Err(StatusCode::NOT_FOUND)
    } else {
        let task = task.unwrap();

        let updated_task = Task {
            id: task.id.clone(),
            is_over_due: task.is_over_due,
            created_by: task.created_by.clone(),
            created_on: task.created_on,

            name: payload.name.unwrap_or(task.name.clone()),
            is_completed: payload.is_completed.unwrap_or(task.is_completed),
            assigned_to: payload.assigned_to.unwrap_or(task.assigned_to.clone()),
            due_date: payload.due_date.unwrap_or(task.due_date),
        };

        *task = updated_task;

        Ok(Json(task.clone()))
    }
}
pub async fn delete_task_handler(
    Path(task_id): Path<String>,
    State(state): State<DB>,
) -> impl IntoResponse {
    let mut tasks = state.lock().await;

    let length_before_deletion = tasks.len();
    tasks.retain(|t| !t.id.eq(&task_id));

    if length_before_deletion == tasks.len() {
        StatusCode::NOT_FOUND
    } else {
        StatusCode::NO_CONTENT
    }
}
