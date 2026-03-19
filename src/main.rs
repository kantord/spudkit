use axum::{Router, extract::Path, routing::get};
use std::process::Command;
use tokio::net::UnixListener;

async fn run_command(Path(command): Path<String>) -> String {
    match Command::new(&command).output() {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            format!("{stdout}{stderr}")
        }
        Err(e) => format!("error: {e}\n"),
    }
}

#[tokio::main]
async fn main() {
    let path = "/tmp/potato.sock";
    let _ = std::fs::remove_file(path);

    let app = Router::new().route("/run/{command}", get(run_command));

    let listener = UnixListener::bind(path).unwrap();
    println!("Listening on {path}");
    axum::serve(listener, app).await.unwrap();
}
