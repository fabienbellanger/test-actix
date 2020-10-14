//! Logger module for customize logs

use chrono::Local;
use env_logger::fmt::Color;
use env_logger::Builder;
use log::LevelFilter;
use log::{info, Level};
use std::io::Write;

/// Initialize logger
pub fn init(level: String) {
    let level = match &*level {
        "trace" => LevelFilter::Trace,
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "Error" => LevelFilter::Error,
        &_ => LevelFilter::Error,
    };

    Builder::new()
        .format(move |buf, record| {
            let mut level_style = buf.style();

            let (color, level_spaces) = match record.level() {
                Level::Trace => (Color::White, " "),
                Level::Debug => (Color::Green, " "),
                Level::Info => (Color::Blue, "  "),
                Level::Warn => (Color::Yellow, "  "),
                Level::Error => (Color::Red, " "),
            };

            level_style.set_color(color.clone()).set_bold(true);
            let line = match record.line() {
                Some(line) => format!(":{}", line),
                None => "".to_owned(),
            };

            writeln!(
                buf,
                "{} [{}]{}{}{} | {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                level_style.value(record.level()),
                level_spaces,
                record.target(),
                line,
                record.args()
            )
        })
        .filter(None, level)
        .init();

    info!("Logger configuration OK");
}
