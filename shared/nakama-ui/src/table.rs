use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Attribute, Cell, CellAlignment, Color, ContentArrangement, Table};

/// A styled table builder with Nakama aesthetics.
///
/// Wraps `comfy_table::Table` to provide consistent styling across all
/// Nakama CLI tools, with purple headers and rounded UTF-8 borders.
pub struct NakamaTable {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
}

impl NakamaTable {
    /// Create a new table with the given column headers.
    pub fn new(headers: &[&str]) -> Self {
        Self {
            headers: headers.iter().map(|h| h.to_string()).collect(),
            rows: Vec::new(),
        }
    }

    /// Add a row of cells to the table.
    pub fn add_row(&mut self, cells: Vec<String>) {
        self.rows.push(cells);
    }

    /// Render the table to a string with Nakama styling.
    ///
    /// Features:
    /// - UTF-8 rounded borders
    /// - Purple bold headers
    /// - Dynamic content arrangement
    pub fn render(&self) -> String {
        let mut table = Table::new();

        // Apply UTF-8 styling with rounded corners
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_content_arrangement(ContentArrangement::Dynamic);

        // Build styled header cells
        let header_cells: Vec<Cell> = self
            .headers
            .iter()
            .map(|h| {
                Cell::new(h)
                    .fg(Color::Magenta)
                    .add_attribute(Attribute::Bold)
                    .set_alignment(CellAlignment::Left)
            })
            .collect();
        table.set_header(header_cells);

        // Add data rows
        for row in &self.rows {
            let cells: Vec<Cell> = row.iter().map(|c| Cell::new(c)).collect();
            table.add_row(cells);
        }

        table.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_table() {
        let table = NakamaTable::new(&["Name", "Value"]);
        let output = table.render();
        assert!(output.contains("Name"));
        assert!(output.contains("Value"));
    }

    #[test]
    fn test_table_with_rows() {
        let mut table = NakamaTable::new(&["Key", "Status"]);
        table.add_row(vec!["api-key".to_string(), "active".to_string()]);
        table.add_row(vec!["token".to_string(), "expired".to_string()]);
        let output = table.render();
        assert!(output.contains("api-key"));
        assert!(output.contains("active"));
        assert!(output.contains("token"));
        assert!(output.contains("expired"));
    }
}
