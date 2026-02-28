use nakama_audit::{AuditEntry, AuditLog, Category, Outcome};
use nakama_core::config::Config;
use nakama_core::error::NakamaResult;
use nakama_core::trace::TraceContext;
use nakama_ui::NakamaUI;
use std::fs;
use std::path::Path;
use std::time::Instant;

/// Parse dependency files (Cargo.toml, package.json, requirements.txt) in the current
/// directory and list all dependencies found.
pub async fn run(config: &Config, ui: &NakamaUI) -> NakamaResult<()> {
    let trace = TraceContext::new("senku", "deps");
    let spinner = ui.step_start("Scanning for dependency files...");
    let start = Instant::now();

    let cwd = std::env::current_dir().map_err(|e| nakama_core::error::NakamaError::Tool {
        tool: "senku".to_string(),
        message: format!("Failed to get current directory: {}", e),
    })?;

    let mut found_any = false;

    // Check Cargo.toml
    let cargo_path = cwd.join("Cargo.toml");
    if cargo_path.exists() {
        found_any = true;
        parse_cargo_toml(ui, &cargo_path);
    }

    // Check package.json
    let pkg_path = cwd.join("package.json");
    if pkg_path.exists() {
        found_any = true;
        parse_package_json(ui, &pkg_path);
    }

    // Check requirements.txt
    let req_path = cwd.join("requirements.txt");
    if req_path.exists() {
        found_any = true;
        parse_requirements_txt(ui, &req_path);
    }

    let elapsed = start.elapsed().as_millis() as u64;

    if !found_any {
        spinner.finish_with_error("No dependency files found (Cargo.toml, package.json, requirements.txt)");
    } else {
        spinner.finish_with_success(&format!("Dependency scan complete ({} ms)", elapsed));
    }

    // Audit log
    if let Ok(audit) = AuditLog::new(&config.audit) {
        let entry = AuditEntry::new(
            &trace.trace_id,
            "senku",
            "deps",
            Category::ToolExecution,
            "Listed project dependencies",
            serde_json::json!({
                "directory": cwd.display().to_string(),
                "found_files": found_any,
            }),
            Outcome::Success,
            elapsed,
        );
        let _ = audit.log(entry);
    }

    Ok(())
}

/// Parse a Cargo.toml and display dependencies.
fn parse_cargo_toml(ui: &NakamaUI, path: &Path) {
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            ui.warn(&format!("Could not read Cargo.toml: {}", e));
            return;
        }
    };

    // Parse as JSON value via serde_json by first converting TOML to a Value
    // We parse TOML manually since we only have serde_json available
    // Simple line-based parser for [dependencies] section
    let mut in_deps = false;
    let mut in_dev_deps = false;
    let mut section_deps: Vec<(String, String, &str)> = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with('[') {
            in_deps = trimmed == "[dependencies]" || trimmed == "[workspace.dependencies]";
            in_dev_deps = trimmed == "[dev-dependencies]";
            continue;
        }

        if (in_deps || in_dev_deps) && trimmed.contains('=') && !trimmed.starts_with('#') {
            let parts: Vec<&str> = trimmed.splitn(2, '=').collect();
            if parts.len() == 2 {
                let name = parts[0].trim().to_string();
                let value = parts[1].trim().trim_matches('"').to_string();

                // Extract version from inline table
                let version = if value.starts_with('{') {
                    extract_version_from_inline(&value)
                } else {
                    value.clone()
                };

                let dep_type = if in_dev_deps { "dev" } else { "runtime" };
                section_deps.push((name, version, dep_type));
            }
        }
    }

    if !section_deps.is_empty() {
        ui.panel("Cargo.toml", &format!("Found {} dependencies", section_deps.len()));

        let headers = &["Package", "Version", "Type"];
        let rows: Vec<Vec<String>> = section_deps
            .iter()
            .map(|(name, version, dep_type)| {
                vec![name.clone(), version.clone(), dep_type.to_string()]
            })
            .collect();
        ui.table(headers, rows);
    } else {
        ui.panel("Cargo.toml", "No dependencies found");
    }
}

