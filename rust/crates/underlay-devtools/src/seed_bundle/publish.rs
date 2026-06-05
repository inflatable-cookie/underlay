use crate::migration_bundle::{BundlePublishOptions, BundlePublishReport, MigrationBundleError};

/// Publish a seed bundle to the local OCI store or remote registry.
///
/// This reuses the migration bundle publish path since the `BundlePackage`
/// format is identical.
pub fn seed_bundle_publish(
    options: &BundlePublishOptions,
) -> Result<BundlePublishReport, MigrationBundleError> {
    crate::migration_bundle::migration_bundle_publish(options)
}
