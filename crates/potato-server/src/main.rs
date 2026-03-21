use tokio::net::UnixListener;

const APPS: &[&str] = &["potato-hello-world", "potato-hello-simple"];

#[tokio::main]
async fn main() {
    let image = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: potato-server <app-name>");
        eprintln!("Available apps: {}", APPS.join(", "));
        std::process::exit(1);
    });

    if !APPS.contains(&image.as_str()) {
        eprintln!("Unknown app: {image}");
        eprintln!("Available apps: {}", APPS.join(", "));
        std::process::exit(1);
    }

    let path = format!("/tmp/potato-{image}.sock");
    let _ = std::fs::remove_file(&path);

    println!("Extracting image filesystem...");
    let static_dir = potato_server::extract_image(&image)
        .await
        .expect("failed to extract image");
    println!("Extracted to {}", static_dir.display());

    let container_id = match potato_server::start_container(&image).await {
        Ok(id) => {
            println!("Container {id} running");
            Some(id)
        }
        Err(e) => {
            println!("No container started (static-only app): {e}");
            None
        }
    };

    let listener = UnixListener::bind(&path).unwrap();
    println!("Listening on {path}");

    let result = axum::serve(listener, potato_server::app(static_dir, container_id.clone())).await;

    if let Some(id) = &container_id {
        println!("Stopping container...");
        potato_server::stop_container(id).await;
    }

    result.unwrap();
}
