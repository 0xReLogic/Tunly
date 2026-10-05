FROM alpine:3.20

# Install ca-certificates for HTTPS, curl, and build tools
RUN apk add --no-cache ca-certificates curl

# Build Tunly from source
FROM rust:latest AS builder

WORKDIR /build

# Clone the repository
RUN git clone https://github.com/0xReLogic/Tunly.git .

WORKDIR /build/backend

# Build release binary
RUN cargo build --release

# Runtime stage
FROM alpine:3.20

RUN apk add --no-cache ca-certificates

WORKDIR /app

# Copy binaries from builder
COPY --from=builder /build/backend/target/release/tunly-server /app/
COPY --from=builder /build/backend/target/release/tunly-client /app/
COPY --from=builder /build/backend/target/release/tunly /app/

RUN chmod +x /app/tunly /app/tunly-server /app/tunly-client

# Expose default port
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD wget --no-verbose --tries=1 --spider http://localhost:8080/healthz || exit 1

# Run server by default
ENTRYPOINT ["/app/tunly-server"]
CMD ["--bind", "0.0.0.0:8080"]

