use log::{LevelFilter, Log, Metadata, Record};
use std::sync::Once;

#[macro_export]
macro_rules! log_world {
    ($level:expr, $($arg:tt)+) => {
        log::log!(target: "world", $level, $($arg)+)
    };
}

#[macro_export]
macro_rules! log_chunk {
    ($level:expr, $($arg:tt)+) => {
        log::log!(target: "chunk", $level, $($arg)+)
    };
}

#[macro_export]
macro_rules! log_render {
    ($level:expr, $($arg:tt)+) => {
        log::log!(target: "render", $level, $($arg)+)
    };
}

#[macro_export]
macro_rules! log_entity {
    ($level:expr, $($arg:tt)+) => {
        log::log!(target: "entity", $level, $($arg)+)
    };
}


static INIT: Once = Once::new();

pub struct GameLogger {
    world_level: LevelFilter,
    chunk_level: LevelFilter,
    render_level: LevelFilter,
    entity_level: LevelFilter,
}

impl GameLogger {
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