/// Extract version from an inline TOML table like `{ version = "1.0", features = [...] }`.
fn extract_version_from_inline(value: &str) -> String {
    // Look for version = "..."
    if let Some(pos) = value.find("version") {
        let after = &value[pos..];
        if let Some(eq_pos) = after.find('=') {
            let after_eq = after[eq_pos + 1..].trim();
            // Find quoted string
            if after_eq.starts_with('"') {
                if let Some(end) = after_eq[1..].find('"') {
                    return after_eq[1..1 + end].to_string();
                }
            }
        }
    }

    // Check for workspace = true
    if value.contains("workspace") {
        return "workspace".to_string();
    }

    // Check for path
    if let Some(pos) = value.find("path") {
        let after = &value[pos..];
        if let Some(eq_pos) = after.find('=') {
            let after_eq = after[eq_pos + 1..].trim();
            if after_eq.starts_with('"') {
                if let Some(end) = after_eq[1..].find('"') {
                    return format!("path:{}", &after_eq[1..1 + end]);
                }
            }
        }
    }

    value.to_string()
}

/// Parse a package.json and display dependencies.
fn parse_package_json(ui: &NakamaUI, path: &Path) {
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            ui.warn(&format!("Could not read package.json: {}", e));
            return;
        }
    };

    let pkg: serde_json::Value = match serde_json::from_str(&content) {
        Ok(v) => v,
        Err(e) => {
            ui.warn(&format!("Could not parse package.json: {}", e));
            return;
        }
    };

    let mut all_deps: Vec<(String, String, String)> = Vec::new();

    if let Some(deps) = pkg.get("dependencies").and_then(|d| d.as_object()) {
        for (name, version) in deps {
            all_deps.push((
                name.clone(),
                version.as_str().unwrap_or("unknown").to_string(),
                "runtime".to_string(),
            ));
        }
    }

    if let Some(deps) = pkg.get("devDependencies").and_then(|d| d.as_object()) {
        for (name, version) in deps {
            all_deps.push((
                name.clone(),
                version.as_str().unwrap_or("unknown").to_string(),
                "dev".to_string(),
            ));
        }
    }

    if !all_deps.is_empty() {
        ui.panel("package.json", &format!("Found {} dependencies", all_deps.len()));

        let headers = &["Package", "Version", "Type"];
        let rows: Vec<Vec<String>> = all_deps
            .iter()
            .map(|(name, version, dep_type)| {
                vec![name.clone(), version.clone(), dep_type.clone()]
            })
            .collect();
        ui.table(headers, rows);
    } else {
        ui.panel("package.json", "No dependencies found");
    }
}

/// Parse a requirements.txt and display dependencies.
fn parse_requirements_txt(ui: &NakamaUI, path: &Path) {
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            ui.warn(&format!("Could not read requirements.txt: {}", e));
            return;
        }
    };

    let mut deps: Vec<(String, String)> = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with('-') {
            continue;
        }

        // Parse lines like "package==1.0.0", "package>=1.0", or just "package"
        let (name, version) = if let Some(pos) = trimmed.find("==") {
            (trimmed[..pos].to_string(), trimmed[pos + 2..].to_string())
        } else if let Some(pos) = trimmed.find(">=") {
            (trimmed[..pos].to_string(), format!(">={}", &trimmed[pos + 2..]))
        } else if let Some(pos) = trimmed.find("<=") {
            (trimmed[..pos].to_string(), format!("<={}", &trimmed[pos + 2..]))
        } else if let Some(pos) = trimmed.find("~=") {
            (trimmed[..pos].to_string(), format!("~={}", &trimmed[pos + 2..]))
        } else {
            (trimmed.to_string(), "any".to_string())
        };

        deps.push((name, version));
    }

    if !deps.is_empty() {
        ui.panel("requirements.txt", &format!("Found {} dependencies", deps.len()));

        let headers = &["Package", "Version"];
        let rows: Vec<Vec<String>> = deps
            .iter()
            .map(|(name, version)| vec![name.clone(), version.clone()])
            .collect();
        ui.table(headers, rows);
    } else {
        ui.panel("requirements.txt", "No dependencies found");
    }
}
