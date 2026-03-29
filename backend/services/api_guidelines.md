# Stellar Insights API Guidelines

## Current Strategy (Post-GraphQL Deprecation)
**REST API (Primary - /api/v1)**:
- **Public API**: All endpoints under `/api` (anchors, corridors, RPC, metrics, etc.).
- **Use Cases**:
  | Use Case | REST | Notes |
  |----------|------|-------|
  | Simple queries | ✅ Primary | Cached lists (anchors, corridors), RPC (payments/trades). |
  | Complex analytics | ✅ | Fee bumps, account merges, liquidity pools. |
  | Real-time (WS) | ✅ | `/ws`, `/ws/alerts`. |
  | Admin/Metrics | ✅ Auth | Pool metrics, cache stats. |
  | Caching/CDN | ✅ | ETag, compression, Redis-backed. |

**GraphQL (Deprecated)**:
- Disabled by default (`default = []` in Cargo.toml).
- Enable: `cargo run --features graphql-deprecated`.
- **Migration**: No active usage found; REST covers all needs. Remove in 3 months.

## Shared Business Logic
- Use `services/` (event_indexer, liquidity_pool_analyzer).
- DB: Direct SQLx queries (add services/ for reuse).

## OpenAPI Docs
- `/swagger-ui` for REST exploration.

## Deprecation Timeline
- Now: Feature-flagged OFF.
- 3 months: Full removal.

