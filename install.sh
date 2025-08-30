#!/usr/bin/env bash
# UTF - Unified Test Framework Installation Script
# Usage: curl -fsSL https://raw.githubusercontent.com/samirparhi-dev/unified-test-framework/main/install.sh | bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Emojis
ROCKET="🚀"
CHECK="✅"
WARNING="⚠️"
ERROR="❌"
INFO="💡"
DOWNLOAD="📥"
CONFIG="⚙️"

# Default values
UTF_INSTALL_DIR="${UTF_INSTALL_DIR:-$HOME/.local/bin}"
UTF_CONFIG_DIR="$HOME/.config/utf"
UTF_VERSION="${UTF_VERSION:-latest}"
UTF_REPO="samirparhi-dev/unified-test-framework"
BINARY_NAME="utf"
GITHUB_API_BASE="https://api.github.com/repos"
GITHUB_RELEASE_BASE="https://github.com"

# Platform detection
detect_platform() {
    local os arch
    
    case "$(uname -s)" in
        Darwin*)
            os="apple-darwin"
            ;;
        Linux*)
            os="unknown-linux-gnu"
            ;;
        CYGWIN*|MINGW*|MSYS*)
            os="pc-windows-msvc"
            ;;
        *)
            echo "${ERROR} Unsupported operating system: $(uname -s)"
            exit 1
            ;;
    esac
    
    case "$(uname -m)" in
        x86_64|amd64)
            arch="x86_64"
            ;;
        aarch64|arm64)
            arch="aarch64"
            ;;
        armv7l)
            arch="armv7"
            ;;
        *)
            echo "${ERROR} Unsupported architecture: $(uname -m)"
            exit 1
            ;;
    esac
    
    echo "${arch}-${os}"
}

# Download function with retry
download_with_retry() {
    local url="$1"
    local output="$2"
    local max_attempts=3
    local attempt=1
    
    while [ $attempt -le $max_attempts ]; do
        echo "  ${INFO} Download attempt $attempt/$max_attempts..."
        
        if command -v curl >/dev/null 2>&1; then
            if curl -sSfL "$url" -o "$output"; then
                return 0
            fi
        elif command -v wget >/dev/null 2>&1; then
            if wget -q "$url" -O "$output"; then
                return 0
            fi
        else
            echo "${ERROR} Neither curl nor wget is available"
            exit 1
        fi
        
        attempt=$((attempt + 1))
        sleep 2
    done
    
    echo "${ERROR} Failed to download after $max_attempts attempts"
    exit 1
}

# Get latest release version
get_latest_version() {
    if [ "$UTF_VERSION" = "latest" ]; then
        local api_url="${GITHUB_API_BASE}/${UTF_REPO}/releases/latest"
        
        if command -v curl >/dev/null 2>&1; then
            curl -sSfL "$api_url" | grep '"tag_name":' | sed -E 's/.*"v?([^"]+)".*/\1/' | head -n1
        elif command -v wget >/dev/null 2>&1; then
            wget -qO- "$api_url" | grep '"tag_name":' | sed -E 's/.*"v?([^"]+)".*/\1/' | head -n1
        else
            echo "0.1.0"  # fallback version
        fi
    else
        echo "$UTF_VERSION"
    fi
}

# Install Rust and Cargo if not present
install_rust() {
    if ! command -v cargo >/dev/null 2>&1; then
        echo "${INFO} Rust not found. Installing Rust..."
        
        # Install rustup
        if command -v curl >/dev/null 2>&1; then
            curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        else
            echo "${ERROR} curl is required to install Rust"
            exit 1
        fi
        
        # Source the cargo environment
        source "$HOME/.cargo/env"
        
        echo "${CHECK} Rust installed successfully"
    else
        echo "${CHECK} Rust already installed"
    fi
}

# Build from source as fallback
build_from_source() {
    echo "${INFO} Building from source..."
    
    local temp_dir
    temp_dir=$(mktemp -d)
    
    cd "$temp_dir"
    
    # Clone repository
    if command -v git >/dev/null 2>&1; then
        git clone "https://github.com/${UTF_REPO}.git" .
    else
        echo "${ERROR} git is required to build from source"
        exit 1
    fi
    
    # Build and install
    cargo install --path . --root "$HOME/.local"
    
    # Cleanup
    cd - >/dev/null
    rm -rf "$temp_dir"
    
    echo "${CHECK} Built and installed from source"
}

# Install language configurations
install_language_configs() {
    echo "${CONFIG} Installing language configurations..."
    
    mkdir -p "$UTF_CONFIG_DIR/language_configs"
    
    # Download language configs
    local configs=("swift.json" "kotlin.json" "csharp.json" "php.json")
    
    for config in "${configs[@]}"; do
        local config_url="https://raw.githubusercontent.com/${UTF_REPO}/main/language_configs/${config}"
        download_with_retry "$config_url" "$UTF_CONFIG_DIR/language_configs/$config" || echo "${WARNING} Failed to download $config (optional)"
    done
    
    echo "${CHECK} Language configurations installed"
}

