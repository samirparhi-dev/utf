# Unified Testing Framework - Usage Guide

A comprehensive guide to using the Unified Testing Framework for automated test generation across multiple programming languages.

## 📋 Table of Contents

- [Quick Start](#-quick-start)
- [Installation](#-installation)
- [Basic Commands](#-basic-commands)
- [Advanced Usage](#-advanced-usage)
- [Language Support](#-language-support)
- [IDE Integration](#-ide-integration)
- [CI/CD Integration](#-cicd-integration)
- [Configuration](#-configuration)
- [Examples](#-examples)
- [Troubleshooting](#-troubleshooting)
- [Best Practices](#-best-practices)

## 🚀 Quick Start

### 1. Installation and Basic Usage

```bash
# Clone and build
git clone https://github.com/unified-testing/unified-test-framework.git
cd unified-test-framework
cargo build --release

# Analyze a file to see detected patterns
./target/release/uft analyze examples/sample.js

# Generate tests for a file
./target/release/uft generate examples/sample.js
```

### 2. First Analysis

```bash
# Analyze your JavaScript file
./target/release/uft analyze src/utils.js
```

**Output:**
```
Analyzing patterns in: src/utils.js
Found 2 patterns:
- abc123 (confidence: 0.90)
  Function: validateEmail with 1 parameters
- def456 (confidence: 0.85)
  Function: calculateSum with 2 parameters
```

### 3. Generate Your First Tests

```bash
# Generate tests with default output
./target/release/uft generate src/utils.js

# Generate tests with custom output directory
./target/release/uft generate src/utils.js --output my-tests/
```

## 💾 Installation

### Option 1: Build from Source (Recommended)

```bash
# Prerequisites: Rust 1.70+
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Clone and build
git clone https://github.com/unified-testing/unified-test-framework.git
cd unified-test-framework
cargo build --release --bin uft

# Install globally (optional)
cargo install --path .
```

### Option 2: Download Pre-built Binary

```bash
# Download for your platform
curl -L -o uft.tar.gz \
  "https://github.com/unified-testing/unified-test-framework/releases/latest/download/uft-linux-x86_64.tar.gz"

tar -xzf uft.tar.gz
chmod +x uft
sudo mv uft /usr/local/bin/
```

### Option 3: Use in CI/CD

See [CI/CD Integration](#-cicd-integration) section for platform-specific instructions.

## 🔧 Basic Commands

### `analyze` - Pattern Detection

Analyzes source code to detect testable patterns.

```bash
# Basic analysis
uft analyze <file>

# Examples
uft analyze src/main.js
uft analyze utils/helper.py
uft analyze lib/core.rs
```

**Output Format:**
```
Analyzing patterns in: src/main.js
Found 3 patterns:
- abc123 (confidence: 0.90)
  Function: validateEmail with 1 parameters
- def456 (confidence: 0.85)  
  Form field: email (type: Email)
- ghi789 (confidence: 0.75)
  Function: processData with 3 parameters
```

### `generate` - Test Generation

Generates framework-specific test files based on detected patterns.

```bash
# Basic generation
uft generate <file>

# With custom output directory
uft generate <file> --output <directory>

# Examples
uft generate src/main.js
uft generate src/main.js --output tests/
uft generate utils/helper.py --output test_suite/
```

**Generated Output:**
- **JavaScript**: `test_generated_javascript_tests.test.js`
- **Python**: `test_generated_python_tests.py`
- **Rust**: `test_generated_rust_tests.rs`

### `plugin` - IDE Plugin Generation

Builds IDE plugins for development environment integration.

```bash
# Build specific plugin
uft plugin <type> --output <directory>

# Available plugin types
uft plugin zed --output ./plugins/
uft plugin vscode --output ./plugins/
uft plugin spring --output ./plugins/

# Build all plugins
for plugin in zed vscode spring; do
  uft plugin $plugin --output ./plugins/
done
```

### `help` - Command Help

```bash
# General help
uft --help

# Command-specific help
uft analyze --help
uft generate --help
uft plugin --help
```

## 🎯 Advanced Usage

### Batch Processing

```bash
# Analyze multiple files
find src/ -name "*.js" -exec uft analyze {} \;

# Generate tests for all Python files
find . -name "*.py" | while read file; do
  uft generate "$file" --output tests/python/
done

# Process specific file patterns
for file in src/**/*.{js,ts,jsx,tsx}; do
  [ -f "$file" ] && uft analyze "$file"
done
```

### Integration with Build Scripts

**package.json (Node.js):**
```json
{
  "scripts": {
    "analyze": "uft analyze src/main.js",
    "generate-tests": "find src/ -name '*.js' -exec uft generate {} --output tests/ \\;",
    "test-generated": "npm test tests/test_generated_*.test.js"
  }
}
```

**Makefile:**
```makefile
.PHONY: analyze generate-tests

analyze:
	@find src/ -name "*.py" -exec uft analyze {} \;

generate-tests:
	@mkdir -p tests/generated
	@find src/ -name "*.py" -exec uft generate {} --output tests/generated/ \;
	@echo "Generated tests available in tests/generated/"

test-all: generate-tests
	pytest tests/
```

**Cargo.toml (Rust):**
```toml
[[bin]]
name = "generate-tests"
path = "scripts/generate_tests.rs"
```

### Custom Workflows

**Git Hook Integration:**
```bash
#!/bin/sh
# .git/hooks/pre-commit

echo "Analyzing changed files..."
git diff --cached --name-only --diff-filter=ACM | grep -E '\.(js|py|rs)$' | while read file; do
  if [ -f "$file" ]; then
    uft analyze "$file"
  fi
done
```

**Watch Mode Script:**
```bash
#!/bin/bash
# watch-and-generate.sh

inotifywait -m -r --format '%w%f' -e modify src/ | while read file; do
  if [[ $file =~ \.(js|py|rs)$ ]]; then
    echo "File changed: $file"
    uft generate "$file" --output tests/auto-generated/
  fi
done
```

## 🌐 Language Support

### JavaScript/TypeScript

**Supported Patterns:**
- Function declarations: `function myFunction(param1, param2) { ... }`
- Arrow functions: `const myFunc = (param) => { ... }`
- Class methods: `class MyClass { myMethod() { ... } }`
- Form validation: `<input type="email" required />`

**Generated Tests:**
```javascript
const { expect } = require('@jest/globals');

describe('Generated Tests', () => {
  test('test_validateEmail', () => {
    // Test validateEmail function
    // TODO: Add specific test implementation
  });

  test('test_email_valid_email', () => {
    // Test valid email input
    const result = validateEmail('test@example.com');
    expect(result).toBe(true);
  });

  test('test_email_invalid_email', () => {
    // Test invalid email input
    const result = validateEmail('invalid-email');
    expect(result).toBe(false);
  });
});
```

**Running Generated Tests:**
```bash
npm install --save-dev jest
npm test tests/test_generated_*.test.js
```

### Python

**Supported Patterns:**
- Function definitions: `def my_function(param1, param2): ...`
- Class methods: `class MyClass: def my_method(self): ...`
- Email validation patterns: `email_field = forms.EmailField()`

**Generated Tests:**
```python
import pytest

class TestGeneratedTests:
    def test_validate_email(self):
        """Test validate_email function"""
        # TODO: Add specific test implementation
        pass
    
    def test_email_valid_email(self):
        """Test valid email input"""
        result = validate_email('test@example.com')
        assert result is True
    
    def test_email_invalid_email(self):
        """Test invalid email input"""
        result = validate_email('invalid-email')
        assert result is False
```

**Running Generated Tests:**
```bash
pip install pytest
pytest tests/test_generated_*.py -v
```

### Rust

**Supported Patterns:**
- Function definitions: `fn my_function(param1: Type, param2: Type) -> ReturnType { ... }`
- Public functions: `pub fn my_public_function() { ... }`
- Methods in impl blocks: `impl MyStruct { fn my_method(&self) { ... } }`

**Generated Tests:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_email() {
        // Test validate_email function
        // TODO: Add specific test implementation
    }
    
    #[test]
    fn test_calculate_sum() {
        // Test calculate_sum function
        // TODO: Add specific test implementation
    }
}
```

**Running Generated Tests:**
```bash
cargo test
```

## 🔌 IDE Integration

### Zed Editor Plugin

**Installation:**
```bash
# Build Zed plugin
uft plugin zed --output ~/.config/zed/extensions/

# The plugin provides:
# - Right-click context menu for "Generate Tests"
# - Command palette: "Unified Testing: Analyze File"
# - Language server integration
```

**Usage in Zed:**
1. Open a supported file (JS, Python, Rust)
2. Right-click → "Generate Tests"
3. Or use Command Palette: `Ctrl+Shift+P` → "Unified Testing: Generate Tests"

### VSCode Extension

**Installation:**
```bash
# Build VSCode extension
uft plugin vscode --output ~/.vscode/extensions/

cd ~/.vscode/extensions/vscode-uft/
npm install && npm run compile && npm run package
```

**Usage in VSCode:**
1. **Command Palette**: `Ctrl+Shift+P` → "Unified Testing: Generate Tests"
2. **Code Lens**: Click "🧪 Generate Tests" above functions
3. **Context Menu**: Right-click file → "Generate Tests"
4. **Hover**: Hover over functions to see testing options

**Extension Features:**
- Automatic pattern detection highlighting
- Test generation progress notifications
- Integration with VSCode test explorer

### Spring IDE (IntelliJ) Plugin

**Installation:**
```bash
# Build Spring IDE plugin
uft plugin spring --output ./plugins/

cd plugins/spring-uft/
./gradlew buildPlugin
```

**Usage in IntelliJ/Spring Tool Suite:**
1. **Tools Menu**: Tools → Unified Testing → Generate Tests
2. **Context Menu**: Right-click file → Unified Testing → Analyze Patterns
3. **Toolbar**: Use the Unified Testing toolbar buttons
4. **Keyboard Shortcuts**: 
   - `Ctrl+Alt+T`: Generate Tests
   - `Ctrl+Alt+A`: Analyze Patterns

## 🔄 CI/CD Integration

### GitHub Actions

**Quick Setup:**
```yaml
# .github/workflows/uft.yml
name: Unified Testing Framework
on: [push, pull_request]

jobs:
  generate-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: unified-testing/unified-test-framework@v1
        with:
          generate-tests: true
          languages: 'javascript,python,rust'
```

**Advanced Configuration:**
```yaml
jobs:
  test-generation:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Analyze and Generate Tests
        uses: unified-testing/unified-test-framework@v1
        with:
          generate-tests: true
          languages: 'javascript,python'
          output-dir: 'generated-tests'
          build-plugins: false
          fail-on-analysis-error: false
          
      - name: Run Generated Tests
        run: |
          if [ -d "generated-tests/javascript" ]; then
            npm test generated-tests/javascript/*.test.js
          fi
          if [ -d "generated-tests/python" ]; then
            pytest generated-tests/python/test_*.py
          fi
```

### GitLab CI

**Quick Setup:**
```yaml
# .gitlab-ci.yml
include:
  - remote: 'https://raw.githubusercontent.com/unified-testing/unified-test-framework/main/pipeline_actions/gitlab/unified-testing-template.yml'

variables:
  GENERATE_TESTS: "true"
```

**Custom Configuration:**
```yaml
stages:
  - analyze
  - generate-tests
  - test

analyze:custom:
  stage: analyze
  extends: .analyze_template
  variables:
    ANALYZE_FILES: "src/main.js src/utils.py lib/core.rs"
  artifacts:
    reports:
      junit: analysis-results.xml

run-generated-tests:
  stage: test
  script:
    - npm test generated-tests/javascript/
    - pytest generated-tests/python/
  dependencies:
    - generate_tests:javascript
    - generate_tests:python
```

### Azure DevOps

**Quick Setup:**
```yaml
# azure-pipelines.yml
resources:
  repositories:
    - repository: uft
      type: github
      name: unified-testing/unified-test-framework

extends:
  template: pipeline_actions/azure/unified-testing-template.yml@unified-testing
  parameters:
    generateTests: true
    languages: 'javascript,python,rust'
```

## ⚙️ Configuration

### Environment Variables

```bash
# Debug mode
export RUST_BACKTRACE=1
export RUST_LOG=debug

# Custom output directory
export UNIFIED_TESTING_OUTPUT_DIR="my-tests"

# Skip certain file patterns
export UNIFIED_TESTING_IGNORE="node_modules,target,dist"
```

### Configuration File (Future Feature)

```yaml
# uft.yml
languages:
  - javascript
  - python
  - rust

output:
  directory: "tests/generated"
  preserve_structure: true

analysis:
  confidence_threshold: 0.7
  patterns:
    - functions
    - form_validation
    - api_endpoints

generation:
  frameworks:
    javascript: "jest"
    python: "pytest"
    rust: "cargo"
  
  templates:
    custom_header: |
      // Generated by Unified Testing Framework
      // Review and modify as needed
```

## 📖 Examples

### Example 1: JavaScript Function Testing

**Input File (src/utils.js):**
```javascript
function validateEmail(email) {
  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
  return emailRegex.test(email);
}

function calculateSum(a, b) {
  return a + b;
}

function processUserData(userData) {
  if (!userData.email || !validateEmail(userData.email)) {
    throw new Error('Invalid email');
  }
  return {
    ...userData,
    processed: true,
    timestamp: Date.now()
  };
}
```

**Commands:**
```bash
# Analyze the file
uft analyze src/utils.js

# Generate tests
uft generate src/utils.js --output tests/
```

**Generated Test (tests/test_generated_javascript_tests.test.js):**
```javascript
const { expect } = require('@jest/globals');

describe('Generated Tests', () => {
  test('test_validateEmail', () => {
    // Test validateEmail function
    // TODO: Add specific test implementation for validateEmail
  });

  test('test_calculateSum', () => {
    // Test calculateSum function
    // TODO: Add specific test implementation for calculateSum
  });

  test('test_processUserData', () => {
    // Test processUserData function
    // TODO: Add specific test implementation for processUserData
  });
});
```

### Example 2: Python Class Testing

**Input File (src/user_service.py):**
```python
import re
from typing import Optional

class UserService:
    def __init__(self):
        self.users = []
    
    def validate_email(self, email: str) -> bool:
        """Validate email format"""
        pattern = r'^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$'
        return bool(re.match(pattern, email))
    
    def create_user(self, name: str, email: str) -> dict:
        """Create a new user"""
        if not self.validate_email(email):
            raise ValueError("Invalid email format")
        
        user = {
            'id': len(self.users) + 1,
            'name': name,
            'email': email,
            'active': True
        }
        self.users.append(user)
        return user
    
    def find_user_by_email(self, email: str) -> Optional[dict]:
        """Find user by email address"""
        for user in self.users:
            if user['email'] == email:
                return user
        return None
```

**Generated Test (tests/test_generated_python_tests.py):**
```python
import pytest

class TestGeneratedTests:
    def test_validate_email(self):
        """Test validate_email function"""
        # TODO: Add specific test implementation for validate_email
        pass
    
    def test_create_user(self):
        """Test create_user function"""
        # TODO: Add specific test implementation for create_user
        pass
    
    def test_find_user_by_email(self):
        """Test find_user_by_email function"""
        # TODO: Add specific test implementation for find_user_by_email
        pass
```

### Example 3: Rust Module Testing

**Input File (src/calculator.rs):**
```rust
pub struct Calculator {
    history: Vec<f64>,
}

impl Calculator {
    pub fn new() -> Self {
        Calculator {
            history: Vec::new(),
        }
    }
    
    pub fn add(&mut self, a: f64, b: f64) -> f64 {
        let result = a + b;
        self.history.push(result);
        result
    }
    
    pub fn multiply(&mut self, a: f64, b: f64) -> f64 {
        let result = a * b;
        self.history.push(result);
        result
    }
    
    pub fn get_history(&self) -> &Vec<f64> {
        &self.history
    }
    
    fn validate_input(&self, value: f64) -> bool {
        !value.is_nan() && !value.is_infinite()
    }
}
```

**Generated Test (tests/test_generated_rust_tests.rs):**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        // Test new function
        // TODO: Add specific test implementation for new
    }

    #[test]
    fn test_add() {
        // Test add function
        // TODO: Add specific test implementation for add
    }

    #[test]
    fn test_multiply() {
        // Test multiply function
        // TODO: Add specific test implementation for multiply
    }

    #[test]
    fn test_get_history() {
        // Test get_history function
        // TODO: Add specific test implementation for get_history
    }

    #[test]
    fn test_validate_input() {
        // Test validate_input function
        // TODO: Add specific test implementation for validate_input
    }
}
```

### Example 4: Full Workflow

**Project Structure:**
```
my-project/
├── src/
│   ├── main.js
│   ├── utils.py
│   └── lib.rs
├── tests/
└── package.json
```

**Complete Workflow:**
```bash
# 1. Analyze all files
echo "🔍 Analyzing all source files..."
find src/ -name "*.js" -o -name "*.py" -o -name "*.rs" | while read file; do
  echo "Analyzing: $file"
  uft analyze "$file"
done

# 2. Generate tests for all files
echo "🧪 Generating tests..."
mkdir -p tests/generated/{javascript,python,rust}

find src/ -name "*.js" -exec uft generate {} --output tests/generated/javascript/ \;
find src/ -name "*.py" -exec uft generate {} --output tests/generated/python/ \;
find src/ -name "*.rs" -exec uft generate {} --output tests/generated/rust/ \;

# 3. Run the generated tests
echo "🚀 Running generated tests..."

# JavaScript tests
if [ -d "tests/generated/javascript" ] && [ "$(ls -A tests/generated/javascript)" ]; then
  npm test tests/generated/javascript/*.test.js
fi

# Python tests  
if [ -d "tests/generated/python" ] && [ "$(ls -A tests/generated/python)" ]; then
  pytest tests/generated/python/test_*.py -v
fi

# Rust tests (copy to src for cargo test)
if [ -d "tests/generated/rust" ] && [ "$(ls -A tests/generated/rust)" ]; then
  # Copy generated test modules to src/ or create a separate test crate
  echo "Generated Rust tests available in tests/generated/rust/"
fi

echo "✅ Workflow complete! Check tests/generated/ for results."
```

## 🚨 Troubleshooting

### Common Issues

#### 1. Build Failures

**Issue**: `cargo build` fails with dependency errors
```
error[E0463]: can't find crate for `unified_test_framework`
```

**Solution**:
```bash
# Clean and rebuild
cargo clean
cargo build --release

# Or update dependencies
cargo update
```

#### 2. Binary Not Found

**Issue**: `uft: command not found`

**Solution**:
```bash
# Use full path
./target/release/uft analyze file.js

# Or install globally
cargo install --path .

# Or add to PATH
export PATH="$PWD/target/release:$PATH"
```

#### 3. No Patterns Detected

**Issue**: Analysis finds no patterns in valid source files

**Possible Causes & Solutions**:
```bash
# Check file extension is supported
uft analyze --help

# Verify file has detectable patterns
# Must have: functions, classes, form fields

# Check file encoding (must be UTF-8)
file -bi your-file.js
```

#### 4. Permission Errors

**Issue**: Cannot write to output directory

**Solution**:
```bash
# Check permissions
ls -la tests/

# Create directory with proper permissions
mkdir -p tests/generated
chmod 755 tests/generated

# Or use different output directory
uft generate file.js --output ~/my-tests/
```

#### 5. Plugin Build Failures

**Issue**: IDE plugin builds fail

**Solution**:
```bash
# Check required dependencies
# For VSCode: Node.js and npm
# For Spring IDE: Java and Gradle

# Clean and retry
rm -rf plugins-output/
uft plugin vscode --output plugins-output/
```

### Debug Mode

```bash
# Enable debug logging
export RUST_BACKTRACE=1
export RUST_LOG=debug
uft analyze file.js

# Check version
uft --version

# Validate installation
uft --help
```

### Getting Help

```bash
# Command help
uft --help
uft analyze --help
uft generate --help
uft plugin --help

# Check examples
ls examples/
uft analyze examples/sample.js
```

## ✨ Best Practices

### 1. File Organization

```bash
# Recommended project structure
project/
├── src/                    # Source code
├── tests/
│   ├── unit/              # Manual unit tests
│   ├── integration/       # Integration tests
│   └── generated/         # Generated tests
│       ├── javascript/
│       ├── python/
│       └── rust/
└── tools/
    └── generate-tests.sh  # Test generation script
```

### 2. Workflow Integration

**Pre-commit Hook:**
```bash
#!/bin/sh
# .git/hooks/pre-commit
git diff --cached --name-only | grep -E '\.(js|py|rs)$' | while read file; do
  uft analyze "$file" >/dev/null || exit 1
done
```

**Continuous Integration:**
```yaml
# Always analyze changes
- name: Analyze Changed Files
  run: |
    git diff --name-only HEAD~1 HEAD | grep -E '\.(js|py|rs)$' | while read file; do
      uft analyze "$file"
    done
```

### 3. Code Quality

- **Review Generated Tests**: Always review and modify generated tests
- **Add Custom Logic**: Replace TODO comments with actual test implementations
- **Test Edge Cases**: Add additional tests for edge cases
- **Maintain Test Data**: Use fixtures and mock data appropriately

### 4. Performance Tips

```bash
# Process files in parallel
find src/ -name "*.js" | xargs -P 4 -I {} uft analyze {}

# Use specific output directories
uft generate file.js --output tests/$(date +%Y%m%d)/

# Cache build artifacts in CI
cache:
  key: uft-${{ hashFiles('Cargo.lock') }}
  paths:
    - target/
    - ~/.cargo/
```

### 5. Team Collaboration

- **Document Custom Patterns**: Document any custom code patterns your team uses
- **Share Templates**: Create team-specific test templates
- **Regular Updates**: Keep the framework updated for new features
- **Code Reviews**: Include generated tests in code review process

---

## 📞 Support & Resources

- **GitHub Repository**: [unified-testing/unified-test-framework](https://github.com/unified-testing/unified-test-framework)
- **Issues & Bug Reports**: [GitHub Issues](https://github.com/unified-testing/unified-test-framework/issues)
- **Feature Requests**: [GitHub Discussions](https://github.com/unified-testing/unified-test-framework/discussions)
- **CI/CD Pipelines**: [Pipeline Documentation](pipeline_actions/README.md)
- **API Reference**: [API Documentation](https://docs.uft.com)

---

*This documentation covers version 0.1.0 of the Unified Testing Framework. For the latest updates, please check the GitHub repository.*