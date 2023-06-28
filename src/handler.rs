use crate::{
    response::{GenericResponse, TaskListResponse, SingleTaskResponse, TaskData},
    WebResult, model::{Task, DB, TaskAdd, TaskUpdate}
};

use chrono::prelude::*;
use uuid::Uuid;
use warp::{reply::{json, with_status}, Reply, hyper::StatusCode};

pub async fn health_check_handler() -> WebResult<impl Reply> {
    const MESSAGE: &str = "Healthy, wooot?";

    let response = &GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };

    Ok(json(&response))
}

pub async fn tasks_list_handler(db: DB) -> WebResult<impl Reply> {
    let tasks = db.lock().await;

    let tasks: Vec<Task> = tasks
        .clone()
        .into_iter()
        .collect();

    let response = TaskListResponse{
        status: "susscess".to_string(),
        results: tasks.len(),
        tasks,
    };

    Ok(json(&response))
}

pub async fn create_todo_handler(body: TaskAdd, db: DB) -> WebResult<impl Reply> {
    let mut tasks = db.lock().await;

    /*
    let task_already_exists = tasks.iter().any(|task| {
        task.name == body.name
    });
    */

    /*
    if task_already_exists {
        return Ok(
            with_status(
                json(
                    &GenericResponse {
                        status: "fail".to_string(),
                        message: format!("Todo with name '{}' already exists", body.name)
                    }
                )
            , StatusCode::CONFLICT)
        );
    }
    */

    let now = Utc::now();

    let new_task = Task {
        id: Uuid::new_v4().to_string(),
        name: body.name,
        created_by: body.created_by,
        created_on: now,
        due_date: body.due_date,
        is_over_due: false,
        is_completed: false,
        assigned_to: body.assigned_to,
    };

    tasks.push(new_task.clone());

    let response = SingleTaskResponse {
        status: "success".to_string(),
        data: TaskData { task: new_task },
    };

    Ok(with_status(json(&response), StatusCode::CREATED))
}

pub async fn get_todo_handler(id: String, db: DB) -> WebResult<impl Reply> {
    let vec = db.lock().await;

    let task = vec.iter().find(|t| {
        t.id.eq(&id)
    });

    if task.is_none() {
        let response = GenericResponse {
            status: "fail".to_string(),
            message: format!("Task with ID {id} not found"),
        };

        return Ok(with_status(json(&response), StatusCode::NOT_FOUND));
    }

    let response = SingleTaskResponse {
        status: "success".to_string(),
        data: TaskData { task: task.unwrap().clone() },
    };

    return Ok(
        with_status(
            json(&response),
            StatusCode::OK
        )
    );
}

pub async fn edit_task_handler(id: String, body: TaskUpdate, db: DB) -> WebResult<impl Reply> {
    let mut tasks = db.lock().await;

    let task = tasks.iter_mut().find(|t| {
        t.id.eq(&id)
    });

    if task.is_none() {
        let response = GenericResponse {
            status: "fail".to_string(),
            message: format!("Task with ID {id} not found"),
        };

        return Ok(with_status(json(&response), StatusCode::NOT_FOUND));
    } else {
        let task = task.unwrap();

        let updated_task = Task {
            id: task.id.clone(),
            is_over_due: task.is_over_due,
            created_by: task.created_by.clone(),
            created_on: task.created_on,

            name: body.name.unwrap_or(task.name.clone()),
            is_completed: body.is_completed.unwrap_or(task.is_completed),
            assigned_to: body.assigned_to.unwrap_or(task.assigned_to.clone()),
            due_date: body.due_date.unwrap_or(task.due_date),
        };

        *task = updated_task;

        let response = SingleTaskResponse {
            status: "success".to_string(),
            data: TaskData { task: task.clone() }
        };

        return Ok(
            with_status(
                json(&response),
                StatusCode::OK
            )
        );
    }
}
pub async fn delete_task_handler(id: String, db: DB) -> WebResult<impl Reply> {
    let mut tasks = db.lock().await;

    let length_before_deletion = tasks.len();
    tasks.retain(|t| t.id != id);

    if length_before_deletion == tasks.len() {
        let response = GenericResponse {
            status: "fail".to_string(),
            message: format!("Task with ID {id} not found"),
        };

        return Ok(
            with_status(
                json(&response),
                StatusCode::NOT_FOUND
            )
        );
    } else {
        return Ok(
            with_status(
                json(&""),
                StatusCode::NO_CONTENT
            )
        );
    }
}
