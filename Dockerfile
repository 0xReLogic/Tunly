FROM alpine:3.20

# Install ca-certificates for HTTPS and curl
RUN apk add --no-cache ca-certificates curl

# Set version (can be overridden at build time)
ARG TUNLY_VERSION=v0.3.1
ARG TARGETARCH

# Create app directory
WORKDIR /app

# Download and extract the appropriate binary based on architecture
RUN if [ "$TARGETARCH" = "amd64" ]; then \
      BINARY="tunly-linux-x86_64.tar.gz"; \
    elif [ "$TARGETARCH" = "arm64" ]; then \
      BINARY="tunly-linux-aarch64.tar.gz"; \
    else \
      echo "Unsupported architecture: $TARGETARCH" && exit 1; \
    fi && \
    curl -L "https://github.com/0xReLogic/Tunly/releases/download/${TUNLY_VERSION}/${BINARY}" -o tunly.tar.gz && \
    tar -xzf tunly.tar.gz && \
    rm tunly.tar.gz && \
    chmod +x tunly tunly-server tunly-client

# Expose default port
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD curl -f http://localhost:8080/healthz || exit 1

# Run server by default
ENTRYPOINT ["/app/tunly-server"]
CMD ["--bind", "0.0.0.0:8080"]
