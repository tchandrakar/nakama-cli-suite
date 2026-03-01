//! Diff analysis engine.
//!
//! Parses unified diff text into structured [`UnifiedDiff`] with files, hunks,
//! and lines. Provides context enrichment and semantic grouping utilities.

use crate::platform::{DiffFile, DiffLine, Hunk, UnifiedDiff};
use anyhow::Result;

/// Parse raw unified diff text into a structured [`UnifiedDiff`].
pub fn parse_unified_diff(raw: &str) -> UnifiedDiff {
    let mut files: Vec<DiffFile> = Vec::new();
    let mut current_file: Option<DiffFile> = None;
    let mut current_hunk: Option<Hunk> = None;
    let mut old_line: u32 = 0;
    let mut new_line: u32 = 0;

    for line in raw.lines() {
        // File header: --- a/path
        if line.starts_with("--- a/") || line.starts_with("--- /dev/null") {
            // If we have a pending hunk, close it.
            if let (Some(ref mut file), Some(hunk)) = (&mut current_file, current_hunk.take()) {
                file.hunks.push(hunk);
            }
            // If we have a pending file, close it.
            if let Some(file) = current_file.take() {
                files.push(file);
            }
            let old_path = if line == "--- /dev/null" {
                None
            } else {
                Some(line.trim_start_matches("--- a/").to_string())
            };
            current_file = Some(DiffFile {
                path: String::new(), // Set when we see +++ line
                old_path,
                hunks: Vec::new(),
                additions: 0,
                deletions: 0,
            });
            continue;
        }

        // File header: +++ b/path
        if line.starts_with("+++ b/") || line.starts_with("+++ /dev/null") {
            if let Some(ref mut file) = current_file {
                if line == "+++ /dev/null" {
                    file.path = file.old_path.clone().unwrap_or_else(|| "deleted".to_string());
                } else {
                    file.path = line.trim_start_matches("+++ b/").to_string();
                }
            }
            continue;
        }

        // Hunk header: @@ -old_start,old_lines +new_start,new_lines @@
        if line.starts_with("@@") {
            // Close previous hunk.
            if let (Some(ref mut file), Some(hunk)) = (&mut current_file, current_hunk.take()) {
                file.hunks.push(hunk);
            }

            let (old_start, old_lines, new_start, new_lines) = parse_hunk_header(line);
            old_line = old_start;
            new_line = new_start;

            current_hunk = Some(Hunk {
                header: line.to_string(),
                old_start,
                old_lines,
                new_start,
                new_lines,
                lines: Vec::new(),
            });
            continue;
        }

        // diff --git header or other metadata — skip.
        if line.starts_with("diff --git")
            || line.starts_with("index ")
            || line.starts_with("new file")
            || line.starts_with("deleted file")
            || line.starts_with("old mode")
            || line.starts_with("new mode")
            || line.starts_with("similarity index")
            || line.starts_with("rename from")
            || line.starts_with("rename to")
            || line.starts_with("Binary files")
        {
            continue;
        }

        // Content lines.
        if let Some(ref mut hunk) = current_hunk {
            if let Some(content) = line.strip_prefix('+') {
                hunk.lines.push(DiffLine {
                    origin: '+',
                    content: content.to_string(),
                    old_lineno: None,
                    new_lineno: Some(new_line),
                });
                new_line += 1;
                if let Some(ref mut file) = current_file {
                    file.additions += 1;
                }
            } else if let Some(content) = line.strip_prefix('-') {
                hunk.lines.push(DiffLine {
                    origin: '-',
                    content: content.to_string(),
                    old_lineno: Some(old_line),
                    new_lineno: None,
                });
                old_line += 1;
                if let Some(ref mut file) = current_file {
                    file.deletions += 1;
                }
            } else if let Some(content) = line.strip_prefix(' ') {
                hunk.lines.push(DiffLine {
                    origin: ' ',
                    content: content.to_string(),
                    old_lineno: Some(old_line),
                    new_lineno: Some(new_line),
                });
                old_line += 1;
                new_line += 1;
            } else if line == "\\ No newline at end of file" {
                // Skip this marker.
            } else {
                // Context line without prefix.
                hunk.lines.push(DiffLine {
                    origin: ' ',
                    content: line.to_string(),
                    old_lineno: Some(old_line),
                    new_lineno: Some(new_line),
                });
                old_line += 1;
                new_line += 1;
            }
        }
    }

    // Close final hunk and file.
    if let (Some(ref mut file), Some(hunk)) = (&mut current_file, current_hunk.take()) {
        file.hunks.push(hunk);
    }
    if let Some(file) = current_file.take() {
        files.push(file);
    }

    UnifiedDiff {
        raw: raw.to_string(),
        files,
    }
}

