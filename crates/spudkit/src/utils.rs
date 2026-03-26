pub fn generate_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

/// Resolve a user-supplied path relative to a base directory inside the container.
/// Returns `None` if the path would escape the base directory.
pub fn resolve_container_path(base: &str, user_path: &str) -> Option<String> {
    use std::path::{Component, PathBuf};

    let mut resolved = PathBuf::from(base);
    for component in std::path::Path::new(user_path).components() {
        match component {
            Component::Normal(c) => resolved.push(c),
            Component::ParentDir => return None,
            _ => {}
        }
    }
    Some(resolved.to_string_lossy().into_owned())
}

/// Resolve a command to its full path inside the container.
/// If the command doesn't start with `/`, prepend `/app/bin/`.
pub fn resolve_cmd(cmd: &[String]) -> Vec<String> {
    let mut resolved = cmd.to_vec();
    if let Some(first) = resolved.first_mut()
        && !first.starts_with('/')
    {
        *first = format!("/app/bin/{first}");
    }
    resolved
}
