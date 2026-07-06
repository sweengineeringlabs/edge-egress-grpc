//! `ValidationRequest` — request marker for [`crate::api::Validator::validate`].

/// Request marker for [`crate::api::Validator::validate`].
///
/// The validation target is `&self`; this type carries no payload — its
/// sole purpose is to satisfy the api/ convention that every trait method
/// accepts exactly one `*Request` parameter.
#[derive(Debug, Clone, Copy, Default)]
pub struct ValidationRequest;
