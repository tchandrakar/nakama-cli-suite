use nakama_audit::{AuditEntry, AuditLog, Category, Outcome};
use nakama_core::config::Config;
use nakama_core::error::NakamaResult;
use nakama_core::trace::TraceContext;
use nakama_ui::NakamaUI;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::time::Instant;

/// Walk the current working directory, collect file stats (count by extension, total lines),
/// and display the results in a table.
pub async fn run(config: &Config, ui: &NakamaUI) -> NakamaResult<()> {
    let trace = TraceContext::new("senku", "index");
    let spinner = ui.step_start("Indexing codebase...");
    let start = Instant::now();

    let cwd = std::env::current_dir().map_err(|e| nakama_core::error::NakamaError::Tool {
        tool: "senku".to_string(),
        message: format!("Failed to get current directory: {}", e),
    })?;

    let mut ext_counts: HashMap<String, usize> = HashMap::new();
    let mut ext_lines: HashMap<String, usize> = HashMap::new();
    let mut total_files: usize = 0;
    let mut total_lines: usize = 0;

    walk_dir(&cwd, &mut ext_counts, &mut ext_lines, &mut total_files, &mut total_lines);

    let elapsed = start.elapsed().as_millis() as u64;
    spinner.finish_with_success(&format!("Indexed {} files ({} ms)", total_files, elapsed));

    // Sort extensions by file count descending
    let mut ext_stats: Vec<(String, usize, usize)> = ext_counts
        .iter()
        .map(|(ext, count)| {
            let lines = ext_lines.get(ext).copied().unwrap_or(0);
            (ext.clone(), *count, lines)
        })
        .collect();
    ext_stats.sort_by(|a, b| b.1.cmp(&a.1));

    // Display as table
    let headers = &["Extension", "Files", "Lines"];
    let rows: Vec<Vec<String>> = ext_stats
        .iter()
        .map(|(ext, count, lines)| {
            vec![ext.clone(), count.to_string(), lines.to_string()]
        })
        .collect();
    ui.table(headers, rows);

    ui.panel(
        "Codebase Summary",
        &format!(
            "Directory: {}\nTotal files: {}\nTotal lines: {}\nFile types: {}",
            cwd.display(),
            total_files,
            total_lines,
            ext_stats.len()
        ),
    );

    // Audit log
    if let Ok(audit) = AuditLog::new(&config.audit) {
        let entry = AuditEntry::new(
            &trace.trace_id,
            "senku",
            "index",
            Category::ToolExecution,
            "Indexed codebase",
            serde_json::json!({
                "directory": cwd.display().to_string(),
                "total_files": total_files,
                "total_lines": total_lines,
                "file_types": ext_stats.len(),
            }),
            Outcome::Success,
            elapsed,
        );
        let _ = audit.log(entry);
    }

    Ok(())
}

/// Recursively walk a directory, counting files and lines by extension.
fn walk_dir(
    dir: &Path,
    ext_counts: &mut HashMap<String, usize>,
    ext_lines: &mut HashMap<String, usize>,
    total_files: &mut usize,
    total_lines: &mut usize,
) {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        let file_name = entry.file_name();
        let name = file_name.to_string_lossy();

        // Skip hidden directories and common non-source directories
        if name.starts_with('.') || name == "target" || name == "node_modules" || name == "__pycache__" {
            continue;
        }

        if path.is_dir() {
            walk_dir(&path, ext_counts, ext_lines, total_files, total_lines);
        } else if path.is_file() {
            let ext = path
                .extension()
                .map(|e| format!(".{}", e.to_string_lossy()))
                .unwrap_or_else(|| "(no ext)".to_string());

            *total_files += 1;
            *ext_counts.entry(ext.clone()).or_insert(0) += 1;

            // Count lines (skip binary-looking files)
            if let Ok(content) = fs::read_to_string(&path) {
                let line_count = content.lines().count();
                *total_lines += line_count;
                *ext_lines.entry(ext).or_insert(0) += line_count;
            }
        }
    }
}
