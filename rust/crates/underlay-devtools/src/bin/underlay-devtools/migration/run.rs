use std::path::PathBuf;

use crate::cli::print_usage;

pub(crate) fn run(mut args: impl Iterator<Item = String>) {
    let mut output: Option<PathBuf> = None;
    let mut bundle: Option<String> = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--output" => {
                let Some(v) = args.next() else {
                    eprintln!("--output requires a value");
                    std::process::exit(2);
                };
                output = Some(PathBuf::from(v));
            }
            "--bundle" => {
                let Some(v) = args.next() else {
                    eprintln!("--bundle requires a value");
                    std::process::exit(2);
                };
                bundle = Some(v);
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
    let Some(bundle_ref) = bundle else {
        eprintln!("missing --bundle");
        std::process::exit(2);
    };

    match underlay_devtools::migration_run(&underlay_devtools::BundleRunOptions {
        bundle_ref,
        output_dir,
        local_store_dir: None,
    }) {
        Ok(report) => {
            println!(
                "run {} -> {} ({}, digest={}, run_id={})",
                report.bundle_ref,
                report.output_file.display(),
                report.status,
                report.digest,
                report.run_id
            );
        }
        Err(err) => {
            eprintln!("migration run failed: {err}");
            std::process::exit(1);
        }
    }
}
