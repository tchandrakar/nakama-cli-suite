//! Git operations wrapper using `git2`.
//!
//! Provides high-level helpers for reading staged diffs, recent logs,
//! current branch info, and performing commits — all backed by libgit2
//! so shinigami never shells out to the `git` CLI.

use anyhow::{Context, Result};
use git2::{DiffFormat, DiffOptions, Repository, Sort};

/// A single commit entry from the log.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct LogEntry {
    pub hash: String,
    pub short_hash: String,
    pub summary: String,
    pub body: Option<String>,
    pub author: String,
    pub date: String,
}

/// Discover and open the git repository that contains the current directory.
pub fn open_repo() -> Result<Repository> {
    Repository::discover(".").context("Not inside a git repository. Run this command from within a git repo.")
}

/// Return the name of the current branch (e.g. `"main"`) or `"HEAD"` if detached.
pub fn current_branch(repo: &Repository) -> Result<String> {
    let head = repo.head().context("Failed to read HEAD")?;
    if head.is_branch() {
        Ok(head
            .shorthand()
            .unwrap_or("HEAD")
            .to_string())
    } else {
        Ok("HEAD (detached)".to_string())
    }
}

/// Get the diff of **staged** changes (index vs. HEAD tree).
///
/// Returns the diff as a unified patch string. If the repo has no commits yet
/// (initial commit scenario) the diff is computed against an empty tree.
pub fn get_staged_diff(repo: &Repository) -> Result<String> {
    let head_tree = match repo.head() {
        Ok(reference) => {
            let commit = reference
                .peel_to_commit()
                .context("HEAD does not point to a commit")?;
            Some(commit.tree().context("Failed to read HEAD tree")?)
        }
        // No commits yet — diff against empty tree.
        Err(_) => None,
    };

    let index = repo.index().context("Failed to read index")?;

    let diff = repo
        .diff_tree_to_index(head_tree.as_ref(), Some(&index), None)
        .context("Failed to compute staged diff")?;

    diff_to_string(&diff)
}

/// Get the diff of **unstaged** (working-directory) changes against the index.
#[allow(dead_code)]
pub fn get_unstaged_diff(repo: &Repository) -> Result<String> {
    let mut opts = DiffOptions::new();
    opts.include_untracked(true);

    let diff = repo
        .diff_index_to_workdir(None, Some(&mut opts))
        .context("Failed to compute working-directory diff")?;

    diff_to_string(&diff)
}

/// Get the **full** diff of all uncommitted changes (staged + unstaged) vs HEAD.
pub fn get_all_uncommitted_diff(repo: &Repository) -> Result<String> {
    let head_tree = match repo.head() {
        Ok(reference) => {
            let commit = reference
                .peel_to_commit()
                .context("HEAD does not point to a commit")?;
            Some(commit.tree().context("Failed to read HEAD tree")?)
        }
        Err(_) => None,
    };

    let mut opts = DiffOptions::new();
    opts.include_untracked(true);

    let diff = repo
        .diff_tree_to_workdir_with_index(head_tree.as_ref(), Some(&mut opts))
        .context("Failed to compute uncommitted diff")?;

    diff_to_string(&diff)
}

/// List file names in the staging area (paths that have staged changes).
pub fn staged_file_names(repo: &Repository) -> Result<Vec<String>> {
    let head_tree = match repo.head() {
        Ok(reference) => {
            let commit = reference.peel_to_commit()?;
            Some(commit.tree()?)
        }
        Err(_) => None,
    };

    let index = repo.index()?;
    let diff = repo.diff_tree_to_index(head_tree.as_ref(), Some(&index), None)?;

    let mut files = Vec::new();
    for delta in diff.deltas() {
        if let Some(path) = delta.new_file().path() {
            files.push(path.to_string_lossy().to_string());
        }
    }
    Ok(files)
}

/// Retrieve the most recent `count` commits from HEAD.
pub fn get_log(repo: &Repository, count: usize) -> Result<Vec<LogEntry>> {
    let mut revwalk = repo.revwalk().context("Failed to create revwalk")?;
    revwalk.push_head().context("Failed to push HEAD to revwalk")?;
    revwalk.set_sorting(Sort::TIME)?;

    let mut entries = Vec::new();
    for (i, oid_result) in revwalk.enumerate() {
        if i >= count {
            break;
        }
        let oid = oid_result.context("Failed to iterate revwalk")?;
        let commit = repo.find_commit(oid).context("Failed to find commit")?;

        let hash = oid.to_string();
        let short_hash = hash[..7.min(hash.len())].to_string();
        let message = commit.message().unwrap_or("");
        let summary = commit.summary().unwrap_or("").to_string();
        let body = {
            let full = message.to_string();
            let after_summary = full.strip_prefix(&summary).unwrap_or("").trim().to_string();
            if after_summary.is_empty() {
                None
            } else {
                Some(after_summary)
            }
        };
        let author_name = commit.author().name().unwrap_or("Unknown").to_string();
        let time = commit.time();
        let secs = time.seconds();
        let date = chrono::DateTime::from_timestamp(secs, 0)
            .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
            .unwrap_or_else(|| "unknown date".to_string());

        entries.push(LogEntry {
            hash,
            short_hash,
            summary,
            body,
            author: author_name,
            date,
        });
    }

    Ok(entries)
}

