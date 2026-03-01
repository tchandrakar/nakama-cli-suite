#!/usr/bin/env bash
#
# Nakama CLI Suite — Installer
# Copyright (c) 2026 Tishant Chandrakar
#
# Installs all Nakama CLI tools to ~/.cargo/bin/
#

set -euo pipefail

# Source cargo environment if available
if [ -f "${HOME}/.cargo/env" ]; then
    # shellcheck source=/dev/null
    . "${HOME}/.cargo/env"
fi

BOLD='\033[1m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
RED='\033[0;31m'
PURPLE='\033[0;35m'
NC='\033[0m'

TOOLS=(
    zangetsu
    shinigami
    jogan
    senku
    sharingan
    tensai
    mugen
    gate
    byakugan
    kami
    itachi
)

echo -e "${PURPLE}${BOLD}"
echo "  ┌─────────────────────────────────────────┐"
echo "  │     Nakama CLI Suite — Installer         │"
echo "  │     Your anime-inspired dev crew         │"
echo "  │                                          │"
echo "  │     Made by Tishant Chandrakar           │"
echo "  └─────────────────────────────────────────┘"
echo -e "${NC}"

# Check Rust toolchain
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Error: Rust/Cargo not found.${NC}"
    echo "Install Rust first: https://rustup.rs"
    exit 1
fi

RUST_VERSION=$(rustc --version | awk '{print $2}')
echo -e "${GREEN}●${NC} Rust toolchain: ${RUST_VERSION}"

# Check minimum Rust version (1.75)
MIN_VERSION="1.75.0"
if [ "$(printf '%s\n' "$MIN_VERSION" "$RUST_VERSION" | sort -V | head -n1)" != "$MIN_VERSION" ]; then
    echo -e "${RED}Error: Rust >= ${MIN_VERSION} required (found ${RUST_VERSION})${NC}"
    echo "Run: rustup update"
    exit 1
fi

# Build all crates in release mode
echo ""
echo -e "${YELLOW}◐${NC} Building all tools (release mode)..."
echo "  This may take a few minutes on first build."
echo ""

cargo build --release --workspace 2>&1 | while IFS= read -r line; do
    if [[ "$line" == *"Compiling"* ]]; then
        echo -e "  ${GREEN}●${NC} $line"
    elif [[ "$line" == *"error"* ]]; then
        echo -e "  ${RED}✕${NC} $line"
    fi
done

if [ ${PIPESTATUS[0]} -ne 0 ]; then
    echo -e "${RED}Build failed. See errors above.${NC}"
    exit 1
fi

# Install binaries
echo ""
echo -e "${YELLOW}◐${NC} Installing tools to ~/.cargo/bin/..."

INSTALLED=0
for tool in "${TOOLS[@]}"; do
    BINARY="target/release/${tool}"
    if [ -f "$BINARY" ]; then
        cp "$BINARY" "${HOME}/.cargo/bin/${tool}"
        chmod 755 "${HOME}/.cargo/bin/${tool}"
        echo -e "  ${GREEN}●${NC} Installed: ${BOLD}${tool}${NC}"
        INSTALLED=$((INSTALLED + 1))
    else
        echo -e "  ${YELLOW}○${NC} Skipped: ${tool} (not built)"
    fi
done

# Create default config directory
mkdir -p "${HOME}/.nakama"
chmod 700 "${HOME}/.nakama"

# Create default config if it doesn't exist
if [ ! -f "${HOME}/.nakama/config.toml" ]; then
    cat > "${HOME}/.nakama/config.toml" << 'TOML'
# Nakama CLI Suite — Global Configuration
# Made by Tishant Chandrakar
#
# Full reference: https://github.com/tchandrakar/nakama-cli-suite#configuration

# ---------------------------------------------------------------------------
# AI Provider (anthropic | openai | google | ollama)
# ---------------------------------------------------------------------------
[ai]
default_provider = "anthropic"

# [ai.anthropic]
# model_fast = "claude-haiku-4-5-20251001"
# model_balanced = "claude-sonnet-4-6"
# model_powerful = "claude-opus-4-6"
# base_url = "https://api.anthropic.com"    # override for proxy

