use owo_colors::{AnsiColors, Style};

/// Nakama CLI Suite color palette.
///
/// Provides consistent styling across all Nakama tools, inspired by modern
/// CLI aesthetics. Each color constant returns an `owo_colors::Style` that
/// can be applied to any displayable value via the `.style()` method.

/// Primary color (purple/magenta) for headers, emphasis, and branding.
pub fn primary() -> Style {
    Style::new().color(AnsiColors::Magenta)
}

/// Success color (green) for completed operations and confirmations.
pub fn success() -> Style {
    Style::new().color(AnsiColors::Green)
}

/// Warning color (yellow/amber) for non-fatal issues and cautions.
pub fn warning() -> Style {
    Style::new().color(AnsiColors::Yellow)
}

/// Error color (red) for failures and critical issues.
pub fn error() -> Style {
    Style::new().color(AnsiColors::Red)
}

/// Info color (dim/gray) for supplementary information and hints.
pub fn info() -> Style {
    Style::new().dimmed()
}

/// Code color (bright white) for code snippets and literal values.
pub fn code() -> Style {
    Style::new().color(AnsiColors::BrightWhite)
}

/// Bold variant for emphasized content.
pub fn bold() -> Style {
    Style::new().bold()
}

/// Dim variant for de-emphasized content.
pub fn dim() -> Style {
    Style::new().dimmed()
}

/// Unicode symbols used in terminal output.
pub mod symbols {
    /// Checkmark for success states.
    pub const CHECK: &str = "\u{2714}"; // heavy checkmark

    /// Cross for failure states.
    pub const CROSS: &str = "\u{2718}"; // heavy ballot X

    /// Warning triangle.
    pub const WARN: &str = "\u{26A0}"; // warning sign

    /// Info circle (approximated with a bullet).
    pub const INFO: &str = "\u{2139}"; // information source

    /// Right arrow for steps.
    pub const ARROW: &str = "\u{25B6}"; // right-pointing triangle

    /// Dot for list items.
    pub const DOT: &str = "\u{2022}"; // bullet

    /// Box-drawing characters for panels.
    pub const BOX_TOP_LEFT: &str = "\u{256D}";     // rounded corner
    pub const BOX_TOP_RIGHT: &str = "\u{256E}";    // rounded corner
    pub const BOX_BOTTOM_LEFT: &str = "\u{2570}";  // rounded corner
    pub const BOX_BOTTOM_RIGHT: &str = "\u{256F}";  // rounded corner
    pub const BOX_HORIZONTAL: &str = "\u{2500}";    // horizontal line
    pub const BOX_VERTICAL: &str = "\u{2502}";      // vertical line
}
