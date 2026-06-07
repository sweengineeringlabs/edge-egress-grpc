# swe-edge-egress-grpc

## WHAT

Outbound gRPC client for swe-edge — Tonic-backed egress with resilience decorators (retry,
circuit-breaker, failover) and trace context propagation.

Key capabilities:

- **`GrpcEgress`** — core trait: makes outbound gRPC calls (unary + streaming) with optional resilience
- **`ResilientGrpcClientPort`** — failover/circuit-breaker extension trait for multi-endpoint scenarios
- **`GrpcRequest`** / **`GrpcResponse`** — value objects: metadata, compression mode, status codes
- **`GrpcChannelConfig`** — typed config VO: endpoint, timeout, TLS, keep-alive (loaded from TOML)
- **`TonicGrpcClient`** — concrete Tonic implementation with built-in `TraceContextInterceptor` for distributed trace propagation
- Workspace crates: `transport`, `auth-bearer`, `retry`, `breaker`, `resilient`

## WHY

| Problem | Solution |
|---------|----------|
| Tonic channel setup (TLS, keepalive, deadlines) copy-pasted across services | `GrpcChannelConfig` + `TonicGrpcClient::from_config()` — all channel config in TOML |
| Trace headers dropped on outbound gRPC calls, breaking distributed tracing | `TraceContextInterceptor` is built into the default client; trace context propagated automatically |
| Circuit-breaking and retry implemented per-client | `retry` and `breaker` workspace crates apply as decorators on `GrpcEgress`; no handler changes |
| Multi-endpoint failover logic duplicated | `ResilientGrpcClientPort` provides a single failover contract; `resilient` workspace crate implements it |
| Diamond dep conflicts when gRPC client types change | One crate, one tag — all consumers pin the same version; kgraph detects conflicts pre-commit |
