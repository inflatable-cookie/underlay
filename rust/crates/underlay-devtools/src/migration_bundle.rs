mod build;
pub(crate) mod local_store;
mod media_shards;
mod model;
mod output;
mod package;
mod remote_registry;
mod run;

pub use build::migration_bundle_build;
pub use model::{
    BundleBuildOptions, BundleBuildReport, BundlePublishOptions, BundlePublishReport,
    BundlePullOptions, BundlePullReport, BundleRunOptions, BundleRunReport, MigrationBundleError,
    MigrationBundleRef,
};
pub use run::{migration_bundle_publish, migration_bundle_pull, migration_run};

pub(super) use output::write_pulled_outputs;
pub(super) use package::{decode_package, sha256_digest, validate_bundle_package, BundlePackage};

const SHA256_PREFIX: &str = "sha256:";
const DEFAULT_MEDIA_SHARD_MAX_BYTES: u64 = 16 * 1024 * 1024;
const OCI_MANIFEST_MEDIA_TYPE: &str = "application/vnd.oci.image.manifest.v1+json";

#[cfg(test)]
#[path = "tests/migration_bundle_tests.rs"]
mod tests;
