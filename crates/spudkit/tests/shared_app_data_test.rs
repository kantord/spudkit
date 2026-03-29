#[allow(dead_code)]
mod helpers;

use helpers::build_labeled_image_with_extra;
use spudkit_core::Spud;
use std::fs;

#[tokio::test]
async fn shared_app_data_mounts_host_directory() {
    build_labeled_image_with_extra(
        "spud-mount-test",
        "LABEL io.github.kantord.spudkit.shared_app_data=\"mount-test-data\"",
    );

    // Create a temp dir with a known file
    let host_data_dir = std::env::temp_dir().join("spudkit-test-shared-data");
    let app_data = host_data_dir.join("mount-test-data");
    fs::create_dir_all(&app_data).unwrap();
    fs::write(app_data.join("hello.txt"), b"from-host").unwrap();

    // Activate the image with the custom data dir
    let spud = Spud::new("mount-test").unwrap();
    let image = spudkit::container::SpudkitImage::from_spud_with_data_dir(spud, &host_data_dir)
        .await
        .unwrap();
    let container = image.start().await.unwrap();

    // The file should be visible inside the container
    let content = container
        .cat_file("/root/.local/share/mount-test-data/hello.txt")
        .await
        .unwrap();
    assert_eq!(
        content,
        Some(b"from-host".to_vec()),
        "host file should be readable from inside the container"
    );

    container.stop().await;
    let _ = fs::remove_dir_all(&host_data_dir);
}

#[tokio::test]
async fn shared_app_data_writes_persist_to_host() {
    build_labeled_image_with_extra(
        "spud-mount-write-test",
        "LABEL io.github.kantord.spudkit.shared_app_data=\"mount-write-data\"",
    );

    let host_data_dir = std::env::temp_dir().join("spudkit-test-shared-write");
    let app_data = host_data_dir.join("mount-write-data");
    fs::create_dir_all(&app_data).unwrap();

    let spud = Spud::new("mount-write-test").unwrap();
    let image = spudkit::container::SpudkitImage::from_spud_with_data_dir(spud, &host_data_dir)
        .await
        .unwrap();
    let container = image.start().await.unwrap();

    // Write a file from inside the container
    let cmd = vec![
        "/bin/sh".to_string(),
        "-c".to_string(),
        "echo from-container > /root/.local/share/mount-write-data/written.txt".to_string(),
    ];
    let _ = container.run(cmd, None).await;

    // The file should appear on the host
    let content = fs::read_to_string(app_data.join("written.txt")).unwrap();
    assert_eq!(
        content.trim(),
        "from-container",
        "container writes should persist to host"
    );

    container.stop().await;
    let _ = fs::remove_dir_all(&host_data_dir);
}

#[tokio::test]
async fn no_shared_app_data_label_means_no_mounts() {
    build_labeled_image_with_extra("spud-no-mount-test", "");

    let host_data_dir = std::env::temp_dir().join("spudkit-test-no-mount");
    let app_data = host_data_dir.join("no-mount-data");
    fs::create_dir_all(&app_data).unwrap();
    fs::write(app_data.join("secret.txt"), b"should-not-be-visible").unwrap();

    let spud = Spud::new("no-mount-test").unwrap();
    let image = spudkit::container::SpudkitImage::from_spud_with_data_dir(spud, &host_data_dir)
        .await
        .unwrap();
    let container = image.start().await.unwrap();

    // The file should NOT be visible inside the container
    let content = container
        .cat_file("/root/.local/share/no-mount-data/secret.txt")
        .await
        .unwrap();
    assert_eq!(
        content, None,
        "without shared_app_data label, host files should not be visible"
    );

    container.stop().await;
    let _ = fs::remove_dir_all(&host_data_dir);
}
