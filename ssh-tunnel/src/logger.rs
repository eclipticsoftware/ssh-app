use std::fs;
use std::io;
use std::path::Path;

use log::LevelFilter;
use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
};

/// Configures the logger
///
/// The logger will print to both stderr (for informational and error statements) and a log file
/// at the given path (for all statements, including debug and trace statements). A new log file
/// will be generated for each invocation, and the previous five logs will be cycled.
pub fn configure_logger(path: &str) -> io::Result<()> {
    let level = log::LevelFilter::Info;

    // Build a stderr logger.
    let stderr = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{m}{n}")))
        .build();

    cycle_logs(path)?;

    // Logging to log file.
    let logfile = FileAppender::builder().build(path).map_err(|err| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to create log file: {err}"),
        )
    })?;

    // Log Trace level output to file where trace is the default level
    // and the programmatically specified level to stderr.
    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(level)))
                .build("stderr", Box::new(stderr)),
        )
        .build(
            Root::builder()
                .appender("logfile")
                .appender("stderr")
                .build(LevelFilter::Trace),
        )
        .map_err(|err| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("Failed to build log config: {err}"),
            )
        })?;

    let _handle = log4rs::init_config(config).map_err(|err| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to create log handler: {err}"),
        )
    })?;

    Ok(())
}

/// Cycles previous log files
///
/// The cycling is done such that the last log file is renamed <path>.0.<ext>, the next youngest file is renamed
/// <path>.1.<ext>, etc... up to five logs.
fn cycle_logs(path: &str) -> io::Result<()> {
    let path = Path::new(path);
    let dir = path.parent().expect("Bad log path: no parent dir");
    let stem = path
        .file_stem()
        .expect("Bad log path: no stem")
        .to_string_lossy();

    let ext = path
        .extension()
        .expect("Bad log path: no extension")
        .to_string_lossy();

    for ver in (0..5).rev() {
        let old = dir.join(&format!("{stem}.{ver}.{ext}"));
        let new = dir.join(&format!("{stem}.{}.{ext}", ver + 1));

        if fs::metadata(&old).is_ok() {
            fs::rename(old, new)?;
        }
    }

    if fs::metadata(path).is_ok() {
        let new = dir.join(format!("{stem}.0.{ext}"));
        fs::rename(path, new)?;
    }

    Ok(())
}
