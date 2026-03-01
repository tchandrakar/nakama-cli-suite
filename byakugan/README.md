# Byakugan

> AI-powered multi-pass PR and code reviewer — part of the [Nakama CLI Suite](../README.md).

Byakugan runs 5 specialized AI review passes (Security, Performance, Style, Logic, Summary) against any PR or local diff, then optionally posts a comprehensive review comment directly to the platform.

## Usage

```bash
# Review current branch against main/master
byakugan review

# Review a PR by URL (auto-detects platform)
byakugan pr "https://bitbucket.org/workspace/repo/pull-requests/123"

# Review a PR and post results as a comment
byakugan pr "https://bitbucket.org/workspace/repo/pull-requests/123" --post

# Review a PR by number (requires git remote or --owner/--repo)
byakugan pr 42 --post

# Review a specific file's changes
byakugan diff src/main.rs

# Suggest improvements for current changes
byakugan suggest

# Run custom rules only (no AI)
byakugan scan

# Combined AI review + rule scan
byakugan report

# Post a comment to a PR
byakugan comment 42 "Looks good!"

# Watch for new PRs and auto-review
byakugan watch
```

### Global Options

```
--provider <PROVIDER>   AI provider: anthropic, openai, google, ollama (default: anthropic)
--tier <TIER>           Model tier: fast, balanced, powerful (default: balanced)
--format <FORMAT>       Output format: terminal, json, markdown (default: terminal)
--platform <PLATFORM>   Platform: github, gitlab, bitbucket (auto-detected if omitted)
--owner <OWNER>         Repository owner (auto-detected from git remote)
--repo <REPO>           Repository name (auto-detected from git remote)
```

### Posting Reviews

Use `--post` on the `pr` subcommand to post a formatted review comment:

```bash
byakugan pr "https://github.com/org/repo/pull/42" --post
```

The posted comment includes:
- Header with byakugan version, AI model, and PR context
- Summary table showing each pass's findings and severity
- Overall verdict (Looks Good / Minor Concerns / Needs Changes)
- Detailed findings per pass (passes with no issues are omitted)
- Token usage footer

If max severity is HIGH or CRITICAL, the review verdict is set to "Request Changes".

## Configuration

All configuration lives in `~/.nakama/config.toml`.

### Auto-Post Reviews

Skip the `--post` flag by enabling auto-post:

```toml
[byakugan]
auto_post_comments = true
```

### Custom Review Passes

Control which passes run:

```toml
[byakugan]
passes = ["security", "performance", "style", "logic", "summary"]
```

### Configurable Prompts

Override the AI system prompt for any pass, or add a global preamble that gets prepended to all prompts:

```toml
[byakugan.prompts]
preamble = "This is a Java Spring Boot project. Focus on Spring-specific patterns and best practices."

# Override individual pass prompts (optional — hardcoded defaults used if omitted):
# security = "Custom security review instructions..."
# performance = "Custom performance review instructions..."
# style = "Custom style review instructions..."
# logic = "Custom logic review instructions..."
# summary = "Custom summary review instructions..."
```

The `preamble` is prepended to whichever prompt is used (custom or default), making it useful for project-specific context without rewriting entire prompts.

### Custom Rules

Define regex-based rules that run without AI (via `byakugan scan`):

```toml
[[byakugan.rules]]
name = "No TODO comments"
description = "TODOs should be tracked as issues"
severity = "low"
pattern = "TODO|FIXME|HACK"
exclude = ["*.md", "CHANGELOG*"]
```

### Platform Tokens

Byakugan needs platform tokens to fetch PR diffs and post reviews. See the [main README](../README.md#2-store-api-keys) for setup.

## Supported Platforms

| Platform   | Fetch PR | Post Review |
|------------|----------|-------------|
| GitHub     | Yes      | Yes         |
| GitLab     | Yes      | Yes         |
| Bitbucket  | Yes      | Yes         |

Platform is auto-detected from the git remote URL, or can be specified with `--platform`.
