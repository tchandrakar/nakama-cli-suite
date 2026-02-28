//! Log line parser that auto-detects common log formats.
//!
//! Supported formats:
//! - Syslog (`Jan  1 12:00:00 host service[pid]: message`)
//! - JSON (`{"timestamp":"...","level":"...","message":"..."}`)
//! - Apache/Nginx combined (`127.0.0.1 - - [01/Jan/2024:12:00:00 +0000] "GET / HTTP/1.1" 200 ...`)
//! - Generic timestamped (`2024-01-01 12:00:00 ERROR message`)
//! - Bare (fallback: entire line is the message)

use regex::Regex;
use std::fmt;
use std::sync::LazyLock;

/// Severity level extracted from a log line.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogLevel::Trace => write!(f, "TRACE"),
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warn => write!(f, "WARN"),
            LogLevel::Error => write!(f, "ERROR"),
        }
    }
}

/// A single parsed log line.
#[derive(Debug, Clone)]
pub struct LogLine {
    /// Extracted timestamp string (format varies by log type).
    pub timestamp: Option<String>,
    /// Detected severity level.
    pub level: Option<LogLevel>,
    /// The human-readable message portion.
    pub message: String,
    /// The original raw line.
    pub raw: String,
}

/// The detected log format of a file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogFormat {
    Syslog,
    Json,
    Apache,
    Generic,
    Bare,
}

impl fmt::Display for LogFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogFormat::Syslog => write!(f, "syslog"),
            LogFormat::Json => write!(f, "JSON"),
            LogFormat::Apache => write!(f, "Apache/Nginx"),
            LogFormat::Generic => write!(f, "generic timestamped"),
            LogFormat::Bare => write!(f, "bare text"),
        }
    }
}

// --- Compiled regex patterns (created once, reused) ---

static RE_SYSLOG: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"^(?P<ts>[A-Z][a-z]{2}\s+\d{1,2}\s+\d{2}:\d{2}:\d{2})\s+\S+\s+\S+:\s+(?P<msg>.*)$",
    )
    .unwrap()
});

static RE_APACHE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"^\S+\s+\S+\s+\S+\s+\[(?P<ts>[^\]]+)\]\s+"(?P<method>\S+)\s+(?P<path>\S+)\s+\S+"\s+(?P<status>\d{3})\s+(?P<size>\S+)"#,
    )
    .unwrap()
});

static RE_GENERIC: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"^(?P<ts>\d{4}[-/]\d{2}[-/]\d{2}[T ]\d{2}:\d{2}:\d{2}(?:[.,]\d+)?(?:Z|[+-]\d{2}:?\d{2})?)\s+(?:\[?(?P<level>TRACE|DEBUG|INFO|WARN(?:ING)?|ERROR|FATAL|CRITICAL)\]?\s+)?(?P<msg>.*)$",
    )
    .unwrap()
});

static RE_LEVEL: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(TRACE|DEBUG|INFO|WARN(?:ING)?|ERROR|FATAL|CRITICAL)\b").unwrap()
});

/// Parse a level string (case-insensitive) into a `LogLevel`.
fn parse_level(s: &str) -> Option<LogLevel> {
    match s.to_uppercase().as_str() {
        "TRACE" => Some(LogLevel::Trace),
        "DEBUG" => Some(LogLevel::Debug),
        "INFO" => Some(LogLevel::Info),
        "WARN" | "WARNING" => Some(LogLevel::Warn),
        "ERROR" | "FATAL" | "CRITICAL" => Some(LogLevel::Error),
        _ => None,
    }
}

/// Detect the log format from a sample of lines.
pub fn detect_format(lines: &[&str]) -> LogFormat {
    let sample: Vec<&&str> = lines.iter().filter(|l| !l.trim().is_empty()).take(20).collect();
    if sample.is_empty() {
        return LogFormat::Bare;
    }

    let mut json_count = 0;
    let mut syslog_count = 0;
    let mut apache_count = 0;
    let mut generic_count = 0;

    for line in &sample {
        let trimmed = line.trim();
        if trimmed.starts_with('{') && trimmed.ends_with('}') {
            json_count += 1;
        }
        if RE_SYSLOG.is_match(trimmed) {
            syslog_count += 1;
        }
        if RE_APACHE.is_match(trimmed) {
            apache_count += 1;
        }
        if RE_GENERIC.is_match(trimmed) {
            generic_count += 1;
        }
    }

    let total = sample.len();
    let threshold = total / 2;

    // Pick the format that matches most lines (JSON > Apache > Syslog > Generic)
    if json_count > threshold {
        LogFormat::Json
    } else if apache_count > threshold {
        LogFormat::Apache
    } else if syslog_count > threshold {
        LogFormat::Syslog
    } else if generic_count > threshold {
        LogFormat::Generic
    } else {
        LogFormat::Bare
    }
}

/// Parse a single log line using the given format hint.
pub fn parse_line(line: &str, format: LogFormat) -> LogLine {
    match format {
        LogFormat::Json => parse_json_line(line),
        LogFormat::Syslog => parse_syslog_line(line),
        LogFormat::Apache => parse_apache_line(line),
        LogFormat::Generic => parse_generic_line(line),
        LogFormat::Bare => parse_bare_line(line),
    }
}

