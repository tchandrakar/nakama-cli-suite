//! Git diff operations using libgit2.
//!
//! Provides functions to obtain diffs between the current branch and
//! main/master, as well as diffs for individual files.

use anyhow::{Context, Result};
use git2::{DiffOptions, Repository};
use std::path::Path;

/// Information about the current branch and its diff against the base branch.
pub struct BranchDiff {
    /// Name of the current branch (e.g., "feature/my-change").
    pub branch_name: String,
    /// Name of the base branch that was used (e.g., "main" or "master").
    pub base_branch: String,
    /// The unified diff text.
    pub diff_text: String,
    /// Number of files changed.
    pub files_changed: usize,
    /// Total lines added.
    pub insertions: usize,
    /// Total lines deleted.
    pub deletions: usize,
}

/// Get the diff between the current HEAD and the main/master branch.
///
/// This discovers the repository from the current directory, finds the merge
/// base between HEAD and main (or master), and produces a unified diff.
pub fn get_branch_diff() -> Result<BranchDiff> {
    let repo = Repository::discover(".")
        .context("Not a git repository (or any parent). Run byakugan from within a git repo.")?;

    // Determine current branch name.
    let head = repo.head().context("Cannot read HEAD -- is this a valid git repository?")?;
    let branch_name = head
        .shorthand()
        .unwrap_or("HEAD")
        .to_string();

    let head_commit = head
        .peel_to_commit()
        .context("HEAD does not point to a commit")?;

    // Find the base branch: prefer "main", fall back to "master".
    let (base_branch_name, base_ref) = repo
        .find_branch("main", git2::BranchType::Local)
        .map(|b| ("main".to_string(), b))
        .or_else(|_| {
            repo.find_branch("master", git2::BranchType::Local)
                .map(|b| ("master".to_string(), b))
        })
        .context(
            "Could not find a 'main' or 'master' branch. \
             Make sure one exists locally.",
        )?;

    let base_commit = base_ref
        .get()
        .peel_to_commit()
        .context("Base branch does not point to a commit")?;

    // Find the merge base (common ancestor).
    let merge_base_oid = repo
        .merge_base(head_commit.id(), base_commit.id())
        .context("Could not find merge base between HEAD and the base branch")?;

    let base_tree = repo
        .find_commit(merge_base_oid)?
        .tree()
        .context("Could not read tree for merge base")?;

    let head_tree = head_commit
        .tree()
        .context("Could not read tree for HEAD")?;

    // Produce the diff.
    let diff = repo
        .diff_tree_to_tree(Some(&base_tree), Some(&head_tree), None)
        .context("Failed to compute diff")?;

    let stats = diff.stats().context("Failed to get diff stats")?;
    let files_changed = stats.files_changed();
    let insertions = stats.insertions();
    let deletions = stats.deletions();

    // Convert the diff to a unified text format.
    let diff_text = diff_to_text(&diff)?;

    Ok(BranchDiff {
        branch_name,
        base_branch: base_branch_name,
        diff_text,
        files_changed,
        insertions,
        deletions,
    })
}

/// Get the diff for a specific file between HEAD and the working tree
/// (unstaged + staged changes), or between HEAD and the base branch if the
/// file has no working-tree changes.
pub fn get_file_diff(file_path: &str) -> Result<String> {
    let repo = Repository::discover(".")
        .context("Not a git repository (or any parent).")?;

    let head = repo.head().context("Cannot read HEAD")?;
    let head_commit = head.peel_to_commit().context("HEAD is not a commit")?;
    let head_tree = head_commit.tree()?;

    // First, try diff of working directory against HEAD for this file.
    let mut opts = DiffOptions::new();
    opts.pathspec(file_path);

    let diff = repo
        .diff_tree_to_workdir_with_index(Some(&head_tree), Some(&mut opts))
        .context("Failed to diff working tree")?;

    let text = diff_to_text(&diff)?;

    if !text.trim().is_empty() {
        return Ok(text);
    }

    // If no working-tree changes, diff against the base branch.
    let (_, base_ref) = repo
        .find_branch("main", git2::BranchType::Local)
        .map(|b| ("main", b))
        .or_else(|_| {
            repo.find_branch("master", git2::BranchType::Local)
                .map(|b| ("master", b))
        })
        .context("Could not find 'main' or 'master' branch")?;

    let base_commit = base_ref.get().peel_to_commit()?;
    let merge_base_oid = repo.merge_base(head_commit.id(), base_commit.id())?;
    let base_tree = repo.find_commit(merge_base_oid)?.tree()?;

    let diff = repo
        .diff_tree_to_tree(Some(&base_tree), Some(&head_tree), Some(&mut opts))
        .context("Failed to diff file against base branch")?;

    let text = diff_to_text(&diff)?;

    if text.trim().is_empty() {
        anyhow::bail!(
            "No changes found for '{}'. The file may be unchanged relative to HEAD and the base branch.",
            file_path
        );
    }

    Ok(text)
}

/// Get the diff of all uncommitted changes (staged + unstaged) in the working tree.
pub fn get_working_diff() -> Result<String> {
    let repo = Repository::discover(".")
        .context("Not a git repository (or any parent).")?;

    let head = repo.head().context("Cannot read HEAD")?;
    let head_commit = head.peel_to_commit()?;
    let head_tree = head_commit.tree()?;

    let diff = repo
        .diff_tree_to_workdir_with_index(Some(&head_tree), None)
        .context("Failed to diff working tree")?;

    let text = diff_to_text(&diff)?;

    if text.trim().is_empty() {
        anyhow::bail!("No uncommitted changes found in the working tree.");
    }

    Ok(text)
}

/// Verify that a given path exists in the repository's working directory.
pub fn validate_file_path(file_path: &str) -> Result<()> {
    let repo = Repository::discover(".")?;
    let workdir = repo
        .workdir()
        .context("Repository has no working directory (bare repo?)")?;

    let full_path = workdir.join(file_path);
    if !full_path.exists() {
        // Also check if the path is absolute and exists.
        let as_path = Path::new(file_path);
        if !as_path.exists() {
            anyhow::bail!(
                "File '{}' does not exist in the repository.",
                file_path
            );
        }
    }

    Ok(())
}

/// Convert a git2::Diff into a unified diff string.
fn diff_to_text(diff: &git2::Diff) -> Result<String> {
    let mut output = String::new();

    diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| {
        let origin = line.origin();
        match origin {
            '+' | '-' | ' ' => output.push(origin),
            _ => {}
        }
        if let Ok(content) = std::str::from_utf8(line.content()) {
            output.push_str(content);
        }
        true
    })
    .context("Failed to format diff output")?;

    Ok(output)
}

