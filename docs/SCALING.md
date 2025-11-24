# Production Scaling Guide

## Overview
Target: handle ~10,000 concurrent viewers smoothly. Avoid per-user processes; generate output in-process, batch writes, and scale horizontally behind a WS-capable load balancer.

## Architecture
- Extract simulator core into a Rust library producing async event streams (lines/bytes).
- Server stays stateless; 1 WebSocket = 1 Tokio task.
- Prefer client-only (WASM/TS) for maximal scale; keep server for links/telemetry.

## Simulation Engine
- Preload logs/templates at startup (`Arc<Vec<Bytes>>`) to avoid per-conn IO.
- Deterministic PRNG per session (seed = sessionId) for variety without overhead.
- Use `tokio::time::interval` (not sleeps) for timers; configurable rate per profile.

## WebSocket Efficiency
- Enable permessage-deflate (server + proxy) and keepalive pings.
- Batch lines: send every 100–200 ms instead of per-line messages.
- Backpressure: bounded channel per session; coalesce or drop oldest on slow clients.

## Limits & Protection
- Cap duration (even “Endless” has a server max, e.g., 2h).
- Rate limit session starts per IP (token bucket).
- Idle timeout and dead-connection cleanup (ping/pong).
- Concurrency caps per node to protect CPU/memory.

## Horizontal Scaling
- Front with NGINX/Envoy/ALB supporting WS; sticky per connection.
- Autoscale by CPU or custom metric (messages/sec).
- Blue/green or rolling deploy; graceful shutdown drains WS sessions.

## Resource Estimates (rule-of-thumb)
- 10 lines/sec × 100 bytes ≈ 1 KB/s per user → 10k users ≈ 10 MB/s (with batching/compression) or ~100 MB/s raw.
- Target <100 KB RAM per session → ~1 GB for 10k (plus app overhead).

## Implementation Steps
1. Create `crates/core`: configs + `Stream<Item = Bytes>` API.
2. Replace child `Command` with in-process generator; preload assets.
3. Add batching/compression and per-session backpressure.
4. Add rate limits, caps, health, Prometheus metrics.
5. Load test (k6/Locust WS) to tune batch sizes and rates.

## Alternative: Client-Only
Port generator to TS or compile core to WASM; run entirely in-browser with `xterm.js`. Server becomes static hosting + shareable URLs. Scales near-infinitely; no WS streaming cost.

