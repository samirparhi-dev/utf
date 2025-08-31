# Unified Test Framework

A powerful Rust CLI tool that automatically generates unit tests for multiple programming languages by analyzing code patterns and creating framework-specific test skeletons.

## 🚀 Quick Start

```bash
# One-line installation (recommended)
curl -fsSL https://raw.githubusercontent.com/samirparhi-dev/unified-test-framework/main/docs/installation/install.sh | bash

# Generate tests for a Git repository
utf git-repo https://github.com/user/repo.git

# Generate tests for a single file  
utf generate src/main.js

# Analyze patterns in a file
utf analyze src/utils.py
```

## 📦 Installation

### Option 1: One-Line Installer (Recommended)

```bash
curl -fsSL https://raw.githubusercontent.com/samirparhi-dev/unified-test-framework/main/docs/installation/install.sh | bash
```

**Features:**
- ✅ Cross-platform (macOS, Linux, Windows/WSL)
- ✅ Automatic platform detection
- ✅ Prebuilt binaries + source build fallback
- ✅ Shell integration (bash/zsh/fish)
- ✅ Language configuration setup
- ✅ Industry-standard coverage targets (Python: 85%, JavaScript: 80%, Rust: 75%)

### Option 2: Build from Source

```bash
git clone https://github.com/your-repo/unified-test-framework
cd unified-test-framework
cargo install --path .
```

**Prerequisites**: Rust 1.70+

### Advanced Installation Options

```bash
# Force reinstall
UTF_FORCE=1 curl -fsSL https://raw.githubusercontent.com/samirparhi-dev/unified-test-framework/main/docs/installation/install.sh | bash

# Install to custom directory
UTF_INSTALL_DIR=~/bin curl -fsSL https://raw.githubusercontent.com/samirparhi-dev/unified-test-framework/main/docs/installation/install.sh | bash

# Install specific version
UTF_VERSION=v0.1.0 curl -fsSL https://raw.githubusercontent.com/samirparhi-dev/unified-test-framework/main/docs/installation/install.sh | bash
```

## 🔧 Commands

| Command | Description | Example |
|---------|-------------|---------|
| `utf git-repo <url>` | Generate tests for entire Git repository | `utf git-repo https://github.com/user/repo.git` |
| `utf generate <file>` | Generate tests for a single file | `utf generate src/main.js --output tests/` |
| `utf analyze <file>` | Analyze code patterns | `utf analyze src/utils.py` |
| `utf languages` | List supported languages | `utf languages` |

## 🌍 Language & Framework Support

| Language | Frameworks Available | Test Location | Example |
|----------|---------------------|---------------|---------|
| **Java** | JUnit 5, TestNG | `src/test/java/` | `UserServiceTest.java` |
| **JavaScript** | Jest, Mocha | `__tests__/` | `utils.test.js` |
| **TypeScript** | Jest, Mocha | `__tests__/` | `service.test.ts` |
| **Python** | pytest, unittest | `tests/` | `test_utils.py` |
| **Rust** | cargo-test, nextest | `tests/` | `test_lib.rs` |
| **Go** | testing, testify | same directory | `utils_test.go` |
| **PHP** | PHPUnit, Pest | `tests/` | `UtilsTest.php` |
| **C#** | NUnit, xUnit | `Tests/` | `UtilsTest.cs` |
| **Swift** | XCTest, Quick | `Tests/` | `UtilsTests.swift` |
| **Kotlin** | JUnit 5, Kotest | `src/test/kotlin/` | `UtilsTest.kt` |

## ✨ Key Features

- **🔄 Git Integration**: Clone and process entire repositories
- **🎯 Smart Test Placement**: Follows language-specific conventions
- **🛡️ Existing Test Detection**: Never overwrites existing tests
- **🔧 Framework Choice**: Interactive selection of testing frameworks
- **📁 Organized Output**: Tests placed in proper directories
- **🚫 Intelligent Filtering**: Skips test/build directories automatically
- **🧪 Real Test Logic**: Generates actual assertions instead of TODO placeholders
- **📊 Coverage Targets**: Industry-standard coverage goals per language
- **🔍 Pattern Recognition**: Detects functions, classes, validations, and APIs

## 💡 Git Repository Workflow

```bash
# The tool will:
# 1. Clone the repository
# 2. Detect languages (Java, Python, JS, etc.)
# 3. Prompt framework choice for each language
# 4. Generate tests in standard locations
# 5. Skip files that already have tests

utf git-repo https://github.com/jaygajera17/E-commerce-project-springBoot.git

# Output example:
# 🔍 Detected languages: ["java", "javascript"]  
# 📋 Choose testing framework for JAVA:
#   1. junit5
#   2. testng
# Enter choice (1-2): 1
# 
# ✅ Generated 87 tests -> src/test/java/UserServiceTest.java
# ⏭️  Test already exists: src/test/java/ProductServiceTest.java
```

## 🎯 Pattern Detection

The framework detects and generates tests for:

- **Functions/Methods**: Parameter analysis and return type detection
- **Classes**: Constructor and method testing
- **Form Validation**: Email fields, required fields
- **API Endpoints**: HTTP methods and parameters (basic)
- **Data Models**: Getters, setters, validation logic

## 📊 Example Output

**JavaScript (Jest)** - Enhanced with Real Logic:
```javascript
describe('Generated JavaScript Tests', () => {
  it('should_validate_correct_email_format', () => {
    // Test valid email input formats
    expect(validateEmail('user@example.com')).toBe(true);
    expect(validateEmail('test.email+tag@example.co.uk')).toBe(true);
    expect(validateEmail('user.name@domain.org')).toBe(true);
  });

  it('should_handle_negative_numbers_in_addition', () => {
    // Test calculateSum with negative numbers
    expect(calculateSum(-2, 3)).toBe(1);
    expect(calculateSum(-5, -3)).toBe(-8);
    expect(calculateSum(5, -2)).toBe(3);
  });
});
```

**Python (pytest)** - Comprehensive Test Coverage:
```python
class TestGenerated:
    def test_calculate_area_positive_numbers(self):
        """Test area calculation with positive numbers"""
        assert calculate_area(5, 3) == 15
        assert calculate_area(10, 7) == 70
        assert calculate_area(1, 1) == 1
        assert calculate_area(2.5, 4) == 10.0

    def test_validate_email_error_handling(self):
        """Test email validation error handling"""
        with pytest.raises(TypeError):
            validate_email(None)
        with pytest.raises(TypeError):
            validate_email(123)
```

**Rust** - Industry-Standard Coverage:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_boundary_values() {
        // Test add with boundary values
        assert_eq!(add(0, 1), 1);
        assert_eq!(add(i32::MAX, 0), i32::MAX);
        assert_eq!(add(i32::MIN, 0), i32::MIN);
    }
}
```

## 🔗 More Information

- **Installation Guide**: See [docs/installation/INSTALL.md](docs/installation/INSTALL.md) for detailed installation options
- **Detailed Usage**: See [docs/usage/USAGE.md](docs/usage/USAGE.md) for comprehensive examples  
- **Coverage Standards**: See [docs/usage/Coverage.md](docs/usage/Coverage.md) for industry targets & examples
- **CI/CD Integration**: See [pipeline_actions/README.md](pipeline_actions/README.md)
- **Architecture**: Built with modular Rust architecture for extensibility

---

**Quick Links**: [Installation](#-installation) | [Commands](#-commands) | [Languages](#-language--framework-support) | [Examples](docs/usage/USAGE.md)