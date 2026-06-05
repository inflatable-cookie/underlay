#[path = "migration/bundle.rs"]
mod bundle;
#[path = "migration/reports.rs"]
mod reports;
#[path = "migration/run.rs"]
mod run;

use crate::cli::print_usage;

pub(crate) fn run(mut args: impl Iterator<Item = String>) {
    let Some(subcommand) = args.next() else {
        eprintln!("missing migration subcommand");
        print_usage();
        std::process::exit(2);
    };

    if subcommand == "run" {
        run::run(args);
        return;
    }

    if subcommand == "report" {
        reports::run(args);
        return;
    }

    if subcommand != "bundle" {
        eprintln!("unknown migration subcommand: {subcommand}");
        print_usage();
        std::process::exit(2);
    }

    let Some(action) = args.next() else {
        eprintln!("missing migration bundle action");
        print_usage();
        std::process::exit(2);
    };

    match action.as_str() {
        "build" => bundle::build(args),
        "publish" => bundle::publish(args),
        "pull" => bundle::pull(args),
        other => {
            eprintln!("unknown migration bundle action: {other}");
            print_usage();
            std::process::exit(2);
        }
    }
}
