use std::{net::SocketAddr, time::Duration};

use axum::{
    extract::{Query, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};
use axum::extract::ws::{Message, WebSocket};
use serde::Deserialize;
use tokio::{fs, io::{AsyncBufReadExt, BufReader}, net::TcpListener, process::Command, time::{sleep, timeout}};
use tower_http::services::ServeDir;

#[derive(Debug, Deserialize)]
struct WsParams {
    theme: Option<String>,
    stack: Option<String>,
    duration: Option<u64>, // seconds
}

#[tokio::main]
async fn main() {
    // Static files under ./web
    let static_service = ServeDir::new("web");

    let app = Router::new()
        .route("/ws", get(ws_handler))
        .fallback_service(static_service);

    let addr: SocketAddr = "127.0.0.1:3000".parse().unwrap();
    println!("installer-web listening on http://{addr}");
    println!("Open in your browser to test the simulation.");

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn ws_handler(ws: WebSocketUpgrade, Query(params): Query<WsParams>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| websocket_stream(socket, params))
}

async fn websocket_stream(mut socket: WebSocket, params: WsParams) {
    let duration_secs = params.duration.unwrap_or(30);
    let stack = params.stack.unwrap_or_else(|| "default".to_string());
    let theme = params.theme.unwrap_or_else(|| "dracula".to_string());

    // Start a child process of the CLI simulator and stream its stdout
    let binary = resolve_cli_binary();
    let mut cmd = Command::new(binary);
    // Run many cycles and enforce duration by timeout
    cmd.arg("--cycles").arg("100000");
    // Map stack -> CLI args
    if stack == "default" {
        cmd.arg("--all");
    } else {
        cmd.arg(&stack);
    }
    cmd.env("WEB_MODE", "1");
    cmd.stdout(std::process::Stdio::piped());
    cmd.stderr(std::process::Stdio::piped());

    match cmd.spawn() {
        Ok(mut child) => {
            let stdout = child.stdout.take();
            let mut reader = stdout.map(BufReader::new);

            let _ = socket
                .send(Message::Text(format!(
                    "\u{001b}[36mStarting simulation (stack: {stack}, theme: {theme}, duration: {duration_secs}s).\u{001b}[0m"
                )))
                .await;

            let end_time = tokio::time::Instant::now() + Duration::from_secs(duration_secs);
            loop {
                // timeout small interval to check duration
                if tokio::time::Instant::now() >= end_time { break; }
                if let Some(r) = reader.as_mut() {
                    let mut line = String::new();
                    match timeout(Duration::from_millis(100), r.read_line(&mut line)).await {
                        Ok(Ok(0)) => break, // EOF
                        Ok(Ok(_n)) => {
                            if !line.is_empty() {
                                let _ = socket.send(Message::Text(line.trim_end_matches('\n').to_string())).await;
                            }
                        }
                        Ok(Err(_e)) => break,
                        Err(_) => {
                            // read timed out; continue to check duration
                        }
                    }
                } else {
                    break;
                }
            }

            // Kill child if still running
            let _ = child.start_kill();
            let _ = child.wait().await;

            let _ = socket
                .send(Message::Text("\u{001b}[32mSimulation complete.\u{001b}[0m".into()))
                .await;
            let _ = socket.close().await;
        }
        Err(e) => {
            let _ = socket
                .send(Message::Text(format!("\u{001b}[31mFailed to start simulator: {e}\u{001b}[0m")))
                .await;
            let _ = socket.close().await;
        }
    }
}

fn demo_lines() -> Vec<String> {
    vec![
        "[INFO] Initializing environment...".into(),
        "[OK] Connected to mirror.oldsoft.org".into(),
        "[INFO] Installing packages...".into(),
        "[WARN] Slow response from mirror, retrying...".into(),
        "[OK] Kernel modules compiled".into(),
        "[INFO] Finalizing configuration".into(),
    ]
}

fn resolve_cli_binary() -> String {
    // Prefer release build; fall back to debug
    let release = "target/release/install-nothing";
    if std::path::Path::new(release).exists() { return release.to_string(); }
    let debug = "target/debug/install-nothing";
    if std::path::Path::new(debug).exists() { return debug.to_string(); }
    // Last resort: rely on PATH
    "install-nothing".to_string()
}
