//! Coverage for grpc/auth-bearer/src/api/jwt_claims.rs (interface counterpart doc module).

/// @covers: jwt_claims — BearerAuthError implements std::error::Error
#[test]
fn test_jwt_claims_api_module_documents_contract() {
    use swe_edge_egress_grpc_auth_bearer::BearerAuthError;
    let e = BearerAuthError::MissingHeader;
    let _ = format!("{e}");
    let _: &dyn std::error::Error = &e;
}