/// Parse a hunk header line: @@ -old_start,old_lines +new_start,new_lines @@
fn parse_hunk_header(line: &str) -> (u32, u32, u32, u32) {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let mut old_start = 1u32;
    let mut old_lines = 0u32;
    let mut new_start = 1u32;
    let mut new_lines = 0u32;

    for part in &parts {
        if let Some(old) = part.strip_prefix('-') {
            let nums: Vec<&str> = old.split(',').collect();
            old_start = nums.first().and_then(|n| n.parse().ok()).unwrap_or(1);
            old_lines = nums.get(1).and_then(|n| n.parse().ok()).unwrap_or(0);
        } else if let Some(new) = part.strip_prefix('+') {
            let nums: Vec<&str> = new.split(',').collect();
            new_start = nums.first().and_then(|n| n.parse().ok()).unwrap_or(1);
            new_lines = nums.get(1).and_then(|n| n.parse().ok()).unwrap_or(0);
        }
    }

    (old_start, old_lines, new_start, new_lines)
}

/// Get added lines from a diff (only '+' lines).
pub fn added_lines(diff: &UnifiedDiff) -> Vec<(String, u32, String)> {
    let mut result = Vec::new();
    for file in &diff.files {
        for hunk in &file.hunks {
            for line in &hunk.lines {
                if line.origin == '+' {
                    if let Some(lineno) = line.new_lineno {
                        result.push((file.path.clone(), lineno, line.content.clone()));
                    }
                }
            }
        }
    }
    result
}

/// Split a large diff at file boundaries to fit within a token budget.
pub fn split_diff_by_files(raw: &str, max_chars: usize) -> Vec<String> {
    let mut chunks = Vec::new();
    let mut current_chunk = String::new();
    let mut current_file_block = String::new();

    for line in raw.lines() {
        if line.starts_with("diff --git") {
            // Start of a new file — flush the previous file block.
            if !current_file_block.is_empty() {
                if current_chunk.len() + current_file_block.len() > max_chars && !current_chunk.is_empty()
                {
                    chunks.push(current_chunk);
                    current_chunk = String::new();
                }
                current_chunk.push_str(&current_file_block);
                current_file_block = String::new();
            }
        }
        current_file_block.push_str(line);
        current_file_block.push('\n');
    }

    // Flush remaining.
    if !current_file_block.is_empty() {
        if current_chunk.len() + current_file_block.len() > max_chars && !current_chunk.is_empty() {
            chunks.push(current_chunk);
            current_chunk = String::new();
        }
        current_chunk.push_str(&current_file_block);
    }
    if !current_chunk.is_empty() {
        chunks.push(current_chunk);
    }

    if chunks.is_empty() {
        chunks.push(raw.to_string());
    }

    chunks
}

/// Group related files by directory/module for semantic review.
pub fn group_files_by_directory(diff: &UnifiedDiff) -> Vec<(String, Vec<String>)> {
    use std::collections::BTreeMap;
    let mut groups: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for file in &diff.files {
        let dir = file
            .path
            .rsplit_once('/')
            .map(|(d, _)| d.to_string())
            .unwrap_or_else(|| ".".to_string());
        groups.entry(dir).or_default().push(file.path.clone());
    }

    groups.into_iter().collect()
}

/// Summary statistics for a diff.
pub fn diff_stats(diff: &UnifiedDiff) -> (usize, usize, usize) {
    let files = diff.files.len();
    let additions: usize = diff.files.iter().map(|f| f.additions).sum();
    let deletions: usize = diff.files.iter().map(|f| f.deletions).sum();
    (files, additions, deletions)
}

/// Read surrounding context from the repository for a file at a given line.
pub fn read_context_lines(file_path: &str, line: u32, context: u32) -> Result<String> {
    let repo = git2::Repository::discover(".")?;
    let workdir = repo.workdir().unwrap_or_else(|| std::path::Path::new("."));
    let full_path = workdir.join(file_path);

    if !full_path.exists() {
        return Ok(String::new());
    }

    let content = std::fs::read_to_string(&full_path)?;
    let lines: Vec<&str> = content.lines().collect();

    let start = (line as usize).saturating_sub(context as usize + 1);
    let end = ((line as usize) + context as usize).min(lines.len());

    let mut result = String::new();
    for (i, l) in lines[start..end].iter().enumerate() {
        result.push_str(&format!("{:4} | {}\n", start + i + 1, l));
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_unified_diff() {
        let raw = r#"diff --git a/src/main.rs b/src/main.rs
index abc..def 100644
--- a/src/main.rs
+++ b/src/main.rs
@@ -1,3 +1,4 @@
 fn main() {
-    println!("hello");
+    println!("hello world");
+    println!("goodbye");
 }
"#;
        let diff = parse_unified_diff(raw);
        assert_eq!(diff.files.len(), 1);
        assert_eq!(diff.files[0].path, "src/main.rs");
        assert_eq!(diff.files[0].additions, 2);
        assert_eq!(diff.files[0].deletions, 1);
        assert_eq!(diff.files[0].hunks.len(), 1);
    }

    #[test]
    fn test_parse_hunk_header() {
        let (os, ol, ns, nl) = parse_hunk_header("@@ -10,5 +12,7 @@ fn foo()");
        assert_eq!(os, 10);
        assert_eq!(ol, 5);
        assert_eq!(ns, 12);
        assert_eq!(nl, 7);
    }

    #[test]
    fn test_split_diff_by_files() {
        let raw = "diff --git a/a.rs b/a.rs\n+line1\n+line2\ndiff --git a/b.rs b/b.rs\n+line3\n";
        let chunks = split_diff_by_files(raw, 40);
        assert!(chunks.len() >= 1);
    }
}
