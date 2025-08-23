# ---------- Builder stage ----------
FROM rust:1-slim AS builder
WORKDIR /app

# Pre-copy manifest to leverage Docker layer caching
COPY Cargo.toml Cargo.lock ./
COPY src/ src/

# Build only the server binary used on Render
RUN cargo build --release --bin tunly-server

# ---------- Runtime stage ----------
FROM debian:bookworm-slim AS runtime

# Minimal runtime deps
RUN useradd -m -u 10001 appuser
WORKDIR /app

# Copy binary
COPY --from=builder /app/target/release/tunly-server /app/tunly-server

ENV RUST_LOG=info

# Render injects PORT automatically; we use --host 0.0.0.0 and read PORT from env via clap
USER appuser
CMD ["/app/tunly-server", "--host", "0.0.0.0"]