# [ai.openai]
# model_fast = "gpt-4.1-nano"
# model_balanced = "gpt-4.1-mini"
# model_powerful = "gpt-4.1"

# [ai.google]
# model_fast = "gemini-2.5-flash"
# model_balanced = "gemini-2.5-flash"
# model_powerful = "gemini-2.5-pro"

# [ai.ollama]
# base_url = "http://localhost:11434"
# model_fast = "llama3:8b"
# model_balanced = "llama3:70b"

# [ai.retry]
# max_retries = 3
# initial_backoff_ms = 1000
# max_backoff_ms = 30000

# Spending limits (optional)
# [ai.budget]
# weekly_limit_usd = 10.00
# alert_threshold_percent = 80
# hard_limit = true

# ---------------------------------------------------------------------------
# Platform tokens (for byakugan PR review, shinigami, etc.)
# Alternatively, use env vars: NAKAMA_GITHUB_API_KEY, NAKAMA_BITBUCKET_API_KEY
# ---------------------------------------------------------------------------
# [platforms.github]
# token = "ghp_..."
# api_url = "https://api.github.com"

# [platforms.gitlab]
# token = "glpat-..."
# api_url = "https://gitlab.com/api/v4"

# [platforms.bitbucket]
# username = "your-username"
# app_password = "your-app-password"
# api_url = "https://api.bitbucket.org/2.0"

# ---------------------------------------------------------------------------
# Byakugan (AI PR reviewer)
# ---------------------------------------------------------------------------
# [byakugan]
# passes = ["security", "performance", "style", "logic", "summary"]
# max_comments = 25
# severity_threshold = "low"
# auto_post_comments = false

# Override AI prompts per pass, or add a project-specific preamble
# [byakugan.prompts]
# preamble = "This is a Java Spring Boot project. Focus on Spring patterns."
# security = "Custom security review prompt..."
# performance = "Custom performance review prompt..."
# style = "Custom style review prompt..."
# logic = "Custom logic review prompt..."
# summary = "Custom summary review prompt..."

# Custom regex rules (run via `byakugan scan`, no AI needed)
# [[byakugan.rules]]
# name = "No TODO comments"
# description = "TODOs should be tracked as issues"
# severity = "low"
# pattern = "TODO|FIXME|HACK"
# exclude = ["*.md"]

# [byakugan.watch]
# poll_interval_seconds = 300
# auto_review = false
# notify = true
# repos = []

# ---------------------------------------------------------------------------
# Logging, UI, Audit, IPC, Updates
# ---------------------------------------------------------------------------
[logging]
level = "info"
# format = "json"
# directory = "~/.nakama/logs"

[ui]
color = "auto"
verbosity = "normal"
# spinners = true
# unicode = true

[audit]
enabled = true
retention_days = 90
# chain_verification = true

# [ipc]
# schema_validation = true
# trace_propagation = true

[updates]
enabled = true
check_interval_hours = 24
TOML
    chmod 600 "${HOME}/.nakama/config.toml"
    echo ""
    echo -e "${GREEN}●${NC} Created default config: ~/.nakama/config.toml"
fi

# Create subdirectories
mkdir -p "${HOME}/.nakama/logs" "${HOME}/.nakama/audit" "${HOME}/.nakama/vault"
chmod 700 "${HOME}/.nakama/logs" "${HOME}/.nakama/audit" "${HOME}/.nakama/vault"

echo ""
echo -e "${GREEN}${BOLD}Installation complete!${NC}"
echo -e "  ${INSTALLED}/${#TOOLS[@]} tools installed to ~/.cargo/bin/"
echo ""
echo -e "  ${BOLD}Quick start:${NC}"
echo -e "    zangetsu ask \"find large files\"    # Shell companion"
echo -e "    shinigami commit                    # AI git commits"
echo -e "    byakugan review                     # PR review"
echo -e "    kami search \"kubernetes tips\"       # Web search"
echo ""
echo -e "  ${BOLD}Configure:${NC}"
echo -e "    Edit ~/.nakama/config.toml to set your AI provider"
echo -e "    Set your API key: export NAKAMA_ANTHROPIC_API_KEY=YOUR_KEY"
echo ""
echo -e "  ${PURPLE}Nakama CLI Suite — Made by Tishant Chandrakar${NC}"
