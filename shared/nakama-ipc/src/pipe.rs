//! Pipe I/O helpers for reading and writing NMP messages over stdin/stdout.

use crate::message::NmpMessage;
use nakama_core::error::{NakamaError, NakamaResult};
use std::io::{self, BufRead, Write};

/// Read a single NMP message from stdin.
///
/// Reads all available data from stdin and deserialises it as an
/// [`NmpMessage`].  Returns an [`NakamaError::Ipc`] on parse failure.
pub fn read_stdin() -> NakamaResult<NmpMessage> {
    let stdin = io::stdin();
    let mut input = String::new();

    // Read all lines from stdin until EOF.
    for line in stdin.lock().lines() {
        let line = line.map_err(|e| NakamaError::Ipc {
            message: format!("Failed to read from stdin: {e}"),
        })?;
        input.push_str(&line);
        input.push('\n');
    }

    let input = input.trim();
    if input.is_empty() {
        return Err(NakamaError::Ipc {
            message: "No input received on stdin".to_string(),
        });
    }

    serde_json::from_str::<NmpMessage>(input).map_err(|e| NakamaError::Ipc {
        message: format!("Failed to parse NMP message from stdin: {e}"),
    })
}

/// Write an NMP message as pretty-printed JSON to stdout.
pub fn write_stdout(msg: &NmpMessage) -> NakamaResult<()> {
    let json = serde_json::to_string_pretty(msg).map_err(|e| NakamaError::Ipc {
        message: format!("Failed to serialize NMP message: {e}"),
    })?;

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    handle.write_all(json.as_bytes()).map_err(|e| NakamaError::Ipc {
        message: format!("Failed to write NMP message to stdout: {e}"),
    })?;

    // Trailing newline for well-formed pipe output.
    handle.write_all(b"\n").map_err(|e| NakamaError::Ipc {
        message: format!("Failed to write trailing newline to stdout: {e}"),
    })?;

    handle.flush().map_err(|e| NakamaError::Ipc {
        message: format!("Failed to flush stdout: {e}"),
    })?;

    Ok(())
}

/// Returns `true` when stdin is a pipe (not an interactive TTY).
///
/// Tools use this to detect whether they have incoming NMP data to consume.
pub fn is_pipe_input() -> bool {
    #[cfg(unix)]
    {
        use std::os::unix::io::AsRawFd;
        // SAFETY: isatty is a well-defined POSIX function.
        unsafe { libc_isatty(io::stdin().as_raw_fd()) == 0 }
    }
    #[cfg(not(unix))]
    {
        // Conservative default on non-Unix platforms.
        false
    }
}

#[cfg(unix)]
extern "C" {
    fn isatty(fd: std::os::raw::c_int) -> std::os::raw::c_int;
}

#[cfg(unix)]
unsafe fn libc_isatty(fd: std::os::raw::c_int) -> std::os::raw::c_int {
    unsafe { isatty(fd) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::NmpMessage;

    #[test]
    fn test_write_stdout_produces_valid_json() {
        // We cannot easily capture stdout in a unit test, but we can verify
        // that serialization itself succeeds without panicking.
        let msg = NmpMessage::new("test", "cmd", "test.v1", serde_json::json!({"key": "value"}));
        let json = serde_json::to_string_pretty(&msg).unwrap();
        let parsed: NmpMessage = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.source.tool, "test");
    }
}
