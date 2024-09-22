use clap::Parser;
use wcr_cli::WcrCli;

mod wcr_cli;

fn main() {
    let cli = WcrCli::parse();
    if let Err(e) = cli.run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
