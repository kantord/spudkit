use anyhow::Context;
use std::io::{Read, Write};
use std::os::unix::net::UnixStream;

fn build_request(method: &str, path: &str, body: Option<&[u8]>) -> Vec<u8> {
    let mut request =
        format!("{method} {path} HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n");
    if let Some(b) = body {
        request.push_str(&format!(
            "Content-Type: application/json\r\nContent-Length: {}\r\n",
            b.len()
        ));
    }
    request.push_str("\r\n");
    let mut bytes = request.into_bytes();
    if let Some(b) = body {
        bytes.extend_from_slice(b);
    }
    bytes
}

/// Send an HTTP request over a Unix socket and return the response body.
pub fn http_request(
    socket_path: &str,
    method: &str,
    path: &str,
    body: Option<&[u8]>,
) -> anyhow::Result<Vec<u8>> {
    let mut stream = UnixStream::connect(socket_path)
        .with_context(|| format!("failed to connect to {socket_path}"))?;

    stream.write_all(&build_request(method, path, body))?;

    let mut response = Vec::new();
    stream.read_to_end(&mut response)?;

    if let Some(pos) = String::from_utf8_lossy(&response).find("\r\n\r\n") {
        Ok(response[pos + 4..].to_vec())
    } else {
        Ok(response)
    }
}

/// Open a Unix socket connection and send an HTTP request, returning the raw stream
/// positioned after the response headers.
pub(crate) fn open_sse_stream(
    socket_path: &str,
    method: &str,
    path: &str,
    body: Option<&[u8]>,
) -> anyhow::Result<std::io::BufReader<UnixStream>> {
    let mut stream = UnixStream::connect(socket_path)
        .with_context(|| format!("failed to connect to {socket_path}"))?;

    stream.write_all(&build_request(method, path, body))?;

    Ok(std::io::BufReader::new(stream))
}
