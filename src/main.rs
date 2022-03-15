use clap::StructOpt;
use utd::{are_you_on_unix, setup_logger};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn main() -> Result<()> {
    let args = utd::args::Cli::parse();
    // don't drop guard
    let _guard = setup_logger(args.log.unwrap_or(utd::args::LogLevel::Trace));

    // Adding a new note/task
    if args.note.is_some() || args.add.is_some() {
        new_entry(&args)?;
    }
    Ok(())
}

fn new_entry(args: &utd::args::Cli) -> Result<()> {
    let mut path = are_you_on_unix();
    path.push(".utd.json");
    let file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(path)?;

    Ok(())
}
