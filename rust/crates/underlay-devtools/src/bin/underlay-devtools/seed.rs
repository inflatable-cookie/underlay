use std::path::PathBuf;

use crate::cli::print_usage;

pub(crate) fn run(mut args: impl Iterator<Item = String>) {
    let Some(subcommand) = args.next() else {
        eprintln!("missing seed subcommand");
        print_usage();
        std::process::exit(2);
    };

    if subcommand != "bundle" {
        eprintln!("unknown seed subcommand: {subcommand}");
        print_usage();
        std::process::exit(2);
    }

    let Some(action) = args.next() else {
        eprintln!("missing seed bundle action");
        print_usage();
        std::process::exit(2);
    };

    match action.as_str() {
        "build" => build(args),
        "publish" => publish(args),
        "pull" => pull(args),
        other => {
            eprintln!("unknown seed bundle action: {other}");
            print_usage();
            std::process::exit(2);
        }
    }
}

fn build(mut args: impl Iterator<Item = String>) {
    let mut source: Option<PathBuf> = None;
    let mut output: Option<PathBuf> = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--source" => {
                let Some(v) = args.next() else {
                    eprintln!("--source requires a value");
                    std::process::exit(2);
                };
                source = Some(PathBuf::from(v));
            }
            "--output" => {
                let Some(v) = args.next() else {
                    eprintln!("--output requires a value");
                    std::process::exit(2);
                };
                output = Some(PathBuf::from(v));
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

    let Some(source_dir) = source else {
        eprintln!("missing --source");
        std::process::exit(2);
    };
    let Some(output_file) = output else {
        eprintln!("missing --output");
        std::process::exit(2);
    };

    match underlay_devtools::seed_bundle_build(&underlay_devtools::SeedBundleBuildOptions {
        source_dir,
        output_file,
    }) {
        Ok(report) => {
            println!(
                "seed bundle written {} (name={}, artifact_type={}, layers={}, sql_files={}, sql_bytes={})",
                report.output_file.display(),
                report.bundle_name,
                report.artifact_type,
                report.layer_count,
                report.sql_file_count,
                report.total_sql_bytes,
            );
            println!("seed bundle digest {}", report.bundle_digest);
        }
        Err(err) => {
            eprintln!("seed bundle build failed: {err}");
            std::process::exit(1);
        }
    }
}

fn publish(mut args: impl Iterator<Item = String>) {
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

    match underlay_devtools::seed_bundle_publish(&underlay_devtools::BundlePublishOptions {
        bundle_file,
        oci_ref,
        local_store_dir: None,
    }) {
        Ok(report) => {
            println!(
                "seed publish {} -> {} ({}, digest={})",
                report.bundle_file.display(),
                report.oci_ref,
                report.status,
                report.digest
            );
        }
        Err(err) => {
            eprintln!("seed bundle publish failed: {err}");
            std::process::exit(1);
        }
    }
}

fn pull(mut args: impl Iterator<Item = String>) {
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

    match underlay_devtools::seed_bundle_pull(&underlay_devtools::SeedBundlePullOptions {
        oci_ref,
        output_dir,
        local_store_dir: None,
    }) {
        Ok(report) => {
            println!(
                "seed pull {} -> {} ({}, digest={}, sql_files={})",
                report.oci_ref,
                report.output_dir.display(),
                report.status,
                report.digest,
                report.sql_file_count,
            );
        }
        Err(err) => {
            eprintln!("seed bundle pull failed: {err}");
            std::process::exit(1);
        }
    }
}
