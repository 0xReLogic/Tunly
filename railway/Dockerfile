# Use minimal Alpine Linux image
FROM alpine:latest

# Install ca-certificates for HTTPS support
RUN apk --no-cache add ca-certificates wget

# Create non-root user
RUN addgroup -g 1001 -S tunnel && \
    adduser -S tunnel -u 1001

# Copy the pre-built binary
COPY tunnel-server /usr/local/bin/tunnel-server

# Copy static files
COPY static/ /usr/local/bin/static/

# Make binary executable
RUN chmod +x /usr/local/bin/tunnel-server

# Switch to non-root user
USER tunnel

# Set working directory
WORKDIR /usr/local/bin

# Expose port (will be overridden by Fly.io)
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD wget --no-verbose --tries=1 --spider http://localhost:8080/health || exit 1

# Run the server
CMD ["./tunnel-server"] 