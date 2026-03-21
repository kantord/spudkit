mod api;
mod calls;
pub mod container;
mod registry;

use axum::{Router, routing::post};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::services::ServeDir;

pub use registry::{AppRegistry, RunningApp};

pub use container::{extract_image, start_container, stop_container};
pub use registry::start;

pub fn app(static_dir: PathBuf, container_id: Option<String>) -> Router {
    let state = calls::AppState {
        container_id,
        stdin_writers: Arc::new(Mutex::new(HashMap::new())),
    };
    Router::new()
        .route("/calls", post(calls::create_call))
        .route("/calls/{id}/stdin", post(calls::call_stdin))
        .nest_service("/files", ServeDir::new(static_dir))
        .with_state(state)
}

fn uuid() -> String {
    uuid::Uuid::new_v4().to_string()
}