# Configure shell integration
configure_shell() {
    echo "${CONFIG} Configuring shell integration..."
    
    local shell_config
    local path_export
    local bin_dir="$HOME/.local/bin"
    
    # Ensure bin directory exists
    mkdir -p "$bin_dir"
    
    # Detect shell and config file
    if [ -n "$ZSH_VERSION" ] || [[ "$SHELL" == *"zsh"* ]]; then
        shell_config="$HOME/.zshrc"
        path_export="export PATH=\"\$PATH:$bin_dir\""
    elif [[ "$SHELL" == *"fish"* ]]; then
        shell_config="$HOME/.config/fish/config.fish"
        mkdir -p "$(dirname "$shell_config")"
        path_export="set -gx PATH \$PATH $bin_dir"
    else
        shell_config="$HOME/.bashrc"
        path_export="export PATH=\"\$PATH:$bin_dir\""
    fi
    
    # Add to PATH if not already there
    if [ -f "$shell_config" ] && grep -q "$bin_dir" "$shell_config"; then
        echo "${CHECK} Shell already configured"
    else
        echo "" >> "$shell_config"
        echo "# Added by Unified Test Framework installer" >> "$shell_config"
        echo "$path_export" >> "$shell_config"
        echo "${CHECK} Shell configuration updated: $shell_config"
    fi
}

# Main installation function
main() {
    echo "${ROCKET} ${CYAN}Unified Test Framework Installer${NC}"
    echo "======================================"
    echo ""
    
    # Check if already installed
    if command -v utf >/dev/null 2>&1; then
        echo "${CHECK} utf is already installed at $(which utf)"
        echo "${INFO} To reinstall, run: ${YELLOW}UTF_FORCE=1 curl -fsSL https://raw.githubusercontent.com/samirparhi-dev/unified-test-framework/main/install.sh | bash${NC}"
        
        if [ "${UTF_FORCE:-}" != "1" ]; then
            exit 0
        fi
        echo "${WARNING} Force reinstalling..."
        echo ""
    fi
    
    # Detect platform
    echo "${INFO} Detecting platform..."
    local platform
    platform=$(detect_platform)
    echo "${CHECK} Platform: $platform"
    echo ""
    
    # Get version
    echo "${INFO} Getting latest version..."
    local version
    version=$(get_latest_version)
    echo "${CHECK} Version: $version"
    echo ""
    
    # Create installation directory
    mkdir -p "$UTF_INSTALL_DIR"
    
    # Try to download prebuilt binary first
    echo "${DOWNLOAD} Attempting to download prebuilt binary..."
    local binary_name="utf"
    if [[ "$platform" == *"windows"* ]]; then
        binary_name="utf.exe"
    fi
    
    local download_url="${GITHUB_RELEASE_BASE}/${UTF_REPO}/releases/download/v${version}/utf-${version}-${platform}.tar.gz"
    local temp_file="/tmp/utf-${version}-${platform}.tar.gz"
    
    if download_with_retry "$download_url" "$temp_file" 2>/dev/null; then
        echo "${CHECK} Downloaded prebuilt binary"
        
        # Extract binary
        cd "$UTF_INSTALL_DIR"
        if command -v tar >/dev/null 2>&1; then
            tar -xzf "$temp_file" "$binary_name"
            chmod +x "$binary_name"
            rm -f "$temp_file"
            echo "${CHECK} Installed prebuilt binary"
        else
            echo "${WARNING} tar not available, falling back to source build"
            install_rust
            build_from_source
        fi
    else
        echo "${WARNING} Prebuilt binary not available, building from source..."
        install_rust
        build_from_source
    fi
    
    echo ""
    
    # Install language configurations
    install_language_configs
    echo ""
    
    # Configure shell
    configure_shell
    echo ""
    
    # Verify installation
    if [ -x "$UTF_INSTALL_DIR/utf" ] || [ -x "$UTF_INSTALL_DIR/utf.exe" ]; then
        echo "${CHECK} ${GREEN}Installation completed successfully!${NC}"
        echo ""
        echo "${INFO} ${CYAN}What was installed:${NC}"
        echo "  • Binary: $UTF_INSTALL_DIR/$binary_name"
        echo "  • Language configs: $UTF_CONFIG_DIR/language_configs/"
        echo "  • Shell integration: configured"
        echo ""
        echo "${ROCKET} ${YELLOW}Next steps:${NC}"
        echo "  1. Restart your terminal or run: ${CYAN}source ~/.bashrc${NC} (or ~/.zshrc)"
        echo "  2. Test with: ${CYAN}utf --help${NC}"
        echo "  3. View languages: ${CYAN}utf languages${NC}"
        echo ""
        echo "${INFO} ${PURPLE}Generate tests from any Git repository:${NC}"
        echo "  ${CYAN}utf git-repo https://github.com/user/repo.git${NC}"
        echo ""
        echo "${INFO} ${PURPLE}Generate tests with comprehensive coverage:${NC}"
        echo "  ${CYAN}utf generate examples/sample.py${NC}  # 85% coverage target"
        echo "  ${CYAN}utf generate examples/sample.js${NC}  # 80% coverage target"
        echo "  ${CYAN}utf generate examples/sample.rs${NC}  # 75% coverage target"
        echo ""
    else
        echo "${ERROR} Installation verification failed"
        exit 1
    fi
}

# Run the installer
main "$@"