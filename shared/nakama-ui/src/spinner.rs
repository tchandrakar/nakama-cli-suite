use crate::theme;
use indicatif::{ProgressBar, ProgressStyle};
use owo_colors::OwoColorize;
use std::time::Duration;

/// A braille-style terminal spinner with Nakama styling.
///
/// Wraps `indicatif::ProgressBar` to provide a consistent look and feel
/// across all Nakama CLI tools.
pub struct Spinner {
    bar: ProgressBar,
}

impl Spinner {
    /// Create a new spinner with the given message.
    ///
    /// Uses a braille-dot animation pattern with a purple spinner character.
    pub fn new(message: &str) -> Self {
        let bar = ProgressBar::new_spinner();
        bar.set_style(
            ProgressStyle::default_spinner()
                .tick_strings(&[
                    "\u{2801}", // braille dot patterns
                    "\u{2802}",
                    "\u{2804}",
                    "\u{2840}",
                    "\u{2880}",
                    "\u{2820}",
                    "\u{2810}",
                    "\u{2808}",
                ])
                .template("{spinner:.purple} {msg}")
                .expect("invalid spinner template"),
        );
        bar.set_message(message.to_string());
        bar.enable_steady_tick(Duration::from_millis(80));

        Self { bar }
    }

    /// Stop the spinner and show a success message with a green checkmark.
    pub fn finish_with_success(self, message: &str) {
        self.bar.finish_and_clear();
        println!(
            "  {} {}",
            theme::symbols::CHECK.style(theme::success()),
            message,
        );
    }

    /// Stop the spinner and show an error message with a red cross.
    pub fn finish_with_error(self, message: &str) {
        self.bar.finish_and_clear();
        eprintln!(
            "  {} {}",
            theme::symbols::CROSS.style(theme::error()),
            message,
        );
    }

    /// Update the spinner's message text without stopping it.
    pub fn update_message(&self, message: &str) {
        self.bar.set_message(message.to_string());
    }
}

impl Drop for Spinner {
    fn drop(&mut self) {
        if !self.bar.is_finished() {
            self.bar.finish_and_clear();
        }
    }
}
