FROM --platform=$BUILDPLATFORM alpine:3.20 AS downloader

ARG TARGETPLATFORM
ARG BUILDPLATFORM
ARG TUNLY_VERSION=v0.3.0

WORKDIR /tmp

RUN apk add --no-cache curl tar && \
    if [ "$TARGETPLATFORM" = "linux/amd64" ]; then \
      BINARY="tunly-linux-x86_64.tar.gz"; \
    elif [ "$TARGETPLATFORM" = "linux/arm64" ]; then \
      BINARY="tunly-linux-aarch64.tar.gz"; \
    else \
      echo "Unsupported platform: $TARGETPLATFORM" && exit 1; \
    fi && \
    curl -L "https://github.com/0xReLogic/Tunly/releases/download/${TUNLY_VERSION}/${BINARY}" -o tunly.tar.gz && \
    tar -xzf tunly.tar.gz && \
    rm tunly.tar.gz

# Runtime stage - minimal image
FROM alpine:3.20

RUN apk add --no-cache ca-certificates

WORKDIR /app

# Copy all binaries from downloader (tunly, tunly-server, tunly-client)
COPY --from=downloader /tmp/tunly /tmp/tunly-server /tmp/tunly-client /app/

RUN chmod +x /app/tunly-server /app/tunly-client /app/tunly

# Expose default port
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD wget --no-verbose --tries=1 --spider http://localhost:8080/healthz || exit 1

# Run server by default
ENTRYPOINT ["/app/tunly-server"]
CMD ["--bind", "0.0.0.0:8080"]

