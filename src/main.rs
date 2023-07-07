mod handler;
mod model;
mod response;

use handler::{health_check_handler, tasks_list_handler, create_task_handler, edit_task_handler, delete_task_handler};
use model::DB;

use axum::{
    routing::{get, post, patch},
    Router,
};

#[tokio::main]
async fn main() {
    let shared_state = model::task_db();

    /*
    let health_check = warp::path!("api" / "health_check")
        .and(warp::get())
        .and_then(handler::health_check_handler);

    let tasks_router = warp::path!("api" / "tasks");
    let tasks_router_id = warp::path!("api" / "tasks" / String);

    let task_routes = tasks_router
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(handler::create_todo_handler)
        .or(tasks_router
            .and(warp::get())
            .and(with_db(db.clone())
            .and_then(handler::tasks_list_handler))
        );

    let task_routes_id = tasks_router_id
        .and(warp::patch())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(handler::edit_task_handler)
            .or(tasks_router_id
            .and(warp::get())
            .and(with_db(db.clone()))
            .and_then(handler::get_todo_handler))

            .or(tasks_router_id
            .and(warp::delete())
            .and(with_db(db.clone()))
            .and_then(handler::delete_task_handler));


    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "OPTIONS"]);

    let routes = task_routes
        .with(warp::log("api"))
        .with(cors)
        .or(task_routes_id)
        .or(health_check);

    println!("🚀 Server started successfully");
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
    */

    // let health_check = warp::path!("api" / "health_check")
    //    .and(warp::get())
    //    .and_then(handler::health_check_handler);
    let app = Router::new()
        .route("/api/health_check", get(health_check_handler))
        .route("/api/tasks", get(tasks_list_handler))
        .route("/api/tasks", post(create_task_handler))
        .route("/api/tasks/:id", patch(edit_task_handler).delete(delete_task_handler))
        .with_state(shared_state);

    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
