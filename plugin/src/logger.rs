//! # Logging Module
//!
//! This module provides logging functionality for the X-Plane UDP Bridge plugin.
//! It initializes a file-based logger with custom formatting and thread information.

use crate::XPlaneUdpBridgePlugin;
use chrono::Local;
use std::fs::OpenOptions;
use std::sync::Once;
use tracing::{Level, info};
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::time::FormatTime;
use tracing_subscriber::fmt::writer::BoxMakeWriter;

/// A custom time formatter for the logging system.
///
/// This struct implements the `FormatTime` trait to provide a custom timestamp
/// format for log entries. It formats the time as "YYYY-MM-DD HH:MM:SS.sss".
struct LocalTime;

impl FormatTime for LocalTime {
    /// Formats the current local time according to the specified format.
    ///
    /// # Arguments
    ///
    /// * `writer` - A mutable writer to write the formatted time to
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if formatting was successful, or an error if formatting failed.
    fn format_time(&self, writer: &mut Writer<'_>) -> std::fmt::Result {
        write!(writer, "{}", Local::now().format("%Y-%m-%d %H:%M:%S%.3f"))
    }
}

/// A static synchronization primitive to ensure the logger is initialized only once.
///
/// This uses the `Once` pattern from the standard library to guarantee that
/// the initialization code runs exactly once, even if `init()` is called multiple times.
static LOGGER_INITIALIZED: Once = Once::new();

/// Initializes the global logger for the plugin.
///
/// This function sets up a file-based logger with custom formatting. The logger will:
/// - Write to a file named after the plugin (e.g., "XPlaneUdpBridge.log")
/// - Include timestamps in "YYYY-MM-DD HH:MM:SS.sss" format
/// - Include target, thread IDs, thread names, and line numbers
/// - Log at INFO level and above
///
/// This function is thread-safe and will only initialize the logger once,
/// even if called from multiple threads.
///
/// # Panics
///
/// This function will panic if it cannot create or open the log file.
pub(crate) fn init_file_logger() {
    LOGGER_INITIALIZED.call_once(|| {
        let filename = XPlaneUdpBridgePlugin::NAME.to_string() + ".log";
        let file = OpenOptions::new().create(true).append(true).open(filename).unwrap();
        let writer = BoxMakeWriter::new(file);
        tracing_subscriber::fmt()
            .with_writer(writer)
            .with_timer(LocalTime)
            .with_ansi(false)
            .with_target(true)
            .with_thread_ids(true)
            .with_thread_names(true)
            .with_line_number(true)
            .with_max_level(Level::INFO)
            .init();
        info!("logger initialized");
    });
}

#[cfg(test)]
mod tests {
    use crate::{XPlaneUdpBridgePlugin, logger};
    use std::fs;
    use tracing::info;

    /// Tests that the logger initialization creates a log file and writes to it.
    ///
    /// This test verifies:
    /// 1. A log file is created when the logger is initialized
    /// 2. The log file contains the expected content
    /// 3. Multiple calls to init() don't cause duplicate initialization
    /// 4. Log messages are properly appended to the file
    #[test]
    fn test_logger_init_create_log_file_and_write() {
        let dir = std::env::current_dir().unwrap();
        let filename = XPlaneUdpBridgePlugin::NAME.to_string() + ".log";
        let log_file_path = dir.join(filename);

        logger::init_file_logger();
        println!("test log file path: {:?}", log_file_path);
        assert!(log_file_path.exists(), "test failed: log file not created");

        let content = fs::read_to_string(log_file_path.as_path()).unwrap();
        assert!(
            content.contains("logger initialized"),
            "test failed: log file content should contain 'logger initialized'"
        );

        // test log file only init once
        info!("test log file content append line 1");
        info!("test log file content append line 2");
        logger::init_file_logger();
        let content = fs::read_to_string(log_file_path.as_path()).unwrap();
        assert!(
            content.contains("test log file content append line 1"),
            "test failed: log file content should contain 'test log file content append line 1'"
        );
        assert!(
            content.contains("test log file content append line 2"),
            "test failed: log file content should contain 'test log file content append line 2'"
        );

        fs::remove_file(log_file_path.as_path()).unwrap();
    }
}
