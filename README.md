# edge-transport-grpc-egress

> **TLDR:** Outbound gRPC client for swe-edge — Tonic-backed with trace propagation, retry, and circuit-breaker decorators behind one `GrpcEgress` trait. See [Overview](scm/docs/README.md) for details.

Outbound gRPC transport for the `swe-edge` stack.

Provides the `GrpcOutbound` port trait, a `tonic`-backed implementation
(`TonicGrpcClient`), optional resilience config, and an outbound interceptor
chain with a built-in `TraceContextInterceptor`.

## Usage

```rust
use edge_transport_grpc_egress::{GrpcChannelConfig, create_transport_from_config};

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
| `edge-transport-grpc-egress-auth-bearer` | Outbound bearer token injection |
| `edge-transport-grpc-egress-auth-mtls` | Mutual TLS client certificates |
| `edge-transport-grpc-egress-authz` | Method-level ACL policy |
| `edge-transport-grpc-egress-retry` | Retry with exponential backoff |
| `edge-transport-grpc-egress-breaker` | Circuit breaker |

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
