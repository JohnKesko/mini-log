//! mini_log is a small environment-controlled logging crate.
//!
//! Logging is disabled by default.
//!
//! To enable logs, set the `LOG_LEVEL` environment variable.
//!
//! Supported values:
//!
//! - `DEBUG`
//! - `INFO`
//! - `WARNING` or `WARN`
//! - `ERROR`
//! - `OFF`
//!
//! Example:
//!
//! ```bash
//! LOG_LEVEL=WARNING cargo run
//! ```
//!
//! Example usage:
//!
//! ```rust
//! use mini_log::*;
//!
//! fn main() {
//!     LogMessage::new(Level::Warning, "My Message here");
//! }
//! ```
//!
//! If `LOG_LEVEL` is missing, invalid, or set to `OFF`, nothing is printed.
//!
use chrono::Local;
use std::env;
use std::fmt;
use std::sync::Once;

static LOG_LEVEL_WARNING: Once = Once::new();

/// The severity level of a log message.
///
/// Log levels are ordered like this:
///
/// `Debug < Info < Warning < Error`
///
/// The active `LOG_LEVEL` environment variable controls which messages are printed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    /// Prints only when `LOG_LEVEL=DEBUG`.
    Debug,

    /// Prints when `LOG_LEVEL=DEBUG` or `LOG_LEVEL=INFO`.
    Info,

    /// Prints when `LOG_LEVEL=DEBUG`, `LOG_LEVEL=INFO`, or `LOG_LEVEL=WARNING`.
    Warning,

    /// Prints when `LOG_LEVEL=DEBUG`, `LOG_LEVEL=INFO`, `LOG_LEVEL=WARNING`, or `LOG_LEVEL=ERROR`.
    Error,
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Debug => write!(f, "DEBUG"),
            Self::Info => write!(f, "INFO"),
            Self::Warning => write!(f, "WARNING"),
            Self::Error => write!(f, "ERROR"),
        }
    }
}

/// A single log message.
///
/// Creating a `LogMessage` with [`LogMessage::new`] prints it immediately,
/// but only if the `LOG_LEVEL` environment variable allows it.
///
/// If `LOG_LEVEL` is missing, invalid, or set to `OFF`, nothing is printed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogMessage {
    timestamp: String,
    level: Level,
    message: String,
}

impl LogMessage {
    /// Creates a new log message.
    ///
    /// This prints immediately if the configured `LOG_LEVEL` allows it.
    ///
    /// If `LOG_LEVEL` is not set, a setup warning is printed once.
    ///
    /// Example:
    ///
    /// ```rust
    /// use mini_log::*;
    ///
    /// fn main() {
    ///     LogMessage::new(Level::Warning, "My Message here");
    /// }
    /// ```
    ///
    /// Run with:
    ///
    /// ```bash
    /// LOG_LEVEL=WARNING cargo run
    /// ```
    pub fn new(level: Level, message: impl Into<String>) -> Self {
        warn_if_log_level_missing();

        let log_message = Self {
            timestamp: current_timestamp(),
            level,
            message: message.into(),
        };

        if should_print(level) {
            println!("{log_message}");
        }

        log_message
    }

    pub fn timestamp(&self) -> &str {
        &self.timestamp
    }

    pub fn level(&self) -> Level {
        self.level
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

/// Returns `true` if the provided level would currently be printed.
///
/// This uses the current value of the `LOG_LEVEL` environment variable.
pub fn is_enabled(level: Level) -> bool {
    should_print(level)
}

/// Returns the current timestamp using the configured `LOG_TIME_FORMAT`.
pub fn get_timestamp() -> String {
    current_timestamp()
}

impl fmt::Display for LogMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] - {} - {}",
            self.timestamp, self.level, self.message
        )
    }
}

fn current_timestamp() -> String {
    match env::var("LOG_TIME_FORMAT") {
        Ok(value) => match value.trim().to_uppercase().as_str() {
            "RFC3339" => timestamp_rfc3339(),
            "TIME_ONLY" => timestamp_time_only(),
            "DATE_ONLY" => timestamp_date_only(),
            "UNIX" => timestamp_unix(),
            "DEFAULT" => timestamp_default(),
            _ => timestamp_default(),
        },
        Err(_) => timestamp_default(),
    }
}

fn timestamp_default() -> String {
    let now = Local::now();

    let date_time = now.format("%Y-%m-%d - %H:%M:%S");
    let centiseconds = now.timestamp_subsec_millis() / 10;

    format!("{}:{:02}", date_time, centiseconds)
}

fn timestamp_rfc3339() -> String {
    Local::now().to_rfc3339()
}

fn timestamp_time_only() -> String {
    Local::now().format("%H:%M:%S").to_string()
}

fn timestamp_date_only() -> String {
    Local::now().format("%Y-%m-%d").to_string()
}

fn timestamp_unix() -> String {
    Local::now().timestamp().to_string()
}

fn should_print(level: Level) -> bool {
    match env::var("LOG_LEVEL") {
        Ok(value) => match value.trim().to_uppercase().as_str() {
            "DEBUG" => level >= Level::Debug,
            "INFO" => level >= Level::Info,
            "WARNING" | "WARN" => level >= Level::Warning,
            "ERROR" => level >= Level::Error,
            "OFF" => false,
            _ => false,
        },
        Err(_) => false,
    }
}

fn warn_if_log_level_missing() {
    if env::var("LOG_LEVEL").is_err() {
        LOG_LEVEL_WARNING.call_once(|| {
            eprintln!(
                r#"
                    mini_log: LOG_LEVEL is not set, no logs will be printed.
                    Set one of these environment variable:
                        LOG_LEVEL=DEBUG
                        LOG_LEVEL=INFO
                        LOG_LEVEL=WARNING
                        LOG_LEVEL=ERROR
                        LOG_LEVEL=OFF
                "#
            );
        });
    }
}
