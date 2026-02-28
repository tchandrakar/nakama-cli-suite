//! `shinigami hook` — Manage git hooks for shinigami integration.
//!
//! Supports installing, removing, and listing shinigami-managed git hooks
//! (e.g. a `prepare-commit-msg` hook that auto-generates commit messages).

use crate::git;
use anyhow::{bail, Context, Result};
use nakama_ui::NakamaUI;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

const HOOK_MARKER: &str = "# managed by shinigami";

const PREPARE_COMMIT_MSG_HOOK: &str = r#"#!/bin/sh
# managed by shinigami
# This hook runs shinigami to suggest a commit message.
# If shinigami is not installed or fails, the commit proceeds normally.

COMMIT_MSG_FILE="$1"
COMMIT_SOURCE="$2"

# Only run for regular commits (not merges, squashes, etc.)
if [ -z "$COMMIT_SOURCE" ]; then
    if command -v shinigami >/dev/null 2>&1; then
        echo "[shinigami] Generating commit message..."
        # shinigami would write to the file; for now this is a placeholder
    fi
fi
"#;

/// Run the hook subcommand.
pub fn run(ui: &NakamaUI, action: &str) -> Result<()> {
    let repo = git::open_repo()?;
    let hooks_dir = get_hooks_dir(&repo)?;

    match action {
        "install" => install_hooks(ui, &hooks_dir),
        "remove" | "uninstall" => remove_hooks(ui, &hooks_dir),
        "list" | "ls" => list_hooks(ui, &hooks_dir),
        other => {
            bail!(
                "Unknown hook action '{}'. Use: install, remove, or list.",
                other
            );
        }
    }
}

fn get_hooks_dir(repo: &git2::Repository) -> Result<PathBuf> {
    let git_dir = repo.path(); // .git directory
    let hooks = git_dir.join("hooks");
    if !hooks.exists() {
        fs::create_dir_all(&hooks).context("Failed to create hooks directory")?;
    }
    Ok(hooks)
}

fn install_hooks(ui: &NakamaUI, hooks_dir: &PathBuf) -> Result<()> {
    let hook_path = hooks_dir.join("prepare-commit-msg");

    if hook_path.exists() {
        let contents = fs::read_to_string(&hook_path)?;
        if contents.contains(HOOK_MARKER) {
            ui.warn("Shinigami hook is already installed.");
            return Ok(());
        }
        // There's an existing hook not managed by us
        ui.warn("A prepare-commit-msg hook already exists. Skipping to avoid overwriting.");
        ui.info("Remove it first or use 'shinigami hook remove' if it was installed by shinigami.");
        return Ok(());
    }

    fs::write(&hook_path, PREPARE_COMMIT_MSG_HOOK)?;

    // Make executable
    let mut perms = fs::metadata(&hook_path)?.permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&hook_path, perms)?;

    ui.success("Installed prepare-commit-msg hook");
    Ok(())
}

fn remove_hooks(ui: &NakamaUI, hooks_dir: &PathBuf) -> Result<()> {
    let hook_path = hooks_dir.join("prepare-commit-msg");

    if !hook_path.exists() {
        ui.warn("No prepare-commit-msg hook found.");
        return Ok(());
    }

    let contents = fs::read_to_string(&hook_path)?;
    if !contents.contains(HOOK_MARKER) {
        ui.warn("The existing prepare-commit-msg hook was not installed by shinigami. Not removing.");
        return Ok(());
    }

    fs::remove_file(&hook_path)?;
    ui.success("Removed prepare-commit-msg hook");
    Ok(())
}

fn list_hooks(ui: &NakamaUI, hooks_dir: &PathBuf) -> Result<()> {
    let known_hooks = [
        "prepare-commit-msg",
        "pre-commit",
        "commit-msg",
        "post-commit",
        "pre-push",
    ];

    let mut rows: Vec<Vec<String>> = Vec::new();

    for hook_name in &known_hooks {
        let path = hooks_dir.join(hook_name);
        if path.exists() {
            let contents = fs::read_to_string(&path).unwrap_or_default();
            let managed = if contents.contains(HOOK_MARKER) {
                "shinigami"
            } else {
                "other"
            };
            rows.push(vec![
                hook_name.to_string(),
                "installed".to_string(),
                managed.to_string(),
            ]);
        } else {
            rows.push(vec![
                hook_name.to_string(),
                "not installed".to_string(),
                "-".to_string(),
            ]);
        }
    }

    ui.table(&["Hook", "Status", "Managed By"], rows);
    Ok(())
}
