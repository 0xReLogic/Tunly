# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- GitHub Actions workflows for CI/CD
- CodeQL security scanning for Rust and JavaScript/TypeScript
- Dependabot configuration for automated dependency updates
- Multi-platform release builds (Linux, macOS, Windows - x86_64 and ARM64)
- Docker multi-arch image builds
- Security policy (SECURITY.md)
- cargo-deny configuration for license and security enforcement

### Changed
- Backend code now uses English-only messages (no Indonesian)
- Fixed all Clippy warnings

### Fixed
- Clippy warnings in client and server code

## [0.1.0] - 2026-02-04

### Added
- Initial release
- Tunly server with ephemeral token mode
- Tunly client with auto-reconnect
- WebSocket-based tunnel architecture
- Session management with idle TTL
- Rate limiting on /token endpoint
- Session logging at /s/:sid/_log
- Support for self-hosted deployment
- Docker support
- Next.js frontend with Material-UI

### Security
- Ephemeral tokens with IP binding
- 5-minute token TTL
- Single-use tokens
- Session isolation
- Security headers (cache-control, x-robots-tag, referrer-policy)

[Unreleased]: https://github.com/0xReLogic/Tunly/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/0xReLogic/Tunly/releases/tag/v0.1.0
