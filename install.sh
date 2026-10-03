#!/usr/bin/env sh
set -e

# ==============================================================================
#  fzfx Installer for Linux
# ==============================================================================

REPO="aether-flux/fzfx"

# Color Codes
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
YELLOW='\033[1;33m'
BOLD='\033[1m'
NC='\033[0m' # No Color

# Print ASCII Art Header
print_header() {
    echo -e "${CYAN}${BOLD}"
    cat << "EOF"
     ___          ___       
    / __)        / __)      
   | |__ _____ _| |___ _   _
  (_   __|___  |_   __| \ / )
    | |   / __/  | |   ) X ( 
    |_|  (_____) |_|  (_/ \_)

  Fast, Semantic CLI Finder
EOF
    echo -e "${NC}"
}

info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1"
    exit 1
}

# OS Check (Linux Only)
check_os() {
    OS="$(uname -s)"
    if [ "$OS" != "Linux" ]; then
        error "This install script currently supports Linux only. OS detected: $OS"
    fi
}

# Architecture Detection
detect_arch() {
    ARCH="$(uname -m)"
    case "$ARCH" in
        x86_64)
            TARGET="x86_64-unknown-linux-gnu"
            ;;
        # aarch64|arm64)
        #     TARGET="aarch64-unknown-linux-gnu"
        #     ;;
        *)
            error "Unsupported architecture: $ARCH"
            ;;
    esac
}

# Determine Installation Path
determine_install_dir() {
    if [ -w "/usr/local/bin" ]; then
        INSTALL_DIR="/usr/local/bin"
    else
        INSTALL_DIR="$HOME/.local/bin"
        mkdir -p "$INSTALL_DIR"
    fi
}

# Helper: HTTP fetch wrapper
fetch_data() {
    url="$1"
    output="$2"
    if command -v curl >/dev/null 2>&1; then
        if [ -n "$output" ]; then
            curl -fsSL "$url" -o "$output"
        else
            curl -fsSL "$url"
        fi
    elif command -v wget >/dev/null 2>&1; then
        if [ -n "$output" ]; then
            wget -qO "$output" "$url"
        else
            wget -qO- "$url"
        fi
    else
        error "Neither 'curl' nor 'wget' was found. Please install one of them."
    fi
}

# Fetch Latest Tag via GitHub API
get_latest_tag() {
    info "Resolving latest release tag from GitHub API..."
    API_URL="https://api.github.com/repos/$REPO/releases/latest"
    
    JSON_RESPONSE="$(fetch_data "$API_URL" "")"
    LATEST_TAG="$(echo "$JSON_RESPONSE" | grep -o '"tag_name": "[^"]*' | head -n 1 | sed 's/"tag_name": "//')"

    if [ -z "$LATEST_TAG" ]; then
        error "Failed to retrieve the latest release tag from GitHub API."
    fi

    info "Latest release version: ${BOLD}$LATEST_TAG${NC}"
}

# Main Download & Install Flow
main() {
    print_header
    check_os
    detect_arch
    determine_install_dir
    get_latest_tag

    BINARY_NAME="fzfx-$TARGET"
    DOWNLOAD_URL="https://github.com/$REPO/releases/download/$LATEST_TAG/$BINARY_NAME"
    TEMP_DIR="$(mktemp -d)"

    cleanup() {
        rm -rf "$TEMP_DIR"
    }
    trap cleanup EXIT

    info "Downloading binary: ${BOLD}$DOWNLOAD_URL${NC}..."
    fetch_data "$DOWNLOAD_URL" "$TEMP_DIR/fzfx" || error "Failed to download $BINARY_NAME from release $LATEST_TAG."

    info "Installing fzfx binary to ${BOLD}$INSTALL_DIR/fzfx${NC}..."
    chmod +x "$TEMP_DIR/fzfx"

    if [ -w "$INSTALL_DIR" ]; then
        mv "$TEMP_DIR/fzfx" "$INSTALL_DIR/fzfx"
    else
        warn "Write permission required for $INSTALL_DIR, running with sudo..."
        sudo mv "$TEMP_DIR/fzfx" "$INSTALL_DIR/fzfx"
    fi

    success "fzfx has been installed successfully to ${BOLD}$INSTALL_DIR/fzfx${NC}!"

    # PATH Check Warning
    case ":$PATH:" in
        *":$INSTALL_DIR:"*) ;;
        *)
            echo ""
            warn "Note: ${BOLD}$INSTALL_DIR${NC} is not currently in your \$PATH."
            warn "Add it to your shell config (~/.bashrc or ~/.zshrc):"
            echo -e "  ${CYAN}export PATH=\"\$HOME/.local/bin:\$PATH\"${NC}"
            ;;
    esac
}

main "$@"
