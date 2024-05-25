use log::{Level, Log};

#[derive(Debug, Default)]
struct Logging;

impl Log for Logging {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            serial1_print!("[");
            match record.level() {
                Level::Error => serial1_print!("\x1b[1;31mERROR\x1b[0m"),
                Level::Warn => serial1_print!("\x1b[1;33mWARN\x1b[0m"),
                Level::Info => serial1_print!("\x1b[1;34mINFO\x1b[0m"),
                Level::Debug => serial1_print!("\x1b[1;32mDEBUG\x1b[0m"),
                Level::Trace => serial1_print!("\x1b[1;37mTRACE\x1b[0m"),
            }
            serial1_print!(
                "] {} {}: {}\n",
                record.target(),
                record.file().unwrap_or("<unknown>"),
                record.args()
            );
        }
    }

    fn flush(&self) {}
}

pub fn init() {
    log::set_logger(&Logging).unwrap();
    log::set_max_level(Level::Trace.to_level_filter());
}
