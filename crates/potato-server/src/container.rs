use bollard::Docker;
use bollard::models::ContainerCreateBody;
use bollard::query_parameters::{CreateContainerOptions, RemoveContainerOptions};
use futures_util::StreamExt;
use std::path::PathBuf;

/// A running app container.
pub struct AppContainer {
    pub id: String,
}

impl AppContainer {
    /// Start a persistent container for an app image.
    pub async fn start(image: &str) -> anyhow::Result<Self> {
        let docker = Docker::connect_with_local_defaults()?;

        let config = ContainerCreateBody {
            image: Some(image.to_string()),
            cmd: Some(vec!["sleep".to_string(), "infinity".to_string()]),
            ..Default::default()
        };

        let name = format!("potato-{}", crate::utils::generate_id());
        let container = docker
            .create_container(
                Some(CreateContainerOptions {
                    name: Some(name),
                    ..Default::default()
                }),
                config,
            )
            .await?;

        docker.start_container(&container.id, None).await?;

        Ok(Self { id: container.id })
    }

    /// Stop and remove the container.
    pub async fn stop(&self) {
        if let Ok(docker) = Docker::connect_with_local_defaults() {
            let _ = docker
                .remove_container(
                    &self.id,
                    Some(RemoveContainerOptions {
                        force: true,
                        ..Default::default()
                    }),
                )
                .await;
        }
    }
}

/// Extract an image's filesystem to a temp directory.
pub async fn extract_image(image: &str) -> anyhow::Result<PathBuf> {
    let docker = Docker::connect_with_local_defaults()?;

    let config = ContainerCreateBody {
        image: Some(image.to_string()),
        cmd: Some(vec!["true".to_string()]),
        ..Default::default()
    };

    let name = format!("potato-extract-{}", crate::utils::generate_id());
    let container = docker
        .create_container(
            Some(CreateContainerOptions {
                name: Some(name),
                ..Default::default()
            }),
            config,
        )
        .await?;

    let extract_dir = std::env::temp_dir().join(format!("potato-{}", crate::utils::generate_id()));
    std::fs::create_dir_all(&extract_dir)?;

    let (pipe_reader, mut pipe_writer) = os_pipe::pipe()?;
    let extract_dir_clone = extract_dir.clone();

    let unpack_handle = std::thread::spawn(
        move || -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            let mut archive = tar::Archive::new(pipe_reader);
            archive.set_preserve_permissions(false);
            archive.set_unpack_xattrs(false);
            for entry in archive.entries()? {
                let mut entry = match entry {
                    Ok(e) => e,
                    Err(_) => continue,
                };
                let kind = entry.header().entry_type();
                if kind.is_file() || kind.is_dir() || kind.is_symlink() || kind.is_hard_link() {
                    let _ = entry.unpack_in(&extract_dir_clone);
                }
            }
            Ok(())
        },
    );

    let mut tar_stream = docker.export_container(&container.id);
    while let Some(chunk) = tar_stream.next().await {
        let chunk = chunk?;
        if std::io::Write::write_all(&mut pipe_writer, &chunk).is_err() {
            break;
        }
    }
    drop(pipe_writer);

    unpack_handle
        .join()
        .map_err(|_| anyhow::anyhow!("unpack thread panicked"))?
        .map_err(|e| anyhow::anyhow!("{e}"))?;

    let _ = docker
        .remove_container(
            &container.id,
            Some(RemoveContainerOptions {
                force: true,
                ..Default::default()
            }),
        )
        .await;

    Ok(extract_dir)
}
