use std::io::Write;

/// Log a preformatted string into the temp log file.
pub fn _log(msg: &str, filename: &str, line: u32, column: u32) -> Result<(), String> {
    let mut path = std::env::temp_dir();
    path.push("term-cave-crawl-rpg.log");

    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .map_err(|e| format!("failed to open log file {:?}: {}", path, e))?;

    let ts = match time_format::now() {
        Ok(d) => d,
        Err(_) => 0
    };
    let line = format!("[{}] [{}:{}:{}] {}\n",
        time_format::strftime_local("%H:%M:%S", ts).expect("Expected a valid format"),
        filename, line, column,
        msg
    );
    file.write_all(line.as_bytes()).map_err(|e| format!("failed to write log: {}", e))?;
    Ok(())
}

/// Log using `std::fmt::Arguments` (used by the `log!` macro).
pub fn _log_fmt(args: std::fmt::Arguments, file: &str, line: u32, column: u32) -> Result<(), String> {
    let mut s = String::new();
    std::fmt::write(&mut s, args).map_err(|e| format!("format error: {}", e))?;
    _log(&s, file, line, column)
}

// Macro-style logging: `crate::log!("{} {}", a, b)`
#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{
        $crate::util::logger::_log_fmt(format_args!($($arg)*), file!(), line!(), column!())
    }};
}
