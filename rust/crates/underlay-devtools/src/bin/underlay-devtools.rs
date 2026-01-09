use std::path::PathBuf;

fn print_usage() {
    eprintln!(
        "underlay-devtools\n\nUSAGE:\n  underlay-devtools sync-migrations --target <DIR> [--dry-run]\n\nCOMMANDS:\n  sync-migrations   Copy Underlay-owned SQL migrations into an app's migrations directory\n\nOPTIONS:\n  --target <DIR>    Target migrations directory (must exist)\n  --dry-run         Print what would be written, without writing\n"
    );
}

fn main() {
    let mut args = std::env::args().skip(1);

    let Some(cmd) = args.next() else {
        print_usage();
        std::process::exit(2);
    };

    if cmd == "--help" || cmd == "-h" {
        print_usage();
        return;
    }

    match cmd.as_str() {
        "sync-migrations" => {
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
        other => {
            eprintln!("unknown command: {other}");
            print_usage();
            std::process::exit(2);
        }
    }
}
