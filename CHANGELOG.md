# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-03-03

### Added
- Initial release of Feishu SDK Rust
- Core functionality
  - HTTP client based on reqwest
  - Token management (app_access_token, tenant_access_token)
  - Token caching with InMemoryCache
  - Pluggable cache, logger, and HTTP client interfaces
- Event system
  - Event models for im.v1, contact.v3, application.v6, approval.v4
  - EventDispatcher for routing events to handlers
  - Signature verification and decryption support
  - Challenge handling for webhook verification
- Card system
  - CardAction model and CardActionHandler
  - SHA1 signature verification
  - Toast and update card responses
- WebSocket support
  - WebSocket client with tokio-tungstenite
  - Heartbeat and reconnection logic
  - Binary message protocol
- HTTP server (optional `server` feature)
  - Salvo-based server for webhooks
  - Event endpoint (`/webhook/event`)
  - Card endpoint (`/webhook/card`)
- Configuration
  - ConfigBuilder for flexible configuration
  - Support for marketplace apps
  - Helpdesk credentials
  - Request timeout and custom headers
  - Config validation
- Client options
  - with_logger, with_log_level, with_token_cache
  - with_http_client, with_request_timeout, with_headers
- Utilities
  - Serializable trait with JsonSerializer
  - File upload/download helpers
  - Content type guessing
  - File size formatting
- Examples
  - get_token: Get tenant access token
  - send_message: Send a message
  - get_user: Get user information
  - call_endpoint: Use typed API wrappers
  - event_server: Event handling server
  - card_handler: Card action handler
  - websocket_client: WebSocket client example
- Documentation
  - Complete README with quick start guide
  - API reference in rustdoc
  - Usage examples

### Changed
- Migrated from axum to Salvo web framework

### Security
- Token caching with expiration
- Signature verification for events and cards
- Event body decryption support

## [Unreleased]

### Added
- CI workflows (`ci.yml`, `release.yml`) for lint, test, docs, coverage, and publish.
- `tools/generate_from_go.py` to generate typed request/response model stubs from endpoint catalog.
- Additional event model coverage (`drive`, `calendar`).
- `docs/USAGE.md`, `docs/FAQ.md`, and `docs/MIGRATION.md`.
- Full application example (`examples/full_app.rs`).
- App ticket resend retry helpers.
- Config-level serializer customization.

### Changed
- Core HTTP execution now respects injected custom `HttpClient`.
- Request execution includes configurable retry behavior and richer request/response debug logs.
- WebSocket protobuf dependency is explicitly declared behind `websocket` feature.

### Compatibility
- Backward-compatible additive release planned as `0.1.1`.
