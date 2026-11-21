# Changelog

All notable changes to this project are documented here. Versions follow [Semantic Versioning](https://semver.org).

## [0.1.0] - 2026-11-21

### Added
- CRUD REST API for `/todos` and a `/health` check, built on Axum 0.8
- SQLite storage with SQLx migrations that run on startup
- Title validation and JSON error responses
- Request tracing with `tower-http`
- Integration tests and CI (fmt, clippy, tests, Docker build)
- Release binaries for Linux, macOS and Windows, and a Docker image on GHCR
- Logo, banner and README diagrams

[0.1.0]: https://github.com/anqorithm/rust-axum-api/releases/tag/v0.1.0
