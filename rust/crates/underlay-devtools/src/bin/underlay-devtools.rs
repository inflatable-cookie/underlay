#[path = "underlay-devtools/cli.rs"]
mod cli;
#[path = "underlay-devtools/migration.rs"]
mod migration;
#[path = "underlay-devtools/seed.rs"]
mod seed;
#[path = "underlay-devtools/sync_migrations.rs"]
mod sync_migrations;

fn main() {
    let mut args = std::env::args().skip(1);

    let Some(cmd) = args.next() else {
        cli::print_usage();
        std::process::exit(2);
    };

    if cmd == "--help" || cmd == "-h" {
        cli::print_usage();
        return;
    }

    match cmd.as_str() {
        "sync-migrations" => sync_migrations::run(args),
        "migration" => migration::run(args),
        "seed" => seed::run(args),
        other => {
            eprintln!("unknown command: {other}");
            cli::print_usage();
            std::process::exit(2);
        }
    }
}
