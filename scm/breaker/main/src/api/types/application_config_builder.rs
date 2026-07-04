//! `ApplicationConfigBuilder` — maps to `config/application.toml`.

/// Config builder corresponding to `config/application.toml`.
///
/// Wraps the external `swe_edge_configbuilder::ConfigBuilderImpl` so api/
/// never references a foreign crate type directly — conversions and the
/// delegating `build_loader` method live in `core/`.
pub struct ApplicationConfigBuilder(pub(crate) swe_edge_configbuilder::ConfigBuilderImpl);
