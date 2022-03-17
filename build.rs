//use clap::{arg, Command};

// Run this example as `cargo run --example man | man -l -`.

use clap::{ArgEnum, Args, Command, Parser};
#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Cli {
    /// Add note(s)
    #[clap(short, long, multiple_values = true)]
    pub note: Option<Vec<String>>,

    /// Add a new task(s)
    #[clap(short, long, multiple_values = true)]
    pub add: Option<Vec<String>>,

    /// Add a new task(s)
    #[clap(short, long, multiple_values = true)]
    pub delete: Option<Vec<String>>,

    /// Check/uncheck task(s) as complete
    #[clap(short, long, multiple_values = true)]
    pub check: Option<Vec<String>>,

    /// Start/stop task(s)
    #[clap(short, long, multiple_values = true)]
    pub begin: Option<Vec<String>>,

    /// Show tasks and sort
    #[clap(short, long, arg_enum)]
    pub sort: Option<SortParam>,

    /// Clear all completed notes/tasks
    #[clap(short, long)]
    pub tidy: bool,

    /// Set a priority level
    #[clap(short, long, arg_enum, multiple_values = true)]
    pub priority: Option<Vec<PriorityLevel>>,

    /// Set log level
    #[clap(short, long, arg_enum)]
    pub log: Option<LogLevel>,

    /// Make ids sequential
    #[clap(short, long, long = "reset-ids")]
    pub re_set_ids: bool,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
pub enum PriorityLevel {
    Low,
    Normal,
    High,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
pub enum SortParam {
    Age,
    Priority,
}
fn main() -> Result<(), std::io::Error> {
    let val = Cli::augment_args(Command::new("utd"));
    let man = clap_mangen::Man::new(val);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;
    std::fs::write("target/utd.1", buffer)?;
    Ok(())
}
