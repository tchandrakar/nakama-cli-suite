use nakama_audit::{AuditEntry, AuditLog, Category, Outcome};
use nakama_core::config::Config;
use nakama_core::error::NakamaResult;
use nakama_core::trace::TraceContext;
use nakama_ui::NakamaUI;
use std::fs;
use std::path::Path;
use std::time::Instant;

/// Generate a directory tree visualization of the current working directory.
pub async fn run(config: &Config, ui: &NakamaUI) -> NakamaResult<()> {
    let trace = TraceContext::new("senku", "map");
    let spinner = ui.step_start("Generating directory map...");
    let start = Instant::now();

    let cwd = std::env::current_dir().map_err(|e| nakama_core::error::NakamaError::Tool {
        tool: "senku".to_string(),
        message: format!("Failed to get current directory: {}", e),
    })?;

    let dir_name = cwd
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| cwd.display().to_string());

    let mut tree = String::new();
    tree.push_str(&dir_name);
    tree.push('\n');

    build_tree(&cwd, &mut tree, "", 0);

    let elapsed = start.elapsed().as_millis() as u64;
    spinner.finish_with_success(&format!("Map generated ({} ms)", elapsed));

    ui.panel("Directory Map", &tree);

    // Audit log
    if let Ok(audit) = AuditLog::new(&config.audit) {
        let entry = AuditEntry::new(
            &trace.trace_id,
            "senku",
            "map",
            Category::ToolExecution,
            "Generated directory map",
            serde_json::json!({
                "directory": cwd.display().to_string(),
            }),
            Outcome::Success,
            elapsed,
        );
        let _ = audit.log(entry);
    }

    Ok(())
}

/// Recursively build a tree string representation.
fn build_tree(dir: &Path, tree: &mut String, prefix: &str, depth: usize) {
    if depth > 6 {
        tree.push_str(&format!("{}...\n", prefix));
        return;
    }

    let mut entries: Vec<_> = match fs::read_dir(dir) {
        Ok(entries) => entries.flatten().collect(),
        Err(_) => return,
    };

    // Sort entries: directories first, then files, alphabetically within each group
    entries.sort_by(|a, b| {
        let a_is_dir = a.path().is_dir();
        let b_is_dir = b.path().is_dir();
        match (a_is_dir, b_is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.file_name().cmp(&b.file_name()),
        }
    });

    // Filter out hidden files and common non-source directories
    let entries: Vec<_> = entries
        .into_iter()
        .filter(|e| {
            let name = e.file_name();
            let name_str = name.to_string_lossy();
            !name_str.starts_with('.')
                && name_str != "target"
                && name_str != "node_modules"
                && name_str != "__pycache__"
        })
        .collect();

    let total = entries.len();

    for (i, entry) in entries.iter().enumerate() {
        let is_last = i == total - 1;
        let connector = if is_last { "└── " } else { "├── " };
        let child_prefix = if is_last { "    " } else { "│   " };

        let name = entry.file_name();
        let name_str = name.to_string_lossy();
        let path = entry.path();

        if path.is_dir() {
            tree.push_str(&format!("{}{}{}/\n", prefix, connector, name_str));
            build_tree(&path, tree, &format!("{}{}", prefix, child_prefix), depth + 1);
        } else {
            tree.push_str(&format!("{}{}{}\n", prefix, connector, name_str));
        }
    }
}
