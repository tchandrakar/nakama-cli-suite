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
# AI Provider: anthropic | openai | google | ollama
[ai]
default_provider = "anthropic"

[logging]
level = "info"

[ui]
color = "auto"
verbosity = "normal"

[audit]
enabled = true
retention_days = 90
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
echo -e "    Run: nakama auth add --service anthropic --key YOUR_KEY"
echo ""
echo -e "  ${PURPLE}Nakama CLI Suite — Made by Tishant Chandrakar${NC}"
