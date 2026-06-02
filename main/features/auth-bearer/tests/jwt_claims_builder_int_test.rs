//! Coverage stub for `api/bearer/jwt/jwt_claims_builder.rs`.

use swe_edge_egress_grpc_auth_bearer::JwtClaimsBuilder;

/// @covers: JwtClaimsBuilder — fluent builder produces valid claims
#[test]
fn bearer_struct_jwt_claims_builder_builds_valid_claims_int_test() -> Result<(), String> {
    let claims = JwtClaimsBuilder::new()
        .iss("iss")
        .aud("aud")
        .sub("sub")
        .exp(9999999999)
        .iat(1000000000)
        .build()?;
    assert_eq!(claims.iss, "iss");
    assert_eq!(claims.sub, "sub");
    Ok(())
}
