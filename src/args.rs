use clap::{ArgEnum, Parser};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Cli {
    /// Add note(s)
    #[clap(short, long, multiple_values = true)]
    pub note: Option<Vec<String>>,

    /// Add a new task(s)
    #[clap(short, long, multiple_values = true)]
    pub add: Option<Vec<String>>,

    /// Check/uncheck task(s) as complete
    #[clap(short, long, multiple_values = true)]
    pub check: Option<Vec<String>>,

    /// Start/stop task(s)
    #[clap(short, long, multiple_values = true)]
    pub begin: Option<Vec<String>>,

    /// Sort task ids
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

    /// Set log level
    #[clap(short, long)]
    pub order_ids: bool,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
pub enum PriorityLevel {
    Low,
    Normal,
    High,
}

impl ToString for PriorityLevel {
    fn to_string(&self) -> String {
        match &self {
            PriorityLevel::Low => "low",
            PriorityLevel::Normal => "normal",
            PriorityLevel::High => "high",
        }
        .to_owned()
    }
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
