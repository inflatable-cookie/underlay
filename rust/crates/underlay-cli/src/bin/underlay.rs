use underlay_cli::{parse_command, print_usage, Command};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let cmd = match parse_command(args) {
        Ok(cmd) => cmd,
        Err(err) => {
            eprintln!("{err}");
            print_usage();
            std::process::exit(2);
        }
    };

    match cmd {
        Command::Help => {
            print_usage();
        }
        Command::Pulse(args) => match underlay_cli::runner::run_pulse(args) {
            Ok(output) => println!("{output}"),
            Err(err) => {
                eprintln!("pulse failed: {err}");
                std::process::exit(1);
            }
        },
    }
}
