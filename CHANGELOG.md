# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2026-02-05

### Added
- Transparent Tunnel Compression using Zlib (Deflate) for reduced bandwidth usage
- JWT-based ephemeral tokens with embedded metadata (session ID, IP, expiry)
- Automated test coverage for JWT authentication flow and session hijacking protection
- HTTP/2 support with multiplexing in tunly-client for improved concurrency
- Integration test suite in security_tests.rs including high-concurrency flood testing
- Support for library-based architecture to enable unit testing
- Structured logging using `tracing` and `tracing-subscriber` for production observability
- Persistent connection pooling in tunly-client to improve local proxy performance
- Prometheus metrics exporter at `/metrics` for real-time monitoring of requests, latency, and sessions

### Security
- Added Shared Secret authentication for `/token` endpoint to restrict access to authorized frontends
- Migrated from raw base64 tokens to structured JWTs for better security and flexibility
- Implemented 2MB request body limit for all proxy requests to prevent memory exhaustion
- Implemented per-IP rate limiting (120 req/60s) for proxy routes to mitigate DDoS attacks
- Resolved unlimited memory usage vulnerability by enforcing strict payload limits

### Changed
- Refactored `ProxyRequest` and `ProxyResponse` to support on-the-fly compression for large payloads
- Enabled custom JWT secret configuration via `--jwt-secret` or `TUNLY_JWT_SECRET` env var
- Improved `/token` endpoint to return structured JSON response using `axum::Json`
- Refactored server logic into a library crate (lib.rs) for improved testability and modularity
- Enabled response compression (gzip/brotli) in tunly-client by removing client-side restrictions
- Improved error handling for 413 Payload Too Large and 429 Too Many Requests scenarios
- Replaced standard `println!` with structured logging macros across all backend components

## [0.1.0] - 2026-02-04

### Added
- Initial release of Tunly HTTP tunnel solution
- Tunly server with ephemeral and fixed token modes
- Tunly client with auto-reconnect and exponential backoff
- WebSocket-based tunnel architecture
- Session management with idle TTL (10 minutes)
- Rate limiting on /token endpoint (10 req/60s per IP)
- Session logging at /s/:sid/_log (last 50 requests)
- Support for self-hosted deployment on VPS/cloud
- Docker support with multi-arch builds (amd64, arm64)
- Next.js frontend with Material-UI
- GitHub Actions CI/CD workflows
- CodeQL security scanning for Rust and JavaScript/TypeScript
- Dependabot configuration for automated dependency updates
- Multi-platform release builds (6 platforms)
- cargo-deny configuration for license and security enforcement
- Extreme binary size optimization (LTO, opt-level=z, UPX compression)

### Security
- Ephemeral tokens with IP binding and session validation
- 5-minute token TTL with single-use enforcement
- Session isolation and automatic garbage collection
- Security headers (cache-control, x-robots-tag, referrer-policy)
- Removed sensitive header logging to prevent log injection
- Token authentication via Authorization header (Bearer)

### Changed
- Backend code uses English-only messages
- Updated to rand 0.9 and axum 0.8 APIs
- Added MIT OR Apache-2.0 dual license

### Fixed
- All Clippy warnings resolved
- Cargo-deny license checks passing
- Security vulnerabilities addressed (log injection, cleartext logging)

[Unreleased]: https://github.com/0xReLogic/Tunly/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/0xReLogic/Tunly/releases/tag/v0.2.0
[0.1.0]: https://github.com/0xReLogic/Tunly/releases/tag/v0.1.0
