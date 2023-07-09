mod handler;
mod model;
mod response;

use handler::{
    health_check_handler,
    tasks_list_handler,
    create_task_handler,
    edit_task_handler,
    delete_task_handler
};

use axum::{
    routing::{get, post, patch},
    http::Request,
    Router, extract::MatchedPath,
};

use tower_http::trace::TraceLayer;
use tracing::info_span;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    let shared_state = model::task_db();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                 "[http_request]=debug".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/api/health_check", get(health_check_handler))
        .route("/api/tasks", get(tasks_list_handler))
        .route("/api/tasks", post(create_task_handler))
        .route("/api/tasks/:id", patch(edit_task_handler).delete(delete_task_handler))
        .with_state(shared_state)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    let matched_path = request
                        .extensions()
                        .get::<MatchedPath>()
                        .map(MatchedPath::as_str);

                    info_span!(
                        "http_request",
                        method = ?request.method(),
                        matched_path,
                        some_other_field = tracing::field::Empty,
                    )
                })
        )
    ;

    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
