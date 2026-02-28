use crate::panel::Panel;
use crate::spinner::Spinner;
use crate::table::NakamaTable;
use crate::theme;
use crossterm::tty::IsTty;
use nakama_core::config::{Config, UiConfig};
use nakama_core::types::{ColorMode, Verbosity};
use owo_colors::OwoColorize;
use std::io::{self, Write};

/// The primary UI interface for all Nakama CLI tools.
///
/// Provides styled terminal output with automatic TTY detection, color mode
/// control, and verbosity filtering. All Nakama tools should use this struct
/// instead of printing directly.
pub struct NakamaUI {
    /// Current verbosity level; messages below this threshold are suppressed.
    verbosity: Verbosity,
    /// Color output mode.
    color: ColorMode,
    /// Whether stdout is connected to a TTY (interactive terminal).
    is_tty: bool,
}

impl NakamaUI {
    /// Create a `NakamaUI` from the global Nakama configuration.
    pub fn from_config(config: &Config) -> Self {
        let is_tty = io::stdout().is_tty();
        let ui_config: &UiConfig = &config.ui;

        // Resolve color mode
        let color = match ui_config.color {
            ColorMode::Auto => {
                if is_tty {
                    ColorMode::Always
                } else {
                    ColorMode::Never
                }
            }
            other => other,
        };

        // If colors are disabled, tell owo-colors globally
        if color == ColorMode::Never {
            owo_colors::set_override(false);
        } else if color == ColorMode::Always {
            owo_colors::set_override(true);
        }

        Self {
            verbosity: ui_config.verbosity,
            color,
            is_tty,
        }
    }

    /// Create a `NakamaUI` with explicit settings (useful for testing).
    pub fn new(verbosity: Verbosity, color: ColorMode, is_tty: bool) -> Self {
        Self {
            verbosity,
            color,
            is_tty,
        }
    }

    /// Start an indeterminate progress spinner for a long-running step.
    ///
    /// Returns a `Spinner` that should be finished with `finish_with_success`
    /// or `finish_with_error`.
    pub fn step_start(&self, message: &str) -> Spinner {
        Spinner::new(message)
    }

    /// Print a success step line with a green checkmark.
    pub fn step_done(&self, message: &str) {
        if self.verbosity >= Verbosity::Normal {
            println!(
                "  {} {}",
                theme::symbols::CHECK.style(theme::success()),
                message,
            );
        }
    }

    /// Print a failure step line with a red cross.
    pub fn step_fail(&self, message: &str) {
        // Always show failures, even in quiet mode
        eprintln!(
            "  {} {}",
            theme::symbols::CROSS.style(theme::error()),
            message,
        );
    }

    /// Print a warning message with a yellow warning symbol.
    pub fn warn(&self, message: &str) {
        if self.verbosity >= Verbosity::Quiet {
            eprintln!(
                "  {} {}",
                theme::symbols::WARN.style(theme::warning()),
                message.style(theme::warning()),
            );
        }
    }

    /// Print an error message with a red cross symbol.
    pub fn error(&self, message: &str) {
        eprintln!(
            "  {} {}",
            theme::symbols::CROSS.style(theme::error()),
            message.style(theme::error()),
        );
    }

    /// Print an informational message in dim/gray text.
    pub fn info(&self, message: &str) {
        if self.verbosity >= Verbosity::Verbose {
            println!(
                "  {} {}",
                theme::symbols::INFO.style(theme::info()),
                message.style(theme::info()),
            );
        }
    }

    /// Print a success message with a green checkmark.
    pub fn success(&self, message: &str) {
        if self.verbosity >= Verbosity::Normal {
            println!(
                "  {} {}",
                theme::symbols::CHECK.style(theme::success()),
                message.style(theme::success()),
            );
        }
    }

    /// Render and print a styled table.
    ///
    /// # Arguments
    /// - `headers` - Column header labels
    /// - `rows` - Row data, each inner Vec corresponds to one row
    pub fn table(&self, headers: &[&str], rows: Vec<Vec<String>>) {
        if self.verbosity >= Verbosity::Normal {
            let mut table = NakamaTable::new(headers);
            for row in rows {
                table.add_row(row);
            }
            println!("{}", table.render());
        }
    }

    /// Render and print a boxed panel with a title and content.
    pub fn panel(&self, title: &str, content: &str) {
        if self.verbosity >= Verbosity::Normal {
            println!("{}", Panel::new(title, content));
        }
    }

    /// Prompt the user for a Y/n confirmation.
    ///
    /// Returns `Ok(true)` if the user confirms, `Ok(false)` if they decline.
    /// Returns an error if stdin is not a TTY (non-interactive context).
    ///
    /// Default is "yes" (pressing Enter without input confirms).
    pub fn confirm(&self, message: &str) -> Result<bool, io::Error> {
        if !self.is_tty {
            return Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "Cannot prompt for confirmation: not a TTY (use --yes flag for non-interactive mode)",
            ));
        }

        print!(
            "  {} {} [Y/n] ",
            theme::symbols::ARROW.style(theme::primary()),
            message,
        );
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let trimmed = input.trim().to_lowercase();
        Ok(trimmed.is_empty() || trimmed == "y" || trimmed == "yes")
    }

    /// Check whether the terminal is interactive (TTY).
    pub fn is_tty(&self) -> bool {
        self.is_tty
    }

    /// Get the current verbosity level.
    pub fn verbosity(&self) -> Verbosity {
        self.verbosity
    }

    /// Get the current color mode.
    pub fn color_mode(&self) -> ColorMode {
        self.color
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ui_creation() {
        let config = Config::default();
        let ui = NakamaUI::from_config(&config);
        assert_eq!(ui.verbosity(), Verbosity::Normal);
    }

    #[test]
    fn test_ui_manual_creation() {
        let ui = NakamaUI::new(Verbosity::Quiet, ColorMode::Never, false);
        assert_eq!(ui.verbosity(), Verbosity::Quiet);
        assert_eq!(ui.color_mode(), ColorMode::Never);
        assert!(!ui.is_tty());
    }

    #[test]
    fn test_confirm_not_tty() {
        let ui = NakamaUI::new(Verbosity::Normal, ColorMode::Never, false);
        let result = ui.confirm("Proceed?");
        assert!(result.is_err());
    }
}
