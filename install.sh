#!/bin/sh
# wow language installer for Linux and macOS
# Usage: curl -fsSL https://raw.githubusercontent.com/wow-language/wow/main/install.sh | sh
set -e

REPO="wow-language/wow"
BINARY="wow"

# ── Parse arguments ───────────────────────────────────────────────────────────
VERSION=""
while [ $# -gt 0 ]; do
    case "$1" in
        --version) VERSION="$2"; shift 2 ;;
        --version=*) VERSION="${1#*=}"; shift ;;
        *) shift ;;
    esac
done

# ── Detect latest version if not pinned ───────────────────────────────────────
if [ -z "$VERSION" ]; then
    printf "Fetching latest wow version...\n"
    VERSION=$(curl -fsSL "https://api.github.com/repos/$REPO/releases/latest" \
        | grep '"tag_name"' \
        | sed 's/.*"tag_name": *"\([^"]*\)".*/\1/')
    if [ -z "$VERSION" ]; then
        printf "Error: Could not fetch latest release. Check your network or visit:\n"
        printf "  https://github.com/%s/releases\n" "$REPO"
        exit 1
    fi
fi

printf "Installing wow %s...\n" "$VERSION"

# ── Detect OS ─────────────────────────────────────────────────────────────────
OS=$(uname -s)
case "$OS" in
    Linux)  OS_NAME="linux" ;;
    Darwin) OS_NAME="macos" ;;
    *)
        printf "Error: Unsupported OS: %s\n" "$OS"
        printf "Supported: Linux, macOS\n"
        printf "Download manually from: https://github.com/%s/releases\n" "$REPO"
        exit 1
        ;;
esac

# ── Detect architecture ───────────────────────────────────────────────────────
ARCH=$(uname -m)
case "$ARCH" in
    x86_64)          ARCH_NAME="x86_64" ;;
    aarch64|arm64)   ARCH_NAME="aarch64" ;;
    *)
        printf "Error: Unsupported architecture: %s\n" "$ARCH"
        printf "Supported: x86_64, aarch64 / arm64\n"
        printf "Download manually from: https://github.com/%s/releases\n" "$REPO"
        exit 1
        ;;
esac

# ── Build download URL ────────────────────────────────────────────────────────
ARCHIVE="wow-${VERSION}-${ARCH_NAME}-${OS_NAME}.tar.gz"
URL="https://github.com/$REPO/releases/download/$VERSION/$ARCHIVE"

printf "Downloading %s...\n" "$ARCHIVE"

# ── Download ──────────────────────────────────────────────────────────────────
TMP_DIR=$(mktemp -d)
trap 'rm -rf "$TMP_DIR"' EXIT

if command -v curl >/dev/null 2>&1; then
    curl -fsSL "$URL" -o "$TMP_DIR/$ARCHIVE" || {
        printf "Error: Download failed.\nURL: %s\n" "$URL"
        exit 1
    }
elif command -v wget >/dev/null 2>&1; then
    wget -qO "$TMP_DIR/$ARCHIVE" "$URL" || {
        printf "Error: Download failed.\nURL: %s\n" "$URL"
        exit 1
    }
else
    printf "Error: Neither curl nor wget found. Install one and retry, or download manually:\n"
    printf "  %s\n" "$URL"
    exit 1
fi

# ── Extract ───────────────────────────────────────────────────────────────────
tar -xzf "$TMP_DIR/$ARCHIVE" -C "$TMP_DIR"

# ── Remove macOS quarantine attribute ─────────────────────────────────────────
if [ "$OS_NAME" = "macos" ] && command -v xattr >/dev/null 2>&1; then
    xattr -d com.apple.quarantine "$TMP_DIR/wow" 2>/dev/null || true
fi

# ── Find install directory ────────────────────────────────────────────────────
USE_SUDO=0
if [ -w "/usr/local/bin" ]; then
    INSTALL_DIR="/usr/local/bin"
elif [ -d "/usr/local/bin" ]; then
    INSTALL_DIR="/usr/local/bin"
    USE_SUDO=1
else
    INSTALL_DIR="$HOME/.local/bin"
    mkdir -p "$INSTALL_DIR"
fi

# ── Install ───────────────────────────────────────────────────────────────────
if [ "$USE_SUDO" = "1" ]; then
    printf "Installing to %s (requires sudo)...\n" "$INSTALL_DIR"
    sudo install -m 755 "$TMP_DIR/wow" "$INSTALL_DIR/wow"
else
    install -m 755 "$TMP_DIR/wow" "$INSTALL_DIR/wow"
fi

# ── Verify ────────────────────────────────────────────────────────────────────
if "$INSTALL_DIR/wow" --version >/dev/null 2>&1; then
    printf "\n  wow %s installed successfully!\n" "$VERSION"
    printf "  Run: wow run myprog.wow\n\n"
else
    printf "Warning: Installed but 'wow --version' did not succeed. Check %s/wow\n" "$INSTALL_DIR"
fi

# ── PATH hint if needed ───────────────────────────────────────────────────────
if [ "$INSTALL_DIR" = "$HOME/.local/bin" ]; then
    case ":$PATH:" in
        *":$HOME/.local/bin:"*) ;;
        *)
            printf "  NOTE: Add %s to your PATH:\n" "$INSTALL_DIR"
            printf "    echo 'export PATH=\"\$HOME/.local/bin:\$PATH\"' >> ~/.bashrc\n"
            printf "    source ~/.bashrc\n"
            printf "\n  (For zsh: replace ~/.bashrc with ~/.zshrc)\n\n"
            ;;
    esac
fi
