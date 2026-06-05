use std::path::PathBuf;

use crate::cli::print_usage;

pub(crate) fn run(mut args: impl Iterator<Item = String>) {
    let mut target: Option<PathBuf> = None;
    let mut dry_run = false;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--target" => {
                let Some(v) = args.next() else {
                    eprintln!("--target requires a value");
                    std::process::exit(2);
                };
                target = Some(PathBuf::from(v));
            }
            "--dry-run" => {
                dry_run = true;
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

    let Some(target) = target else {
        eprintln!("missing --target");
        print_usage();
        std::process::exit(2);
    };

    match underlay_devtools::sync_migrations(&target, dry_run) {
        Ok(report) => {
            for path in &report.written {
                if dry_run {
                    println!("would write {}", path.display());
                } else {
                    println!("wrote {}", path.display());
                }
            }
            for path in &report.skipped {
                println!("skipped {}", path.display());
            }
        }
        Err(err) => {
            eprintln!("sync-migrations failed: {err}");
            std::process::exit(1);
        }
    }
}
