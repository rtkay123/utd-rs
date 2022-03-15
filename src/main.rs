use clap::StructOpt;
use utd::setup_logger;

fn main() {
    let args = utd::args::Cli::parse();
    let _guard = setup_logger(args.log.unwrap_or(utd::args::LogLevel::Trace));
}
