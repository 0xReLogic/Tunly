# Tunly

Pioneering HTTP over QUIC (MoQ) tunneling.

- Open-source tunneling with a Media over QUIC (MoQ) HTTP bridge proof-of-concept.
- Local round-trip latency target: < 300 ms (measured ~63 ms on localhost).
- Built in Rust.

## Quick Start (Local MoQ POC)

1) Start a local HTTP server (port 9000)
   python3 -m http.server 9000

2) Start moq-relay (dev)
   cd ~/.cargo/git/checkouts/moq-*/rs/moq-relay
   cargo run -- cfg/dev.toml

3) Run the MoQ server bridge (forwards to 127.0.0.1:9000)
   cd backend
   RUST_LOG=moq_lite=debug,moq_native=debug \
   cargo run --bin tunly-moq-server -- --remote http://127.0.0.1:4443 --host 127.0.0.1 --port 9000

4) Run the MoQ client bridge (HTTP entry at 127.0.0.1:8000)
   cd backend
   RUST_LOG=moq_lite=debug,moq_native=debug \
   cargo run --bin tunly-moq-client -- --remote http://127.0.0.1:4443 --local 127.0.0.1:8000

5) Measure RTT
   curl -w time_total:%{time_total}n -o /dev/null -s http://127.0.0.1:8000/

Notes:
- python3 http.server returns 501 for POST; use an echo server (FastAPI/httpbin) to test POST/PUT.
- This is a PoC, not production-ready.

## Legacy docs

The previous long-form README is in docs/LEGACY.md.
