//! Tail a log file and highlight errors in red, warnings in yellow.

use nakama_core::config::Config;
use nakama_core::error::NakamaResult;
use nakama_ui::NakamaUI;
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::thread;
use std::time::Duration;

/// Tail a log file, highlighting errors in red and warnings in yellow.
pub async fn run(_config: &Config, _ui: &NakamaUI, source: &str) -> NakamaResult<()> {
    let file = File::open(source).map_err(|e| {
        nakama_core::error::NakamaError::Tool {
            tool: "sharingan".to_string(),
            message: format!("Failed to open log file '{}': {}", source, e),
        }
    })?;

    println!("Tailing {}  (Ctrl+C to stop)\n", source);

    let mut reader = BufReader::new(file);
    // Seek to end of file to only show new lines
    reader.seek(SeekFrom::End(0)).map_err(|e| {
        nakama_core::error::NakamaError::Tool {
            tool: "sharingan".to_string(),
            message: format!("Failed to seek in file: {}", e),
        }
    })?;

    // Also print the last 10 lines for context
    let content = std::fs::read_to_string(source).unwrap_or_default();
    let all_lines: Vec<&str> = content.lines().collect();
    let start = if all_lines.len() > 10 { all_lines.len() - 10 } else { 0 };
    for line in &all_lines[start..] {
        print_highlighted_line(line);
    }

    loop {
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(0) => {
                // No new data, sleep briefly
                thread::sleep(Duration::from_millis(200));
            }
            Ok(_) => {
                let trimmed = line.trim_end();
                print_highlighted_line(trimmed);
            }
            Err(e) => {
                eprintln!("Error reading file: {}", e);
                break Ok(());
            }
        }
    }
}

/// Print a log line with color highlighting based on content.
fn print_highlighted_line(line: &str) {
    let upper = line.to_uppercase();
    if upper.contains("ERROR") || upper.contains("FATAL") || upper.contains("CRITICAL") {
        // Red for errors
        println!("\x1b[31m{}\x1b[0m", line);
    } else if upper.contains("WARN") {
        // Yellow for warnings
        println!("\x1b[33m{}\x1b[0m", line);
    } else if upper.contains("DEBUG") || upper.contains("TRACE") {
        // Dim for debug/trace
        println!("\x1b[2m{}\x1b[0m", line);
    } else {
        println!("{}", line);
    }
}