fn parse_json_line(line: &str) -> LogLine {
    if let Ok(value) = serde_json::from_str::<serde_json::Value>(line) {
        let timestamp = value
            .get("timestamp")
            .or_else(|| value.get("ts"))
            .or_else(|| value.get("time"))
            .or_else(|| value.get("@timestamp"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let level_str = value
            .get("level")
            .or_else(|| value.get("severity"))
            .or_else(|| value.get("log_level"))
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let level = parse_level(level_str);

        let message = value
            .get("message")
            .or_else(|| value.get("msg"))
            .or_else(|| value.get("text"))
            .and_then(|v| v.as_str())
            .unwrap_or(line)
            .to_string();

        LogLine {
            timestamp,
            level,
            message,
            raw: line.to_string(),
        }
    } else {
        parse_bare_line(line)
    }
}

fn parse_syslog_line(line: &str) -> LogLine {
    if let Some(caps) = RE_SYSLOG.captures(line) {
        let ts = caps.name("ts").map(|m| m.as_str().to_string());
        let msg = caps
            .name("msg")
            .map(|m| m.as_str().to_string())
            .unwrap_or_else(|| line.to_string());

        // Try to extract level from message body
        let level = RE_LEVEL
            .find(&msg)
            .and_then(|m| parse_level(m.as_str()));

        LogLine {
            timestamp: ts,
            level,
            message: msg,
            raw: line.to_string(),
        }
    } else {
        parse_bare_line(line)
    }
}

fn parse_apache_line(line: &str) -> LogLine {
    if let Some(caps) = RE_APACHE.captures(line) {
        let ts = caps.name("ts").map(|m| m.as_str().to_string());
        let status: u16 = caps
            .name("status")
            .and_then(|m| m.as_str().parse().ok())
            .unwrap_or(0);
        let method = caps.name("method").map(|m| m.as_str()).unwrap_or("?");
        let path = caps.name("path").map(|m| m.as_str()).unwrap_or("?");

        let level = if status >= 500 {
            Some(LogLevel::Error)
        } else if status >= 400 {
            Some(LogLevel::Warn)
        } else {
            Some(LogLevel::Info)
        };

        let message = format!("{} {} -> {}", method, path, status);

        LogLine {
            timestamp: ts,
            level,
            message,
            raw: line.to_string(),
        }
    } else {
        parse_bare_line(line)
    }
}

fn parse_generic_line(line: &str) -> LogLine {
    if let Some(caps) = RE_GENERIC.captures(line) {
        let ts = caps.name("ts").map(|m| m.as_str().to_string());
        let level = caps
            .name("level")
            .and_then(|m| parse_level(m.as_str()));
        let msg = caps
            .name("msg")
            .map(|m| m.as_str().to_string())
            .unwrap_or_else(|| line.to_string());

        // If level not captured from the fixed group, try to find it anywhere
        let level = level.or_else(|| {
            RE_LEVEL.find(&msg).and_then(|m| parse_level(m.as_str()))
        });

        LogLine {
            timestamp: ts,
            level,
            message: msg,
            raw: line.to_string(),
        }
    } else {
        parse_bare_line(line)
    }
}

fn parse_bare_line(line: &str) -> LogLine {
    let level = RE_LEVEL
        .find(line)
        .and_then(|m| parse_level(m.as_str()));

    LogLine {
        timestamp: None,
        level,
        message: line.to_string(),
        raw: line.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_json_line() {
        let line = r#"{"timestamp":"2024-01-15T10:30:00Z","level":"ERROR","message":"Connection refused"}"#;
        let parsed = parse_line(line, LogFormat::Json);
        assert_eq!(parsed.level, Some(LogLevel::Error));
        assert_eq!(parsed.message, "Connection refused");
        assert!(parsed.timestamp.is_some());
    }

    #[test]
    fn test_parse_syslog_line() {
        let line = "Jan 15 10:30:00 myhost sshd[1234]: Failed password for root";
        let parsed = parse_line(line, LogFormat::Syslog);
        assert!(parsed.timestamp.is_some());
        assert_eq!(parsed.message, "Failed password for root");
    }

    #[test]
    fn test_parse_apache_line() {
        let line = r#"127.0.0.1 - frank [10/Oct/2024:13:55:36 -0700] "GET /apache_pb.gif HTTP/1.0" 500 2326"#;
        let parsed = parse_line(line, LogFormat::Apache);
        assert_eq!(parsed.level, Some(LogLevel::Error));
        assert!(parsed.message.contains("500"));
    }

    #[test]
    fn test_parse_generic_line() {
        let line = "2024-01-15 10:30:00 ERROR Something went wrong";
        let parsed = parse_line(line, LogFormat::Generic);
        assert_eq!(parsed.level, Some(LogLevel::Error));
        assert!(parsed.message.contains("Something went wrong"));
    }

    #[test]
    fn test_detect_format_json() {
        let lines = vec![
            r#"{"timestamp":"2024-01-15T10:30:00Z","level":"INFO","message":"ok"}"#,
            r#"{"timestamp":"2024-01-15T10:30:01Z","level":"ERROR","message":"fail"}"#,
            r#"{"timestamp":"2024-01-15T10:30:02Z","level":"DEBUG","message":"test"}"#,
        ];
        assert_eq!(detect_format(&lines), LogFormat::Json);
    }

    #[test]
    fn test_detect_format_generic() {
        let lines = vec![
            "2024-01-15 10:30:00 INFO Starting up",
            "2024-01-15 10:30:01 ERROR Failed to connect",
            "2024-01-15 10:30:02 WARN Retrying",
        ];
        assert_eq!(detect_format(&lines), LogFormat::Generic);
    }

    #[test]
    fn test_parse_level_variants() {
        assert_eq!(parse_level("WARNING"), Some(LogLevel::Warn));
        assert_eq!(parse_level("FATAL"), Some(LogLevel::Error));
        assert_eq!(parse_level("CRITICAL"), Some(LogLevel::Error));
        assert_eq!(parse_level("info"), Some(LogLevel::Info));
    }
}
