#!/usr/bin/env bash
#
# Nakama CLI Suite — Binary Installer (from GitHub Releases)
# Copyright (c) 2026 Tishant Chandrakar
#
# Downloads pre-built binaries from the latest GitHub release and
# installs them to ~/.cargo/bin/. No Rust toolchain required.
#
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/tchandrakar/nakama-cli-suite/main/install-release.sh | bash
#

set -euo pipefail

BOLD='\033[1m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
RED='\033[0;31m'
PURPLE='\033[0;35m'
NC='\033[0m'

REPO="tchandrakar/nakama-cli-suite"
INSTALL_DIR="${HOME}/.cargo/bin"

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
echo "  │   Nakama CLI Suite — Release Installer   │"
echo "  │   Your anime-inspired dev crew           │"
echo "  │                                          │"
echo "  │   Made by Tishant Chandrakar             │"
echo "  └─────────────────────────────────────────┘"
echo -e "${NC}"

# Detect OS and architecture
detect_platform() {
    local os arch

    os="$(uname -s)"
    arch="$(uname -m)"

    case "$os" in
        Linux)
            case "$arch" in
                x86_64) echo "linux-x86_64" ;;
                *)
                    echo -e "${RED}Error: Unsupported Linux architecture: ${arch}${NC}" >&2
                    echo "Supported: x86_64" >&2
                    exit 1
                    ;;
            esac
            ;;
        Darwin)
            case "$arch" in
                x86_64) echo "darwin-x86_64" ;;
                arm64)  echo "darwin-aarch64" ;;
                *)
                    echo -e "${RED}Error: Unsupported macOS architecture: ${arch}${NC}" >&2
                    exit 1
                    ;;
            esac
            ;;
        *)
            echo -e "${RED}Error: Unsupported OS: ${os}${NC}" >&2
            echo "Supported: Linux, macOS" >&2
            exit 1
            ;;
    esac
}

PLATFORM="$(detect_platform)"
echo -e "${GREEN}●${NC} Detected platform: ${BOLD}${PLATFORM}${NC}"

# Fetch the latest release tag from GitHub API
echo -e "${YELLOW}◐${NC} Fetching latest release..."
LATEST_TAG=$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name"' | sed -E 's/.*"tag_name": *"([^"]+)".*/\1/')

if [ -z "$LATEST_TAG" ]; then
    echo -e "${RED}Error: Could not determine latest release.${NC}"
    exit 1
fi

echo -e "${GREEN}●${NC} Latest release: ${BOLD}${LATEST_TAG}${NC}"

# Download tarball
TARBALL="nakama-${PLATFORM}.tar.gz"
DOWNLOAD_URL="https://github.com/${REPO}/releases/download/${LATEST_TAG}/${TARBALL}"

TMPDIR="$(mktemp -d)"
trap 'rm -rf "$TMPDIR"' EXIT

echo -e "${YELLOW}◐${NC} Downloading ${TARBALL}..."
curl -fsSL -o "${TMPDIR}/${TARBALL}" "$DOWNLOAD_URL"

# Verify checksum if available
CHECKSUM_URL="${DOWNLOAD_URL}.sha256"
if curl -fsSL -o "${TMPDIR}/${TARBALL}.sha256" "$CHECKSUM_URL" 2>/dev/null; then
    echo -e "${YELLOW}◐${NC} Verifying checksum..."
    cd "$TMPDIR"
    if command -v sha256sum &>/dev/null; then
        sha256sum -c "${TARBALL}.sha256"
    elif command -v shasum &>/dev/null; then
        shasum -a 256 -c "${TARBALL}.sha256"
    fi
    cd - >/dev/null
fi

# Extract
echo -e "${YELLOW}◐${NC} Extracting..."
tar xzf "${TMPDIR}/${TARBALL}" -C "$TMPDIR"

# Install
mkdir -p "$INSTALL_DIR"

INSTALLED=0
for tool in "${TOOLS[@]}"; do
    BINARY="${TMPDIR}/nakama-${PLATFORM}/${tool}"
    if [ -f "$BINARY" ]; then
        cp "$BINARY" "${INSTALL_DIR}/${tool}"
        chmod 755 "${INSTALL_DIR}/${tool}"
        echo -e "  ${GREEN}●${NC} Installed: ${BOLD}${tool}${NC}"
        INSTALLED=$((INSTALLED + 1))
    else
        echo -e "  ${YELLOW}○${NC} Skipped: ${tool} (not found in release)"
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

# Check PATH
if [[ ":$PATH:" != *":${INSTALL_DIR}:"* ]]; then
    echo ""
    echo -e "${YELLOW}Warning: ${INSTALL_DIR} is not in your PATH.${NC}"
    echo -e "  Add it with: export PATH=\"\$HOME/.cargo/bin:\$PATH\""
fi

echo ""
echo -e "${GREEN}${BOLD}Installation complete!${NC} (${LATEST_TAG})"
echo -e "  ${INSTALLED}/${#TOOLS[@]} tools installed to ${INSTALL_DIR}/"
echo ""
echo -e "  ${BOLD}Quick start:${NC}"
echo -e "    zangetsu ask \"find large files\"    # Shell companion"
echo -e "    shinigami commit                    # AI git commits"
echo -e "    byakugan review                     # PR review"
echo -e "    kami search \"kubernetes tips\"       # Web search"
echo ""
echo -e "  ${BOLD}Configure:${NC}"
echo -e "    Edit ~/.nakama/config.toml to set your AI provider"
echo ""
echo -e "  ${PURPLE}Nakama CLI Suite — Made by Tishant Chandrakar${NC}"
