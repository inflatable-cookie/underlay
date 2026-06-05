use std::path::PathBuf;

use crate::cli::print_usage;

pub(crate) fn build(mut args: impl Iterator<Item = String>) {
    let mut output: Option<PathBuf> = None;
    let mut source_system: Option<String> = None;
    let mut target_schema_version: Option<String> = None;
    let mut media_dir: Option<PathBuf> = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--output" => {
                let Some(v) = args.next() else {
                    eprintln!("--output requires a value");
                    std::process::exit(2);
                };
                output = Some(PathBuf::from(v));
            }
            "--source-system" => {
                let Some(v) = args.next() else {
                    eprintln!("--source-system requires a value");
                    std::process::exit(2);
                };
                source_system = Some(v);
            }
            "--target-schema-version" => {
                let Some(v) = args.next() else {
                    eprintln!("--target-schema-version requires a value");
                    std::process::exit(2);
                };
                target_schema_version = Some(v);
            }
            "--media-dir" => {
                let Some(v) = args.next() else {
                    eprintln!("--media-dir requires a value");
                    std::process::exit(2);
                };
                media_dir = Some(PathBuf::from(v));
            }
            "--help" | "-h" => {
                print_usage();
                return;
            }
            other => {
                eprintln!("unknown argument: {other}");
                print_usage();
                std::process::exit(2);
            }
        }
    }

    let Some(output) = output else {
        eprintln!("missing --output");
        std::process::exit(2);
    };
    let Some(source_system) = source_system else {
        eprintln!("missing --source-system");
        std::process::exit(2);
    };
    let Some(target_schema_version) = target_schema_version else {
        eprintln!("missing --target-schema-version");
        std::process::exit(2);
    };

    match underlay_devtools::migration_bundle_build(&underlay_devtools::BundleBuildOptions {
        output_file: output,
        source_system,
        target_schema_version,
        media_dir,
        media_shard_max_bytes: None,
    }) {
        Ok(report) => {
            println!(
                "bundle written {} (artifact_type={}, layers={}, sidecars={})",
                report.output_file.display(),
                report.artifact_type,
                report.layer_count,
                report.sidecar_count
            );
            println!("bundle digest {}", report.bundle_digest);
            println!(
                "media assets {}, media shards {}",
                report.media_asset_count, report.media_shard_count
            );
        }
        Err(err) => {
            eprintln!("migration bundle build failed: {err}");
            std::process::exit(1);
        }
    }
}

pub(crate) fn publish(mut args: impl Iterator<Item = String>) {
    let mut bundle: Option<PathBuf> = None;
    let mut oci_ref: Option<String> = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--bundle" => {
                let Some(v) = args.next() else {
                    eprintln!("--bundle requires a value");
                    std::process::exit(2);
                };
                bundle = Some(PathBuf::from(v));
            }
            "--oci-ref" => {
                let Some(v) = args.next() else {
                    eprintln!("--oci-ref requires a value");
                    std::process::exit(2);
                };
                oci_ref = Some(v);
            }
            "--help" | "-h" => {
                print_usage();
                return;
            }
            other => {
                eprintln!("unknown argument: {other}");
                print_usage();
                std::process::exit(2);
            }
        }
    }

    let Some(bundle_file) = bundle else {
        eprintln!("missing --bundle");
        std::process::exit(2);
    };
    let Some(oci_ref) = oci_ref else {
        eprintln!("missing --oci-ref");
        std::process::exit(2);
    };

    match underlay_devtools::migration_bundle_publish(&underlay_devtools::BundlePublishOptions {
        bundle_file,
        oci_ref,
        local_store_dir: None,
    }) {
        Ok(report) => {
            println!(
                "publish {} -> {} ({}, digest={})",
                report.bundle_file.display(),
                report.oci_ref,
                report.status,
                report.digest
            );
        }
        Err(err) => {
            eprintln!("migration bundle publish failed: {err}");
            std::process::exit(1);
        }
    }
}

pub(crate) fn pull(mut args: impl Iterator<Item = String>) {
    let mut output: Option<PathBuf> = None;
    let mut oci_ref: Option<String> = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--output" => {
                let Some(v) = args.next() else {
                    eprintln!("--output requires a value");
                    std::process::exit(2);
                };
                output = Some(PathBuf::from(v));
            }
            "--oci-ref" => {
                let Some(v) = args.next() else {
                    eprintln!("--oci-ref requires a value");
                    std::process::exit(2);
                };
                oci_ref = Some(v);
            }
            "--help" | "-h" => {
                print_usage();
                return;
            }
            other => {
                eprintln!("unknown argument: {other}");
                print_usage();
                std::process::exit(2);
            }
        }
    }

    let Some(output_dir) = output else {
        eprintln!("missing --output");
        std::process::exit(2);
    };
    let Some(oci_ref) = oci_ref else {
        eprintln!("missing --oci-ref");
        std::process::exit(2);
    };

    match underlay_devtools::migration_bundle_pull(&underlay_devtools::BundlePullOptions {
        oci_ref,
        output_dir,
        local_store_dir: None,
    }) {
        Ok(report) => {
            println!(
                "pull {} -> {} ({}, digest={})",
                report.oci_ref,
                report.output_file.display(),
                report.status,
                report.digest
            );
        }
        Err(err) => {
            eprintln!("migration bundle pull failed: {err}");
            std::process::exit(1);
        }
    }
}
