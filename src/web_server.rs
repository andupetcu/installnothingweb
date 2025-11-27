use std::{net::SocketAddr, sync::atomic::{AtomicUsize, Ordering}, time::Duration};

use axum::{
    extract::{Query, WebSocketUpgrade},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
};
use axum::extract::ws::{Message, WebSocket};
use serde::Deserialize;
use tokio::{io::{AsyncBufReadExt, BufReader}, net::TcpListener, process::Command, time::timeout};
use tower_http::services::ServeDir;
use tracing::{info, warn};

// Simple connection counter for rate limiting
static ACTIVE_CONNECTIONS: AtomicUsize = AtomicUsize::new(0);
const MAX_CONNECTIONS: usize = 50;

#[derive(Debug, Deserialize)]
struct WsParams {
    theme: Option<String>,
    stack: Option<String>,
    duration: Option<u64>, // seconds
    mode: Option<String>,  // "endless" or omitted
}

#[tokio::main]
async fn main() {
    // Initialize logger
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // Static files under ./web
    let static_service = ServeDir::new("web");

    let app = Router::new()
        .route("/ws", get(ws_handler))
        .route("/health", get(|| async { "ok" }))
        .fallback_service(static_service);

    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(3000);
    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().unwrap();
    info!("installer-web listening on http://{addr}");
    info!("Open in your browser to test the simulation.");

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C handler");
    info!("Shutdown signal received, stopping server...");
}

async fn ws_handler(ws: WebSocketUpgrade, Query(params): Query<WsParams>) -> impl IntoResponse {
    // Check rate limit
    let current = ACTIVE_CONNECTIONS.load(Ordering::SeqCst);
    if current >= MAX_CONNECTIONS {
        warn!("Rate limit exceeded: {} active connections", current);
        return (StatusCode::SERVICE_UNAVAILABLE, "Too many connections").into_response();
    }

    ws.on_upgrade(move |socket| websocket_stream(socket, params)).into_response()
}

async fn websocket_stream(mut socket: WebSocket, params: WsParams) {
    // Increment connection counter
    ACTIVE_CONNECTIONS.fetch_add(1, Ordering::SeqCst);

    let duration_secs = params.duration.unwrap_or(30);
    let stack = params.stack.unwrap_or_else(|| "default".to_string());
    let theme = params.theme.unwrap_or_else(|| "dracula".to_string());
    let endless = matches!(params.mode.as_deref(), Some("endless")) || duration_secs == 0;

    // Start a child process of the CLI simulator and stream its stdout
    let binary = resolve_cli_binary();
    let mut cmd = Command::new(binary);
    // Run effectively infinite cycles (duration is enforced by timeout or client disconnect)
    cmd.arg("--cycles").arg("999999999");
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

            // Spawn task to log stderr
            if let Some(stderr) = child.stderr.take() {
                tokio::spawn(async move {
                    let mut reader = BufReader::new(stderr);
                    let mut line = String::new();
                    while reader.read_line(&mut line).await.unwrap_or(0) > 0 {
                        warn!("[child stderr] {}", line.trim());
                        line.clear();
                    }
                });
            }

            let banner = if endless {
                format!("\u{001b}[36mStarting simulation (stack: {stack}, theme: {theme}, mode: ENDLESS).\u{001b}[0m")
            } else {
                format!("\u{001b}[36mStarting simulation (stack: {stack}, theme: {theme}, duration: {duration_secs}s).\u{001b}[0m")
            };
            let _ = socket.send(Message::Text(banner)).await;

            if endless {
                // Stream until client closes or child exits
                loop {
                    if let Some(r) = reader.as_mut() {
                        let mut line = String::new();
                        match timeout(Duration::from_millis(200), r.read_line(&mut line)).await {
                            Ok(Ok(0)) => break, // EOF
                            Ok(Ok(_n)) => {
                                if !line.is_empty() {
                                    if socket.send(Message::Text(line.trim_end_matches('\n').to_string())).await.is_err() {
                                        break; // client gone
                                    }
                                }
                            }
                            Ok(Err(_e)) => break,
                            Err(_) => {
                                // idle
                            }
                        }
                    } else {
                        break;
                    }
                }
            } else {
                // Duration-bound streaming
                let end_time = tokio::time::Instant::now() + Duration::from_secs(duration_secs);
                loop {
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
                            Err(_) => { /* read timed out */ }
                        }
                    } else {
                        break;
                    }
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

    // Decrement connection counter
    ACTIVE_CONNECTIONS.fetch_sub(1, Ordering::SeqCst);
}

fn resolve_cli_binary() -> String {
    // Check env var first (for Docker/production)
    if let Ok(path) = std::env::var("CLI_BINARY_PATH") {
        return path;
    }
    // Development paths
    for path in ["target/release/install-nothing", "target/debug/install-nothing"] {
        if std::path::Path::new(path).exists() {
            return path.to_string();
        }
    }
    // Fallback to PATH
    "install-nothing".to_string()
}
