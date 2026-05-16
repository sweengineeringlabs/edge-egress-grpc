# swe-edge-egress-grpc

Outbound gRPC transport for the `swe-edge` stack.

Provides the `GrpcOutbound` port trait, a `tonic`-backed implementation
(`TonicGrpcClient`), optional resilience config, and an outbound interceptor
chain with a built-in `TraceContextInterceptor`.

## Usage

```rust
use swe_edge_egress_grpc::{GrpcChannelConfig, create_transport_from_config};

let config    = GrpcChannelConfig::new("https://api.example.com:443");
let transport = create_transport_from_config(&config).await?;
// transport: Arc<dyn GrpcOutbound>
```

## Public surface (`saf/`)

| Export | Purpose |
|--------|---------|
| `GrpcOutbound` | Port trait — `send(req)` → `GrpcOutboundResult<Resp>` |
| `TonicGrpcClient` | `tonic`-backed implementation |
| `GrpcChannelConfig` | Endpoint, timeout, TLS, and keep-alive settings |
| `GrpcOutboundError` / `GrpcOutboundResult` | Error and result types |
| `create_transport_from_config(config)` | Factory — returns `Arc<dyn GrpcOutbound>` |

## Auth interceptors (feature crates)

Auth, mTLS, and authorisation interceptors live in sibling crates under
`egress/grpc/main/features/`:

| Crate | Purpose |
|-------|---------|
| `swe-edge-egress-grpc-auth-bearer` | Outbound bearer token injection |
| `swe-edge-egress-grpc-auth-mtls` | Mutual TLS client certificates |
| `swe-edge-egress-grpc-authz` | Method-level ACL policy |
| `swe-edge-egress-grpc-retry` | Retry with exponential backoff |
| `swe-edge-egress-grpc-breaker` | Circuit breaker |

## Crate layout (SEA)

| Layer | Path | Role |
|-------|------|------|
| `api/` | `src/api/` | Traits, value objects, error types |
| `core/` | `src/core/` | `pub(crate)` implementations |
| `saf/` | `src/saf/` | Factory functions + curated re-exports |

## Building

```bash
cd egress/grpc
cargo build
cargo test
cargo clippy -- -D warnings
```
