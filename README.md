# swe-edge-egress-grpc

Outbound gRPC transport for the SWE edge stack.

Provides the `GrpcOutbound` port trait, a tonic-backed implementation
(`TonicGrpcClient`), an optional resilience layer (`ResilienceConfig`),
and an outbound interceptor chain with a built-in `TraceContextInterceptor`.

## Usage

```rust
use swe_edge_egress_grpc::{GrpcChannelConfig, create_transport_from_config};

let config = GrpcChannelConfig::new("https://api.example.com:443");
let transport = create_transport_from_config(&config)?;
```

## Crate layout (SEA)

| Layer | Path | Role |
|---|---|---|
| api | `src/api/` | Traits, value objects, error types |
| core | `src/core/` | Implementations |
| saf | `src/saf/` | Factory functions + curated re-exports |
| gateway | `src/gateway/` | Crate entry point |
