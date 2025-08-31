# UTF Installation Guide

## One-Line Installation

For macOS, Linux, and Windows (WSL/Git Bash):

```bash
curl -fsSL https://raw.githubusercontent.com/samirparhi-dev/unified-test-framework/main/install.sh | bash
```

## Installation Options

### Environment Variables

- `UTF_INSTALL_DIR`: Custom installation directory (default: `$HOME/.local/bin`)
- `UTF_VERSION`: Specific version to install (default: `latest`)
- `UTF_FORCE`: Force reinstall if already installed (default: `0`)

### Examples

```bash
# Custom installation directory
UTF_INSTALL_DIR=~/bin curl -fsSL https://raw.githubusercontent.com/samirparhi-dev/unified-test-framework/main/install.sh | bash

# Force reinstall
UTF_FORCE=1 curl -fsSL https://raw.githubusercontent.com/samirparhi-dev/unified-test-framework/main/install.sh | bash

# Install specific version
UTF_VERSION=0.1.0 curl -fsSL https://raw.githubusercontent.com/samirparhi-dev/unified-test-framework/main/install.sh | bash
```

## What Gets Installed

1. **UTF Binary**: The main `utf` executable
2. **Language Configurations**: JSON files for dynamic language support
3. **Shell Integration**: PATH configuration for your shell (bash/zsh/fish)

## Platform Support

| Platform | Architecture | Status |
|----------|--------------|--------|
| macOS | x86_64 | ✅ Supported |
| macOS | arm64 (M1/M2) | ✅ Supported |
| Linux | x86_64 | ✅ Supported |
| Linux | arm64 | ✅ Supported |
| Windows | WSL/Git Bash | ✅ Supported |

## Verification

After installation, verify it works:

```bash
# Check version
utf --version

# View help
utf --help

# List supported languages  
utf languages

# Test with a sample file
utf generate examples/sample.py
```

## Troubleshooting

### Command not found: utf

1. Restart your terminal
2. Or manually source your shell config:
   ```bash
   source ~/.bashrc   # for bash
   source ~/.zshrc    # for zsh
   ```

### Installation Failed

1. **Check internet connection**: The installer downloads from GitHub
2. **Install dependencies**:
   - `curl` or `wget` for downloading
   - `git` for source builds (fallback)
   - `tar` for extracting binaries
3. **Build from source**: If prebuilt binaries fail
   ```bash
   git clone https://github.com/samirparhi-dev/unified-test-framework
   cd unified-test-framework
   cargo install --path .
   ```

### Permission Issues

If you get permission errors:

```bash
# Install to user directory (recommended)
UTF_INSTALL_DIR=$HOME/.local/bin curl -fsSL https://raw.githubusercontent.com/samirparhi-dev/unified-test-framework/main/install.sh | bash

# Make sure ~/.local/bin is in PATH
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

## Uninstallation

To remove UTF:

```bash
# Remove binary
rm -f ~/.local/bin/utf

# Remove configuration (optional)
rm -rf ~/.config/utf

# Remove from shell config (manual)
# Edit ~/.bashrc or ~/.zshrc and remove the UTF PATH export
```

## Manual Installation

If the automated installer doesn't work:

### Option 1: Download Release Binary

1. Go to [Releases](https://github.com/samirparhi-dev/unified-test-framework/releases)
2. Download the appropriate binary for your platform
3. Extract and place in your PATH:
   ```bash
   mkdir -p ~/.local/bin
   tar -xzf utf-*-your-platform.tar.gz
   mv utf ~/.local/bin/
   chmod +x ~/.local/bin/utf
   ```

### Option 2: Build from Source

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and build
git clone https://github.com/samirparhi-dev/unified-test-framework
cd unified-test-framework
cargo install --path .
```

## Post-Installation

Once installed, UTF provides:

- **Real test generation**: Creates actual assertions, not just TODOs
- **Industry coverage targets**: Python (85%), JavaScript (80%), Rust (75%)
- **Multiple frameworks**: Jest, pytest, cargo-test, JUnit, and more
- **Cross-language support**: 9 languages with built-in + dynamic adapters

Start generating tests:

```bash
utf git-repo https://github.com/user/awesome-project.git
```

---

*For more detailed usage, see [USAGE.md](USAGE.md) and [Coverage.md](Coverage.md)*