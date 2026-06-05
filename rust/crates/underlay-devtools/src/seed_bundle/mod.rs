//! Seed-data OCI bundle build/publish/pull.
//!
//! Seed bundles package SQL files from a seed-bundle directory into OCI bundles
//! for portable distribution. Each SQL file becomes a DataChunk layer with ordering
//! annotations. Unlike migration bundles, seed bundles have no media shards or
//! decision indexes -- just SQL.
//!
//! Reuses the same `BundlePackage` JSON envelope and local OCI store infrastructure
//! as migration bundles, with a distinct artifact type.

mod build;
mod model;
mod package;
mod publish;
mod pull;

pub use build::seed_bundle_build;
pub use model::{
    SeedBundleBuildOptions, SeedBundleBuildReport, SeedBundlePullOptions, SeedBundlePullReport,
};
pub use publish::seed_bundle_publish;
pub use pull::seed_bundle_pull;
