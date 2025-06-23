use log::{LevelFilter, Log, Metadata, Record};
use std::sync::Once;

/// Macro for logging messages with the "world" target.
/// 
/// Usage: `log_world!(log::Level::Info, "message")`
#[macro_export]
macro_rules! log_world {
    ($level:expr, $($arg:tt)+) => {
        log::log!(target: "world", $level, $($arg)+)
    };
}

/// Macro for logging messages with the "chunk" target.
/// 
/// Usage: `log_chunk!(log::Level::Debug, "message")`
#[macro_export]
macro_rules! log_chunk {
    ($level:expr, $($arg:tt)+) => {
        log::log!(target: "chunk", $level, $($arg)+)
    };
}

/// Macro for logging messages with the "render" target.
/// 
/// Usage: `log_render!(log::Level::Trace, "message")`
#[macro_export]
macro_rules! log_render {
    ($level:expr, $($arg:tt)+) => {
        log::log!(target: "render", $level, $($arg)+)
    };
}

/// Macro for logging messages with the "entity" target.
/// 
/// Usage: `log_entity!(log::Level::Warn, "message")`
#[macro_export]
macro_rules! log_entity {
    ($level:expr, $($arg:tt)+) => {
        log::log!(target: "entity", $level, $($arg)+)
    };
}

/// Logger implementation for the game, supporting different log levels for different targets.
pub struct GameLogger {
    world_level: LevelFilter,
    chunk_level: LevelFilter,
    render_level: LevelFilter,
    entity_level: LevelFilter,
}

impl GameLogger {
    /// Initializes the global logger instance.
    /// This should be called once at the start of the program.
    pub fn init() {
        INIT.call_once(|| {
            let logger = GameLogger {
                world_level: LevelFilter::Info,
                chunk_level: LevelFilter::Info,
                render_level: LevelFilter::Info,
                entity_level: LevelFilter::Info,
            };
            log::set_boxed_logger(Box::new(logger))
                .map(|()| log::set_max_level(LevelFilter::Trace))
                .expect("Failed to set logger");
        });
    }

    /// Determines if a log message should be logged based on the target and level.
    ///
    /// - `target`: The log target string.
    /// - `level`: The log level.
    ///
    /// Returns `true` if the message should be logged, `false` otherwise.
    fn should_log(&self, target: &str, level: log::Level) -> bool {
        let filter = match target {
            "world" => self.world_level,
            "chunk" => self.chunk_level,
            "render" => self.render_level,
            "entity" => self.entity_level,
            _ => LevelFilter::Info,
        };
        level <= filter
    }
}

impl Log for GameLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        let target = metadata.target();
        self.should_log(target, metadata.level())
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let color = match record.level() {
                log::Level::Error => "\x1b[31m",
                log::Level::Warn => "\x1b[33m",  
                log::Level::Info => "\x1b[32m",  
                log::Level::Debug => "\x1b[36m",
                log::Level::Trace => "\x1b[90m", 
            };
            println!(
                "{}[{:5}][{}] {}\x1b[0m",
                color,
                record.level(),
                record.target(),
                record.args()
            );
        }
    }

    fn flush(&self) {}
}

static INIT: Once = Once::new();