mod api;
mod app_manager;
pub mod container;

pub use api::{app_router, potato_router};
pub use app_manager::start;
pub use app_manager::{AppManager, RunningApp};

pub fn app(static_dir: std::path::PathBuf, container_id: Option<String>) -> axum::Router {
    app_router(static_dir, container_id)
}

fn uuid() -> String {
    uuid::Uuid::new_v4().to_string()
}
