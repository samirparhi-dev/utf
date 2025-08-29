# Unified Test Framework

A powerful Rust CLI tool that automatically generates unit tests for multiple programming languages by analyzing code patterns and creating framework-specific test skeletons.

## 🚀 Quick Start

```bash
# One-line installation (recommended)
curl -sSfL https://install.uft.dev | sh

# Generate tests for a Git repository
uft git-repo https://github.com/user/repo.git

# Generate tests for a single file  
uft generate src/main.js

# Analyze patterns in a file
uft analyze src/utils.py
```

## 📦 Installation

### Option 1: One-Line Installer (Recommended)

```bash
curl -sSfL https://install.uft.dev | sh
```

**Features:**
- ✅ Cross-platform (macOS, Linux, Windows)
- ✅ Automatic platform detection
- ✅ Prebuilt binaries + source build fallback
- ✅ Shell integration (bash/zsh/fish)
- ✅ Language configuration setup

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
UFT_FORCE=1 curl -sSfL https://install.uft.dev | sh

# Install to custom directory
UFT_INSTALL_DIR=~/bin curl -sSfL https://install.uft.dev | sh

# Install specific version
UFT_VERSION=v1.0.0 curl -sSfL https://install.uft.dev | sh
```

## 🔧 Commands

| Command | Description | Example |
|---------|-------------|---------|
| `uft git-repo <url>` | Generate tests for entire Git repository | `uft git-repo https://github.com/user/repo.git` |
| `uft generate <file>` | Generate tests for a single file | `uft generate src/main.js --output tests/` |
| `uft analyze <file>` | Analyze code patterns | `uft analyze src/utils.py` |
| `uft languages` | List supported languages | `uft languages` |

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

## 💡 Git Repository Workflow

```bash
# The tool will:
# 1. Clone the repository
# 2. Detect languages (Java, Python, JS, etc.)
# 3. Prompt framework choice for each language
# 4. Generate tests in standard locations
# 5. Skip files that already have tests

uft git-repo https://github.com/jaygajera17/E-commerce-project-springBoot.git

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

**JavaScript (Jest)**:
```javascript
describe('UserService Tests', () => {
  test('should validate email correctly', () => {
    // TODO: Implement test logic
  });
});
```

**Python (pytest)**:
```python
class TestUserService:
    def test_validate_email(self):
        """Test email validation"""
        # TODO: Implement test logic
        pass
```

**Java (JUnit 5)**:
```java
class UserServiceTest {
    @Test
    void shouldValidateEmailCorrectly() {
        // TODO: Implement test logic
    }
}
```

## 🔗 More Information

- **Detailed Usage**: See [USAGE.md](USAGE.md) for comprehensive examples
- **CI/CD Integration**: See [pipeline_actions/README.md](pipeline_actions/README.md)
- **Architecture**: Built with modular Rust architecture for extensibility

---

**Quick Links**: [Installation](#-installation) | [Commands](#-commands) | [Languages](#-language--framework-support) | [Examples](USAGE.md)