pub mod args;
mod data;
pub use data::*;
use std::path::PathBuf;
mod config;
pub use config::*;

pub fn setup_logger(log_level: args::LogLevel) -> tracing_appender::non_blocking::WorkerGuard {
    let file_appender = tracing_appender::rolling::daily(data_dir(), "utd-log");
    let (file_writer, guard) = tracing_appender::non_blocking(file_appender);
    tracing::subscriber::set_global_default(
        tracing_subscriber::fmt::Subscriber::builder()
            .with_max_level(match log_level {
                args::LogLevel::Trace => tracing::Level::TRACE,
                args::LogLevel::Debug => tracing::Level::DEBUG,
                args::LogLevel::Info => tracing::Level::INFO,
                args::LogLevel::Warn => tracing::Level::WARN,
                args::LogLevel::Error => tracing::Level::ERROR,
            })
            .with_writer(file_writer)
            .with_ansi(false)
            //.pretty()
            .finish(),
    )
    .expect("Unable to set global tracing subscriber");
    tracing::info!(
        "{} {} has started",
        clap::crate_name!(),
        clap::crate_version!()
    );
    guard
}

pub fn data_dir() -> PathBuf {
    use directories::ProjectDirs;
    let dirs = ProjectDirs::from("org", "Ugly Todo", "utd").unwrap();
    dirs.data_local_dir().to_path_buf()
}
