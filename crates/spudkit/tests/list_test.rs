#[tokio::test]
async fn list_available_spuds_returns_labeled_images() {
    // Ensure at least one spud- prefixed labeled image exists
    let _ = std::process::Command::new("docker")
        .args(["tag", "spudkit-base:latest", "spud-test-list:latest"])
        .output();
    // Also tag a second one
    let _ = std::process::Command::new("docker")
        .args(["tag", "spudkit-base:latest", "spud-test-list2:latest"])
        .output();

    let spuds = spudkit::container::SpudkitImage::list_available()
        .await
        .unwrap();

    // Should return Spud instances with stripped names
    let names: Vec<&str> = spuds.iter().map(|s| s.name()).collect();
    assert!(
        names.contains(&"test-list"),
        "expected test-list in {names:?}"
    );
    assert!(
        names.contains(&"test-list2"),
        "expected test-list2 in {names:?}"
    );
}

#[tokio::test]
async fn list_available_spuds_excludes_non_spud_prefixed() {
    // spudkit-base has the label but not the spud- prefix — should be excluded
    let spuds = spudkit::container::SpudkitImage::list_available()
        .await
        .unwrap();
    let names: Vec<&str> = spuds.iter().map(|s| s.name()).collect();
    assert!(
        !names.contains(&"spudkit-base"),
        "should not include non-spud-prefixed images: {names:?}"
    );
}

#[tokio::test]
async fn list_available_spuds_excludes_unlabeled() {
    // debian:bookworm-slim has spud- tag but no label
    let _ = std::process::Command::new("docker")
        .args(["tag", "debian:bookworm-slim", "spud-no-label:latest"])
        .output();

    let spuds = spudkit::container::SpudkitImage::list_available()
        .await
        .unwrap();
    let names: Vec<&str> = spuds.iter().map(|s| s.name()).collect();
    assert!(
        !names.contains(&"no-label"),
        "should not include unlabeled images: {names:?}"
    );
}
