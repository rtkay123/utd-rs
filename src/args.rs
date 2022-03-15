use clap::{ArgEnum, Parser};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Cli {
    /// Add note(s)
    #[clap(short, long)]
    pub note: Option<Vec<String>>,

    /// Add a new task(s)
    #[clap(short, long)]
    pub add: Option<Vec<String>>,

    /// Check/uncheck task(s) as complete
    #[clap(short, long)]
    pub check: Option<Vec<String>>,

    /// Start/stop task(s)
    #[clap(short, long)]
    pub begin: Option<Vec<String>>,

    /// Sort all tasks and notes
    #[clap(short, long, arg_enum)]
    pub sort: Option<SortParam>,

    /// Clear all completed notes/tasks
    #[clap(short, long)]
    pub tidy: bool,

    /// Set a priority level
    #[clap(short, long, arg_enum)]
    pub priority: Option<PriorityLevel>,

    /// Set log level
    #[clap(short, long, arg_enum)]
    pub log: Option<LogLevel>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
pub enum PriorityLevel {
    Low,
    Medium,
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
    Time,
    Priority,
    Id,
}
