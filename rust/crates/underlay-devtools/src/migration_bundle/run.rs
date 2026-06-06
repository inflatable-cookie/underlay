use super::{
    decode_package, local_store, remote_registry, sha256_digest, validate_bundle_package,
    BundlePublishOptions, BundlePublishReport, BundlePullOptions, BundlePullReport,
    BundleRunOptions, BundleRunReport, MigrationBundleError,
};

pub fn migration_bundle_publish(
    options: &BundlePublishOptions,
) -> Result<BundlePublishReport, MigrationBundleError> {
    if options.oci_ref().trim().is_empty() {
        return Err(MigrationBundleError::InvalidInput(
            "oci_ref must not be empty".to_string(),
        ));
    }

    if !options.bundle_file().exists() {
        return Err(MigrationBundleError::InvalidInput(format!(
            "bundle file does not exist: {}",
            options.bundle_file().display()
        )));
    }

    let bytes = std::fs::read(options.bundle_file())?;
    let package = decode_package(&bytes)?;
    validate_bundle_package(&package)?;

    let digest = sha256_digest(&bytes);
    if let Some(ref_digest) = local_store::digest_from_ref(options.oci_ref())? {
        if ref_digest != digest {
            return Err(MigrationBundleError::Validation(format!(
                "oci_ref digest mismatch: ref={}, actual={}",
                ref_digest, digest
            )));
        }
    }

    if options.local_store_dir().is_none() && remote_registry::is_remote_ref(options.oci_ref()) {
        return remote_registry::remote_publish(options, &bytes);
    }

    local_store::publish_local_bundle(options, &bytes, digest)
}

pub fn migration_bundle_pull(
    options: &BundlePullOptions,
) -> Result<BundlePullReport, MigrationBundleError> {
    if options.oci_ref().trim().is_empty() {
        return Err(MigrationBundleError::InvalidInput(
            "oci_ref must not be empty".to_string(),
        ));
    }

    std::fs::create_dir_all(options.output_dir())?;

    if options.local_store_dir().is_none() && remote_registry::is_remote_ref(options.oci_ref()) {
        return remote_registry::remote_pull(options);
    }

    local_store::pull_local_bundle(options)
}

pub fn migration_run(options: &BundleRunOptions) -> Result<BundleRunReport, MigrationBundleError> {
    let bundle_ref = options.bundle_ref();
    let requested_digest = bundle_ref.digest().to_string();

    let pull = migration_bundle_pull(
        &BundlePullOptions::new(bundle_ref.to_string(), options.output_dir().clone())
            .with_optional_local_store_dir(options.local_store_dir().cloned()),
    )?;

    if pull.digest != requested_digest {
        return Err(MigrationBundleError::Validation(format!(
            "pulled digest mismatch for run: requested {}, resolved {}",
            requested_digest, pull.digest
        )));
    }

    Ok(BundleRunReport {
        bundle_ref: bundle_ref.to_string(),
        output_file: pull.output_file,
        digest: pull.digest,
        run_id: underlay_core::Uuid::new_v7(),
        status: "prepared".to_string(),
    })
}
