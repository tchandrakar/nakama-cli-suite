//! Shared diff compression utilities.
//!
//! Provides smart diff compression (stripping noise files, binary diffs,
//! reducing context lines) and simple sandwich truncation for non-diff text.

/// File patterns to exclude from diffs (lock files, minified assets, generated code).
const EXCLUDED_PATTERNS: &[&str] = &[
    "*.lock",
    "package-lock.json",
    "yarn.lock",
    "pnpm-lock.yaml",
    "go.sum",
    "*.min.js",
    "*.min.css",
    "*.map",
    "*.generated.*",
    "*.pb.go",
    "*.snap",
];

/// Smart diff compression pipeline.
///
/// 1. Strips entire file diffs for excluded patterns (lock files, minified, generated)
/// 2. Strips binary diff sections
/// 3. Reduces context lines to ±3 around actual changes
/// 4. Sandwich-truncates at file boundaries if still over `max_chars`
/// 5. Appends a stats footer
pub fn compress_diff(raw_diff: &str, max_chars: usize) -> String {
    if raw_diff.len() <= max_chars {
        return raw_diff.to_string();
    }

    let mut files_stripped = 0usize;
    let original_len = raw_diff.len();

    // Split diff into per-file sections
    let file_sections = split_into_file_sections(raw_diff);

    // Step 1 & 2: Filter out excluded files and binary diffs
    let mut kept_sections: Vec<&str> = Vec::new();
    for section in &file_sections {
        if is_excluded_file(section) {
            files_stripped += 1;
            continue;
        }
        if is_binary_diff(section) {
            files_stripped += 1;
            continue;
        }
        kept_sections.push(section);
    }

    // Step 3: Reduce context lines in each kept section
    let reduced: Vec<String> = kept_sections
        .iter()
        .map(|s| reduce_context_lines(s, 3))
        .collect();

    let mut result = reduced.join("");

    // Step 4: Sandwich-truncate at file boundaries if still over limit
    if result.len() > max_chars {
        result = truncate_at_file_boundaries(&result, max_chars);
    }

    // Step 5: Stats footer
    let chars_saved = original_len.saturating_sub(result.len());
    if files_stripped > 0 || chars_saved > 0 {
        result.push_str(&format!(
            "\n[compressed: {} file(s) stripped, {} chars saved]\n",
            files_stripped, chars_saved
        ));
    }

    result
}

/// Simple sandwich truncation for non-diff text (e.g. commit lists).
///
/// Keeps the first and last portions with a marker in between.
/// Preserves whole lines at the cut points.
pub fn truncate_diff(text: &str, max_chars: usize) -> String {
    if text.len() <= max_chars {
        return text.to_string();
    }

    let marker = "\n\n... [truncated for AI context window] ...\n\n";
    let available = max_chars.saturating_sub(marker.len());
    let head_budget = available * 2 / 3;
    let tail_budget = available.saturating_sub(head_budget);

    // Find the last newline within head_budget
    let head_end = text[..head_budget.min(text.len())]
        .rfind('\n')
        .map(|i| i + 1)
        .unwrap_or(head_budget.min(text.len()));

    // Find the first newline in the tail region
    let tail_start_min = text.len().saturating_sub(tail_budget);
    let tail_start = text[tail_start_min..]
        .find('\n')
        .map(|i| tail_start_min + i + 1)
        .unwrap_or(tail_start_min);

    let mut result = String::with_capacity(max_chars + marker.len());
    result.push_str(&text[..head_end]);
    result.push_str(marker);
    result.push_str(&text[tail_start..]);
    result
}

/// Split a unified diff into per-file sections.
///
/// Each section starts with a line beginning with `diff --git` (or similar)
/// or `---` at the start of the diff.
fn split_into_file_sections(diff: &str) -> Vec<&str> {
    let mut sections = Vec::new();
    let mut last_start = 0;

    for (i, _) in diff.match_indices("\ndiff --git ") {
        if i > last_start {
            sections.push(&diff[last_start..i + 1]);
        }
        last_start = i + 1; // skip the leading newline
    }

    // Push the last section
    if last_start < diff.len() {
        sections.push(&diff[last_start..]);
    }

    sections
}

/// Check if a file section matches any excluded pattern.
fn is_excluded_file(section: &str) -> bool {
    // Extract the file path from the first line: `diff --git a/path b/path`
    let first_line = section.lines().next().unwrap_or("");

    let path = if first_line.starts_with("diff --git ") {
        // Extract b/path portion
        first_line
            .split(" b/")
            .nth(1)
            .unwrap_or("")
    } else {
        return false;
    };

    if path.is_empty() {
        return false;
    }

    for pattern in EXCLUDED_PATTERNS {
        if matches_glob(path, pattern) {
            return true;
        }
    }

    false
}

