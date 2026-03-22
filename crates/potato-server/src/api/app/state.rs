use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub(crate) type StdinWriter = Arc<Mutex<Option<Box<dyn tokio::io::AsyncWrite + Send + Unpin>>>>;

#[derive(Clone)]
pub(crate) struct AppState {
    pub container_id: Option<String>,
    pub stdin_writers: Arc<Mutex<HashMap<String, StdinWriter>>>,
}
