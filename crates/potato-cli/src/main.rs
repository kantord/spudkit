use clap::Parser;
use potato_client::{PotatoClient, SseEvent};
use std::io::BufRead;

/// Run containerized apps from the command line
#[derive(Parser)]
#[command(name = "potato", version, about)]
struct Args {
    /// Docker image name of the app
    app: String,

    /// Command to run inside the container
    command: Vec<String>,
}

fn format_data(data: &serde_json::Value) -> String {
    match data {
        serde_json::Value::String(s) => s.clone(),
        other => other.to_string(),
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let client = PotatoClient::new();
    let app = client.app(&args.app).await?;
    let app_for_stdin = app.clone();

    let (started_tx, started_rx) = tokio::sync::oneshot::channel::<String>();

    let cmd = args.command;
    let output_handle = tokio::spawn(async move {
        let mut started_tx = Some(started_tx);
        app.call(&cmd, |event| match event {
            SseEvent::Started { call_id } => {
                if let Some(tx) = started_tx.take() {
                    let _ = tx.send(call_id);
                }
            }
            SseEvent::Output(data) => println!("{}", format_data(&data)),
            SseEvent::Error(data) => eprintln!("{}", format_data(&data)),
            SseEvent::End => {}
        })
        .await;
    });

    let call_id = match started_rx.await {
        Ok(id) => id,
        Err(_) => {
            let _ = output_handle.await;
            return Ok(());
        }
    };

    // Forward stdin line by line concurrently with output
    let stdin_handle = tokio::spawn(async move {
        let (line_tx, mut line_rx) = tokio::sync::mpsc::channel::<String>(32);

        // Read stdin in a blocking thread
        std::thread::spawn(move || {
            let stdin = std::io::stdin();
            for line in stdin.lock().lines() {
                match line {
                    Ok(l) => {
                        if line_tx.blocking_send(l).is_err() {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
        });

        // Send each line as it arrives
        while let Some(line) = line_rx.recv().await {
            let data = serde_json::json!({ "text": line });
            let _ = app_for_stdin.send_stdin(&call_id, &data).await;
        }
    });

    // Wait for output to finish (process exit closes the stream)
    let _ = output_handle.await;
    stdin_handle.abort();
    Ok(())
}