/// Simple glob matching: supports `*` prefix (extension match) and exact names.
fn matches_glob(path: &str, pattern: &str) -> bool {
    let filename = path.rsplit('/').next().unwrap_or(path);

    if pattern.starts_with('*') {
        // *.ext or *.foo.* patterns
        let suffix = &pattern[1..]; // e.g. ".lock", ".min.js"
        if suffix.contains('*') {
            // Pattern like *.generated.* — check if filename contains the middle part
            let parts: Vec<&str> = pattern.split('*').collect();
            // For *.generated.*, parts = ["", ".generated.", ""]
            if parts.len() == 3 {
                return filename.contains(parts[1]);
            }
            return false;
        }
        filename.ends_with(suffix)
    } else {
        // Exact filename match
        filename == pattern
    }
}

/// Check if a diff section is a binary diff.
fn is_binary_diff(section: &str) -> bool {
    section.contains("Binary files") && section.contains("differ")
}

/// Reduce context lines in a unified diff section to ±`context` lines around changes.
fn reduce_context_lines(section: &str, context: usize) -> String {
    let lines: Vec<&str> = section.lines().collect();
    let mut keep = vec![false; lines.len()];

    // First pass: mark header lines (anything before the first hunk) as kept
    let mut in_header = true;
    let mut change_indices: Vec<usize> = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        if in_header {
            keep[i] = true;
            if line.starts_with("@@") {
                in_header = false;
            }
            continue;
        }

        // Mark hunk headers
        if line.starts_with("@@") {
            keep[i] = true;
            continue;
        }

        // Track change lines
        if line.starts_with('+') || line.starts_with('-') {
            change_indices.push(i);
        }
    }

    // Second pass: mark ±context lines around each change
    for &ci in &change_indices {
        let start = ci.saturating_sub(context);
        let end = (ci + context + 1).min(lines.len());
        for i in start..end {
            keep[i] = true;
        }
    }

    // Build result, inserting hunk separators where we skip lines
    let mut result = String::new();
    let mut last_kept = true;
    for (i, line) in lines.iter().enumerate() {
        if keep[i] {
            if !last_kept && !line.starts_with("@@") {
                // We skipped lines — no extra marker needed, the hunk headers
                // from the original diff suffice
            }
            result.push_str(line);
            result.push('\n');
            last_kept = true;
        } else {
            last_kept = false;
        }
    }

    result
}

