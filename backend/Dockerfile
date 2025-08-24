# ---------- Builder stage ----------
FROM rust:1-slim AS builder
WORKDIR /app
RUN apt-get update \
 && apt-get install -y --no-install-recommends \
    build-essential \
    pkg-config \
    libssl-dev \
    ca-certificates \
 && rm -rf /var/lib/apt/lists/*

# Pre-copy manifest to leverage Docker layer caching
# NOTE: Build context is the repository root on Koyeb. Since this Dockerfile
# lives in backend/, we must copy from backend/<path> here.
COPY backend/Cargo.toml backend/Cargo.lock ./
COPY backend/src/ src/

# Build only the server binary used on Render
RUN cargo build --release --locked --bin tunly-server

# ---------- Runtime stage ----------
FROM debian:bookworm-slim AS runtime

# Minimal runtime deps
RUN apt-get update \
 && apt-get install -y --no-install-recommends \
    ca-certificates \
    openssl \
 && rm -rf /var/lib/apt/lists/* \
 && useradd -m -u 10001 appuser
WORKDIR /app

# Copy binary
COPY --from=builder /app/target/release/tunly-server /app/tunly-server

ENV RUST_LOG=info
EXPOSE 8080

# Render injects PORT automatically; we use --host 0.0.0.0 and read PORT from env via clap
USER appuser
CMD ["/app/tunly-server", "--host", "0.0.0.0"]
