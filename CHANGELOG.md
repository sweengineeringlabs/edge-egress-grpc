# Changelog

All notable changes to `swe-edge-egress-grpc` are documented here.

## Unreleased

- Added `ResilienceConfig` on `GrpcChannelConfig` for config-driven retry + circuit breaker
- Added `ResourceExhaustedContext` discrimination (Capacity / RateLimit / HardQuota)
- Added `Retry-After` header extraction via `parse_retry_after_hint`
- Added `GrpcChannelConfigError::Config` variant for resilience validation errors
- Added `half_open_probe_count` to `CircuitBreaker`
