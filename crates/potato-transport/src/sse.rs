use std::io::BufRead;

/// A parsed SSE event with its event type and data.
pub enum SseEvent {
    Started { call_id: String },
    Output(serde_json::Value),
    Error(serde_json::Value),
    End,
}

/// Stream raw SSE data lines from a Unix socket. Calls `on_line` for each `data:` line
/// (with prefix stripped). Sends `{"event":"end"}` when the stream closes.
pub fn stream_sse_raw(
    socket_path: &str,
    method: &str,
    path: &str,
    body: Option<&[u8]>,
    mut on_line: impl FnMut(&str),
) {
    let reader = match crate::socket::open_sse_stream(socket_path, method, path, body) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("failed to connect: {e}");
            std::process::exit(1);
        }
    };

    let mut past_headers = false;

    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };

        if !past_headers {
            if line.is_empty() {
                past_headers = true;
            }
            continue;
        }

        if let Some(data) = line.strip_prefix("data:") {
            let data = data.trim();
            if !data.is_empty() {
                on_line(data);
            }
        }
    }

    on_line(r#"{"event":"end"}"#);
}

/// Stream SSE events from a Unix socket. Calls `on_event` for each parsed event.
/// Returns when the stream ends or the connection closes.
pub fn stream_sse(
    socket_path: &str,
    method: &str,
    path: &str,
    body: Option<&[u8]>,
    mut on_event: impl FnMut(SseEvent),
) {
    let reader = match crate::socket::open_sse_stream(socket_path, method, path, body) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("failed to connect: {e}");
            std::process::exit(1);
        }
    };

    let mut past_headers = false;

    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };

        if !past_headers {
            if line.is_empty() {
                past_headers = true;
            }
            continue;
        }

        let Some(data) = line.strip_prefix("data:") else {
            continue;
        };
        let data = data.trim();
        if data.is_empty() {
            continue;
        }

        let Ok(parsed) = serde_json::from_str::<serde_json::Value>(data) else {
            continue;
        };

        let event = parsed
            .get("event")
            .and_then(|e| e.as_str())
            .unwrap_or("output");

        match event {
            "started" => {
                let call_id = parsed["data"]["call_id"].as_str().unwrap_or("").to_string();
                on_event(SseEvent::Started { call_id });
            }
            "end" => {
                on_event(SseEvent::End);
                break;
            }
            "error" => {
                if let Some(d) = parsed.get("data") {
                    on_event(SseEvent::Error(d.clone()));
                }
            }
            _ => {
                if let Some(d) = parsed.get("data") {
                    on_event(SseEvent::Output(d.clone()));
                }
            }
        }
    }
}
