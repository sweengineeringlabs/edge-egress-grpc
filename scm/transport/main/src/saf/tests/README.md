# saf/tests/

This directory exists to satisfy the `impl_public_tests_external` arch audit
check, which requires `main/src/saf/tests/` to be present. It holds no Rust
source — all public functions in saf/ (the factory `create` methods,
`TransportConstruction`) are tested externally in the crate-root `tests/`
directory (e.g. `tests/transport_construction_svc_factory_int_test.rs`,
`tests/grpc_egress_svc_factory_int_test.rs`), exercised as an external
consumer would import them.
