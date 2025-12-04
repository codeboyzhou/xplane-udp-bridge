use chrono::Local;
use std::fs::OpenOptions;
use std::sync::Once;
use tracing::{Level, info};
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::time::FormatTime;
use tracing_subscriber::fmt::writer::BoxMakeWriter;

struct LocalTime;

impl FormatTime for LocalTime {
    fn format_time(&self, writer: &mut Writer<'_>) -> std::fmt::Result {
        write!(writer, "{}", Local::now().format("%Y-%m-%d %H:%M:%S%.3f"))
    }
}

static LOGGER_INITIALIZED: Once = Once::new();

pub fn init_file_logger(filename: &str) {
    LOGGER_INITIALIZED.call_once(|| {
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
    use crate::logger;
    use std::fs;
    use tracing::info;

    #[test]
    fn test_logger_init_create_log_file_and_write() {
        let dir = std::env::current_dir().unwrap();
        let filename = "test.log";
        let log_file_path = dir.join(filename);

        logger::init_file_logger(filename);
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
        logger::init_file_logger(filename);
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
