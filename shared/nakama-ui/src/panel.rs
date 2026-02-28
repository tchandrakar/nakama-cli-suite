use crate::theme;
use owo_colors::OwoColorize;

/// Render a boxed panel with a title and content, using Unicode rounded borders.
///
/// The panel is styled with purple borders and a bold title. Content is
/// wrapped within the box with consistent padding.
///
/// # Example output
/// ```text
/// +-- My Title ------+
/// |  Some content     |
/// |  goes here.       |
/// +-------------------+
/// ```
pub struct Panel;

impl Panel {
    /// Render a panel to a styled string.
    ///
    /// # Arguments
    /// - `title` - The panel title, displayed in the top border
    /// - `content` - The panel body, may contain multiple lines
    ///
    /// # Returns
    /// A fully rendered string ready for printing to the terminal.
    pub fn new(title: &str, content: &str) -> String {
        let content_lines: Vec<&str> = content.lines().collect();

        // Calculate the width: max of title + padding or widest content line + padding
        let title_width = title.len() + 4; // " title " plus borders
        let max_content_width = content_lines
            .iter()
            .map(|line| line.len())
            .max()
            .unwrap_or(0);
        let inner_width = std::cmp::max(title_width, max_content_width + 2);

        // Build top border: +-- Title ---...---+
        let title_display = format!(" {} ", title);
        let remaining = inner_width.saturating_sub(title_display.len());
        let top = format!(
            "{}{}{}{}",
            theme::symbols::BOX_TOP_LEFT,
            theme::symbols::BOX_HORIZONTAL,
            title_display,
            theme::symbols::BOX_HORIZONTAL.repeat(remaining.max(1)),
        );
        // Ensure the top border ends with the top-right corner
        let top = format!(
            "{}{}",
            &top[..top.len().min(top.len())],
            theme::symbols::BOX_TOP_RIGHT,
        );

        // Build bottom border
        // +2 accounts for the single horizontal after top-left and the title section
        let bottom_width = inner_width + 2; // match the inner content width
        let bottom = format!(
            "{}{}{}",
            theme::symbols::BOX_BOTTOM_LEFT,
            theme::symbols::BOX_HORIZONTAL.repeat(bottom_width),
            theme::symbols::BOX_BOTTOM_RIGHT,
        );

        // Build content lines
        let mut lines = Vec::new();
        lines.push(top.style(theme::primary()).to_string());

        for line in &content_lines {
            let padding = inner_width.saturating_sub(line.len());
            let rendered = format!(
                "{} {}{} {}",
                theme::symbols::BOX_VERTICAL,
                line,
                " ".repeat(padding),
                theme::symbols::BOX_VERTICAL,
            );
            lines.push(format!(
                "{}{}{}",
                theme::symbols::BOX_VERTICAL.style(theme::primary()),
                &rendered[theme::symbols::BOX_VERTICAL.len()..rendered.len() - theme::symbols::BOX_VERTICAL.len()],
                theme::symbols::BOX_VERTICAL.style(theme::primary()),
            ));
        }

        // Empty line if content was empty
        if content_lines.is_empty() {
            let rendered = format!(
                "{} {}{} {}",
                theme::symbols::BOX_VERTICAL.style(theme::primary()),
                "",
                " ".repeat(inner_width),
                theme::symbols::BOX_VERTICAL.style(theme::primary()),
            );
            lines.push(rendered);
        }

        lines.push(bottom.style(theme::primary()).to_string());

        lines.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_panel_contains_title() {
        let output = Panel::new("Test Title", "Hello world");
        // Panel should contain the title text (ignoring ANSI codes)
        assert!(output.contains("Test Title"));
    }

    #[test]
    fn test_panel_contains_content() {
        let output = Panel::new("Title", "Content line");
        assert!(output.contains("Content line"));
    }

    #[test]
    fn test_panel_multiline() {
        let output = Panel::new("Info", "Line one\nLine two\nLine three");
        assert!(output.contains("Line one"));
        assert!(output.contains("Line two"));
        assert!(output.contains("Line three"));
    }

    #[test]
    fn test_panel_empty_content() {
        // Should not panic with empty content
        let output = Panel::new("Empty", "");
        assert!(output.contains("Empty"));
    }
}
