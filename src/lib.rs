pub mod args;
mod data;
pub use data::*;
use std::path::PathBuf;

pub fn setup_logger(log_level: args::LogLevel) -> tracing_appender::non_blocking::WorkerGuard {
    let file_appender = tracing_appender::rolling::daily(are_you_on_unix(), "utd-log");
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
            .pretty()
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

// This function only gets compiled if the target family is unix
#[cfg(target_family = "unix")]
pub fn are_you_on_unix() -> PathBuf {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("utd").unwrap();
    xdg_dirs.get_state_home()
}

// And this function only gets compiled if the target family is *not* unix
#[cfg(not(target_family = "unix"))]
pub fn are_you_on_unix() -> &'static str {
    "where does windows store logs even?"
}
