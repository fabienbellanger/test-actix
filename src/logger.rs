//! Logger module for customize logs

use chrono::Local;
use env_logger::fmt::Color;
use env_logger::Builder;
use log::Level;
use std::io::Write;
use tracing::Level as TLevel;
use tracing_subscriber::{fmt::format, prelude::*};

/// Initialize logger
pub fn init(level: String) {
    let level = match &*level {
        "trace" => Level::Trace,
        "debug" => Level::Debug,
        "info" => Level::Info,
        "warn" => Level::Warn,
        "Error" => Level::Error,
        &_ => Level::Error,
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

            level_style.set_color(color).set_bold(true);
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
        .filter(None, level.to_level_filter())
        .init();

    info!("Logger configuration OK");
}

pub fn _init_with_tracing(level: String) {
    // Tracing
    // -------
    // tracing_subscriber::fmt()
    //     // .with_env_filter(EnvFilter::from_default_env())
    //     .with_max_level(tracing::Level::TRACE)
    //     .init();
    // https://github.com/tokio-rs/tracing/blob/master/examples/examples/fmt-multiple-writers.rs
    // https://github.com/tokio-rs/tracing/blob/master/examples/examples/fmt-custom-field.rs

    let level = match &*level {
        "trace" => TLevel::TRACE,
        "debug" => TLevel::DEBUG,
        "info" => TLevel::INFO,
        "warn" => TLevel::WARN,
        "Error" => TLevel::ERROR,
        &_ => TLevel::ERROR,
    };

    let format = format::debug_fn(|writer, field, value| {
        // We'll format the field name and value separated with a colon.
        if field.to_string() == "message" || field.to_string() == "log.line" {
            write!(writer, "{}: {:?}", field, value)
        } else {
            write!(writer, "")
        }
    })
    .display_messages()
    .delimited(" || ");

    tracing_subscriber::fmt()
        .with_thread_names(false)
        .with_max_level(level)
        .fmt_fields(format)
        .with_timer(tracing_subscriber::fmt::time::ChronoLocal::with_format(String::from(
            "%Y-%m-%dT%H:%M:%S",
        )))
        .init();

    info!("Logger configuration OK");
}