/// Get log entries between two revspecs (inclusive of `from`, exclusive).
/// If `from` is `None`, tries to find the most recent tag as starting point.
/// If `to` is `None`, defaults to HEAD.
pub fn get_log_range(
    repo: &Repository,
    from: Option<&str>,
    to: Option<&str>,
) -> Result<Vec<LogEntry>> {
    let to_ref = to.unwrap_or("HEAD");
    let to_obj = repo
        .revparse_single(to_ref)
        .with_context(|| format!("Failed to resolve ref '{}'", to_ref))?;
    let to_oid = to_obj
        .peel_to_commit()
        .with_context(|| format!("'{}' does not point to a commit", to_ref))?
        .id();

    let from_oid = if let Some(from_ref) = from {
        let from_obj = repo
            .revparse_single(from_ref)
            .with_context(|| format!("Failed to resolve ref '{}'", from_ref))?;
        Some(
            from_obj
                .peel_to_commit()
                .with_context(|| format!("'{}' does not point to a commit", from_ref))?
                .id(),
        )
    } else {
        // Try to find the most recent tag
        find_latest_tag_oid(repo).ok()
    };

    let mut revwalk = repo.revwalk()?;
    revwalk.push(to_oid)?;
    if let Some(hide_oid) = from_oid {
        let _ = revwalk.hide(hide_oid); // ignore error if unreachable
    }
    revwalk.set_sorting(Sort::TIME)?;

    let mut entries = Vec::new();
    for oid_result in revwalk {
        let oid = oid_result?;
        let commit = repo.find_commit(oid)?;

        let hash = oid.to_string();
        let short_hash = hash[..7.min(hash.len())].to_string();
        let message = commit.message().unwrap_or("");
        let summary = commit.summary().unwrap_or("").to_string();
        let body = {
            let full = message.to_string();
            let after_summary = full.strip_prefix(&summary).unwrap_or("").trim().to_string();
            if after_summary.is_empty() {
                None
            } else {
                Some(after_summary)
            }
        };
        let author_name = commit.author().name().unwrap_or("Unknown").to_string();
        let time = commit.time();
        let secs = time.seconds();
        let date = chrono::DateTime::from_timestamp(secs, 0)
            .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
            .unwrap_or_else(|| "unknown date".to_string());

        entries.push(LogEntry {
            hash,
            short_hash,
            summary,
            body,
            author: author_name,
            date,
        });
    }

    Ok(entries)
}

/// Find the OID of the most recent tag reachable from HEAD.
fn find_latest_tag_oid(repo: &Repository) -> Result<git2::Oid> {
    let tag_names = repo.tag_names(None)?;
    let mut best: Option<(i64, git2::Oid)> = None;

    for name in tag_names.iter().flatten() {
        if let Ok(obj) = repo.revparse_single(name) {
            if let Ok(commit) = obj.peel_to_commit() {
                let time = commit.time().seconds();
                if best.as_ref().map_or(true, |(t, _)| time > *t) {
                    best = Some((time, commit.id()));
                }
            }
        }
    }

    best.map(|(_, oid)| oid)
        .context("No tags found in repository")
}

/// Create a commit on HEAD with the given message using the staged index.
///
/// Uses the repo's default signature (from git config `user.name` / `user.email`).
pub fn create_commit(repo: &Repository, message: &str) -> Result<git2::Oid> {
    let sig = repo
        .signature()
        .context("Failed to determine git signature. Set user.name and user.email in your git config.")?;

    let mut index = repo.index()?;
    let tree_oid = index.write_tree()?;
    let tree = repo.find_tree(tree_oid)?;

    let parent = match repo.head() {
        Ok(reference) => {
            let commit = reference.peel_to_commit()?;
            Some(commit)
        }
        Err(_) => None,
    };

    let parents: Vec<&git2::Commit> = parent.iter().collect();

    let oid = repo
        .commit(Some("HEAD"), &sig, &sig, message, &tree, &parents)
        .context("Failed to create commit")?;

    Ok(oid)
}

/// Get the number of commits on the current branch that are ahead of the
/// merge-base with the default branch (useful for squash).
#[allow(dead_code)]
pub fn count_commits_since_merge_base(
    repo: &Repository,
    base_branch: &str,
) -> Result<usize> {
    let head_oid = repo.head()?.peel_to_commit()?.id();
    let base_ref = format!("refs/heads/{}", base_branch);
    let base_obj = repo
        .revparse_single(&base_ref)
        .or_else(|_| repo.revparse_single(base_branch))?;
    let base_oid = base_obj.peel_to_commit()?.id();

    let merge_base = repo.merge_base(head_oid, base_oid)?;

    let mut revwalk = repo.revwalk()?;
    revwalk.push(head_oid)?;
    revwalk.hide(merge_base)?;
    revwalk.set_sorting(Sort::TIME)?;

    let count = revwalk.count();
    Ok(count)
}

/// Helper: convert a `git2::Diff` to a patch string.
fn diff_to_string(diff: &git2::Diff) -> Result<String> {
    let mut text = String::new();
    diff.print(DiffFormat::Patch, |_delta, _hunk, line| {
        let origin = line.origin();
        // Prefix with +/- for context
        match origin {
            '+' | '-' | ' ' => text.push(origin),
            _ => {}
        }
        if let Ok(content) = std::str::from_utf8(line.content()) {
            text.push_str(content);
        }
        true
    })
    .context("Failed to format diff")?;

    Ok(text)
}

/// Truncate a diff string to fit within a token budget.
///
/// If the diff exceeds `max_chars`, keeps the first and last portions with
/// a marker in between.
pub fn truncate_diff(diff: &str, max_chars: usize) -> String {
    if diff.len() <= max_chars {
        return diff.to_string();
    }

    let keep = max_chars / 2;
    let marker = "\n\n... [diff truncated for AI context window] ...\n\n";
    let start = &diff[..keep];
    let end = &diff[diff.len() - keep..];

    format!("{}{}{}", start, marker, end)
}
