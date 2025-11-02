# Tunly

[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux%20%7C%20macOS-blue?style=for-the-badge)](https://github.com/0xReLogic/Tunly/releases)

Open-source tunneling now pioneering HTTP over QUIC (Media over QUIC, MoQ) with a focused, low-latency bridge POC.

---

## Motivation

Traditional HTTP tunnels work, but we’re exploring the next step: Media over QUIC (MoQ). Tunly’s MoQ POC aims for sub‑300ms local RTT (measured ~63ms on localhost) while retaining the simplicity of a no-nonsense toolchain.

---

## Key Features (POC)

- HTTP → MoQ → HTTP bridge using `tunly-moq-client` and `tunly-moq-server`.
- Works with a local `moq-relay` (dev config) using publish/subscribe tracks and per‑request IDs.
- Simple envelopes for request/response (headers + body) over MoQ Groups/Frames.
- QUIC-based transport with multiplexed uni-streams (reduced head‑of‑line blocking vs TCP/WS tunnels).

---

## Why MoQ (vs classic WebSocket/TCP tunnels)

- QUIC multiplexing: concurrent requests don’t block each other on a single TCP stream.
- Pub/Sub model: broadcasts/tracks fit fan‑out and subscription patterns naturally.
- Framed delivery: Groups/Frames map cleanly to discrete HTTP messages over MoQ.
- Low latency locally: measured ~63 ms RTT in our E2E POC.

---

## Components

- `moq-relay` — local relay (dev cfg) for MoQ pub/sub.
- `tunly-moq-client` — HTTP entry (e.g., `127.0.0.1:8000`) → publish request envelopes to MoQ.
- `tunly-moq-server` — subscribe requests → forward to local HTTP target (e.g., `127.0.0.1:9000`) → publish response envelopes.

---

## How It Works (MoQ)

- Client publishes request to `anon/tunly-http-req/{id}` on track `http`.
- Server subscribes, reads one Group/Frame → converts to HTTP → forwards to local target.
- Server publishes response to `anon/tunly-http-resp/{id}` on track `http`.
- Client subscribes and matches `id`, then returns as normal HTTP response.

MoQ primitives used: Origin, Broadcast, Track, Group, Frame.

---

## When to Use

- Low‑latency demos where tail latency matters and multiple requests run in parallel.
- Testing chatty APIs (many small requests) without TCP head‑of‑line issues typical of single‑WS tunnels.
- Exploring media‑style delivery semantics (pub/sub, groups/frames) for non‑media HTTP payloads.

---

## Quick Start (Local MoQ POC)

1) Start a local HTTP server (port 9000)

```bash
python3 -m http.server 9000
```

2) Start moq-relay (dev)

```bash
cd ~/.cargo/git/checkouts/moq-*/rs/moq-relay
cargo run -- cfg/dev.toml
```

3) Run the MoQ server bridge (forwards to 127.0.0.1:9000)

```bash
cd backend
RUST_LOG=moq_lite=debug,moq_native=debug \
cargo run --bin tunly-moq-server -- --remote http://127.0.0.1:4443 --host 127.0.0.1 --port 9000
```

4) Run the MoQ client bridge (HTTP entry at 127.0.0.1:8000)

```bash
cd backend
RUST_LOG=moq_lite=debug,moq_native=debug \
cargo run --bin tunly-moq-client -- --remote http://127.0.0.1:4443 --local 127.0.0.1:8000
```

5) Measure RTT

```bash
curl -w 'time_total:%{time_total}\n' -o /dev/null -s http://127.0.0.1:8000/
```

Notes:

- `python3 -m http.server` replies 501 for POST; to test POST/PUT use an echo server (e.g., FastAPI/httpbin).
- This is a POC, not production-ready.

---

## Troubleshooting

- 504/timeout: ensure `[moq] publish/announced/recv` logs appear on both client and server; verify moq-relay is running; verify ports.
- POST 501: use an echo server (FastAPI/httpbin) instead of `python3 -m http.server`.

---

## Status & Roadmap

- Status: POC (local). Local RTT target < 300 ms (measured ~63 ms).
- Next: hardening, auth, multi-hop relay, deployment guides.

---

## Logs

- Client/Server emit MoQ bridge logs like:

```
[moq] publish req: anon/tunly-http-req/{id}
[moq] announced req: anon/tunly-http-req/{id}
[moq] recv request id={id} bytes={n}
[moq] publish resp: anon/tunly-http-resp/{id}
[moq] announced resp: anon/tunly-http-resp/{id}
[moq] recv response id={id} bytes={n}
```

---

## License

MIT — see `LICENSE` for details.
