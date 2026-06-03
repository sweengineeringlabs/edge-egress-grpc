//! `ApplicationConfigBuilder` — maps to `config/application.toml`.

/// Config builder corresponding to `config/application.toml`.
#[expect(
    dead_code,
    reason = "SEA api/ type alias — re-exported name, not directly referenced yet"
)]
pub type ApplicationConfigBuilder = swe_edge_configbuilder::ConfigBuilderImpl;
