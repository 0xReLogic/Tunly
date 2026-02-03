# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

[Unreleased]: https://github.com/0xReLogic/Tunly/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/0xReLogic/Tunly/releases/tag/v0.1.0
