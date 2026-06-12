# ADR-001: Egress gRPC Security Specialisations

**Status:** Accepted  
**Date:** 2026-06-12  
**Parent:** [ADR-015 — Three-Tier Security Layer Architecture](../../../../../docs/3-architecture/adr/ADR-015-security-layer-architecture.md)  
**See also:** [ADR-016 — gRPC Security Specialisations](../../../../../docs/3-architecture/adr/ADR-016-grpc-security-specialisations.md)  
**Deciders:** phdsystems  
**Affects:** `egress/grpc` — auth-bearer, transport

---

## Context

This ADR records which security contracts are specific to `egress/grpc` and why, and which are delegated to `swe-edge-security`. It is the local counterpart to ADR-015/ADR-016 for engineers working in this workspace.

Egress gRPC security has one directional mandate:

> Attach the correct outbound credential (signed JWT or mTLS client certificate) to a gRPC channel or call before it leaves the process boundary.

---

## Egress gRPC–Specific Concepts (must stay here)

### `BearerEgressConfig` and `BearerEgressInterceptor`

Mints and signs outbound JWTs using `BearerSecret` (private key material only — see R6) and injects them as `Authorization: Bearer` gRPC metadata. gRPC-specific because:

- Writes to gRPC metadata, not HTTP headers
- The interceptor is tonic client-side middleware
- JWT minting (`exp`, `iat` auto-set) is an outbound-only concern

### `BearerSecret` (egress variant — private key only)

`BearerSecret::Rs256 { private_pem }` — holds the **private** key for signing.  
This must never be the same type as `ingress/grpc`'s `BearerSecret::Rs256 { public_pem }`.  
See **R6** in ADR-015: unifying them would allow a signing key to reach a verification path, or a verification key to be used for signing — both silent failures.

### `MtlsConfig`

Client certificate + private key + optional pinned CA bundle for outbound tonic channels. Stays here because:

- It wires into `tonic::transport::Channel`, not `reqwest::ClientBuilder`
- The CA pinning field (`ca_pem_path`) serves server-certificate validation on the client side — a different role from the ingress server's client-CA field

---

## Concepts Delegated to Shared (`swe-edge-security`)

| Local item | After ADR-015 Step 5 |
|-----------|----------------------|
| `struct JwtClaims` (in auth-bearer) | `pub use swe_edge_security::JwtClaims` |
| mTLS load/parse errors | Use `swe_edge_security::TlsConfigError` variants |

---

## Error Conversion Requirement

All security errors in this workspace must implement `Into<swe_edge_security::SecurityError>`:

| Error | Variant | Status |
|-------|---------|--------|
| `BearerAuthError` | `SecurityError::Auth(String)` | Add in Step 5 |
| `MtlsConfig` load/parse errors | Via `swe_edge_security::TlsConfigError` → `SecurityError::Tls(String)` | Add in Step 5 |

---

## What Egress gRPC Security Must Never Do

- Verify inbound bearer tokens or inspect caller identity
- Hold public key material for verification in `BearerSecret` — only `private_pem` for signing
- Perform tenant resolution from request metadata
- Import from `ingress/*` crates
- Redefine `JwtClaims` locally — always `pub use swe_edge_security::JwtClaims`
