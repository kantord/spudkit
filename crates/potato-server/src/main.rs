use tokio::net::UnixListener;

#[tokio::main]
async fn main() {
    let image = "potato-hello-world";
    let path = "/tmp/potato.sock";
    let _ = std::fs::remove_file(path);

    println!("Extracting image filesystem...");
    let static_dir = potato_server::extract_image(image)
        .await
        .expect("failed to extract image");
    println!("Extracted to {}", static_dir.display());

    let listener = UnixListener::bind(path).unwrap();
    println!("Listening on {path}");
    axum::serve(listener, potato_server::app(static_dir, image.to_string()))
        .await
        .unwrap();
}