/// Truncate at file boundaries (never split mid-file).
fn truncate_at_file_boundaries(diff: &str, max_chars: usize) -> String {
    let marker = "\n\n... [remaining files truncated] ...\n\n";
    let sections = split_into_file_sections(diff);

    if sections.is_empty() {
        return diff[..max_chars.min(diff.len())].to_string();
    }

    // Strategy: keep files from the start as many as fit, then append tail files
    let budget = max_chars.saturating_sub(marker.len());
    let head_budget = budget * 2 / 3;
    let tail_budget = budget.saturating_sub(head_budget);

    let mut head = String::new();
    let mut head_count = 0;
    for section in &sections {
        if head.len() + section.len() > head_budget && head_count > 0 {
            break;
        }
        head.push_str(section);
        head_count += 1;
    }

    // Tail: work backwards
    let mut tail_sections: Vec<&str> = Vec::new();
    let mut tail_len = 0;
    for section in sections[head_count..].iter().rev() {
        if tail_len + section.len() > tail_budget && !tail_sections.is_empty() {
            break;
        }
        tail_sections.push(section);
        tail_len += section.len();
    }
    tail_sections.reverse();

    if tail_sections.is_empty() {
        head.push_str(marker);
        head
    } else {
        let mut result = head;
        result.push_str(marker);
        for s in tail_sections {
            result.push_str(s);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_file_diff(path: &str, content: &str) -> String {
        format!(
            "diff --git a/{path} b/{path}\n--- a/{path}\n+++ b/{path}\n{content}"
        )
    }

    #[test]
    fn test_strip_excluded_files() {
        let lock_diff = make_file_diff("Cargo.lock", "@@ -1,3 +1,3 @@\n old\n-removed\n+added\n");
        let real_diff = make_file_diff("src/main.rs", "@@ -1,3 +1,3 @@\n old\n-removed\n+added\n");
        let combined = format!("{}\n{}", lock_diff, real_diff);

        let result = compress_diff(&combined, 50); // force compression
        assert!(!result.contains("Cargo.lock"), "should strip .lock files");
        assert!(result.contains("src/main.rs"), "should keep real files");
        assert!(result.contains("1 file(s) stripped"));
    }

    #[test]
    fn test_strip_binary_diffs() {
        let binary = "diff --git a/image.png b/image.png\nBinary files a/image.png and b/image.png differ\n";
        let real = make_file_diff("src/lib.rs", "@@ -1,3 +1,3 @@\n ctx\n-old\n+new\n");
        let combined = format!("{}\n{}", binary, real);

        let result = compress_diff(&combined, 50);
        assert!(!result.contains("image.png"), "should strip binary diffs");
        assert!(result.contains("src/lib.rs"));
    }

    #[test]
    fn test_strip_minified_and_generated() {
        let minjs = make_file_diff("dist/bundle.min.js", "@@ -1 +1 @@\n-old\n+new\n");
        let generated = make_file_diff("api/types.generated.ts", "@@ -1 +1 @@\n-old\n+new\n");
        let pbgo = make_file_diff("proto/msg.pb.go", "@@ -1 +1 @@\n-old\n+new\n");
        let real = make_file_diff("src/app.rs", "@@ -1 +1 @@\n-old\n+new\n");
        let combined = format!("{}\n{}\n{}\n{}", minjs, generated, pbgo, real);

        let result = compress_diff(&combined, 50);
        assert!(!result.contains("bundle.min.js"));
        assert!(!result.contains("types.generated.ts"));
        assert!(!result.contains("msg.pb.go"));
        assert!(result.contains("src/app.rs"));
        assert!(result.contains("3 file(s) stripped"));
    }

    #[test]
    fn test_reduce_context_lines() {
        // Build a diff with many context lines around a single change
        let mut diff_content = String::from("@@ -1,20 +1,20 @@\n");
        for i in 0..8 {
            diff_content.push_str(&format!(" context line {}\n", i));
        }
        diff_content.push_str("-old line\n");
        diff_content.push_str("+new line\n");
        for i in 0..8 {
            diff_content.push_str(&format!(" context line after {}\n", i));
        }

        let section = make_file_diff("src/test.rs", &diff_content);
        let reduced = reduce_context_lines(&section, 3);

        // Should keep the 3 lines before and 3 lines after the change
        assert!(reduced.contains("context line 5"));
        assert!(reduced.contains("context line 6"));
        assert!(reduced.contains("context line 7"));
        assert!(reduced.contains("-old line"));
        assert!(reduced.contains("+new line"));
        assert!(reduced.contains("context line after 0"));
        assert!(reduced.contains("context line after 1"));
        assert!(reduced.contains("context line after 2"));
        // Should NOT keep early context lines
        assert!(!reduced.contains("context line 0\n"), "should drop distant context");
    }

    #[test]
    fn test_truncate_diff_short_text() {
        let text = "short text";
        assert_eq!(truncate_diff(text, 100), "short text");
    }

    #[test]
    fn test_truncate_diff_long_text() {
        let lines: String = (1..=50)
            .map(|i| format!("line {}\n", i))
            .collect();
        let result = truncate_diff(&lines, 200);
        assert!(result.contains("line 1"));
        assert!(result.contains("[truncated for AI context window]"));
        assert!(result.contains("line 50"));
        assert!(result.len() <= lines.len());
    }

    #[test]
    fn test_compress_diff_no_op_when_under_limit() {
        let diff = make_file_diff("src/main.rs", "@@ -1 +1 @@\n-old\n+new\n");
        let result = compress_diff(&diff, 100_000);
        assert_eq!(result, diff);
    }

    #[test]
    fn test_matches_glob_extension() {
        assert!(matches_glob("Cargo.lock", "*.lock"));
        assert!(matches_glob("path/to/Gemfile.lock", "*.lock"));
        assert!(!matches_glob("src/lockfile.rs", "*.lock"));
    }

    #[test]
    fn test_matches_glob_exact() {
        assert!(matches_glob("package-lock.json", "package-lock.json"));
        assert!(matches_glob("path/to/package-lock.json", "package-lock.json"));
        assert!(!matches_glob("not-package-lock.json", "package-lock.json"));
    }

    #[test]
    fn test_matches_glob_generated() {
        assert!(matches_glob("types.generated.ts", "*.generated.*"));
        assert!(matches_glob("foo/bar.generated.go", "*.generated.*"));
        assert!(!matches_glob("generated.txt", "*.generated.*"));
    }

    #[test]
    fn test_full_pipeline() {
        let lock = make_file_diff("yarn.lock", "@@ -1,100 +1,100 @@\n-a\n+b\n");
        let binary = "diff --git a/img.png b/img.png\nBinary files differ\n";
        let real = make_file_diff("src/main.rs", "@@ -1,3 +1,3 @@\n ctx\n-old\n+new\n");
        let combined = format!("{}\n{}\n{}", lock, binary, real);

        let result = compress_diff(&combined, 50);
        assert!(!result.contains("yarn.lock"));
        assert!(!result.contains("img.png"));
        assert!(result.contains("src/main.rs"));
        assert!(result.contains("compressed:"));
    }
}
