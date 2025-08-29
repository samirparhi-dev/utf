# Unified Test Framework - Usage Guide

Comprehensive guide for using the Unified Test Framework CLI to generate tests across multiple programming languages.

## 📋 Table of Contents

- [Installation](#-installation)
- [Core Commands](#-core-commands)
- [Directory Testing](#-directory-testing)
- [Git Repository Testing](#-git-repository-testing)
- [Framework Selection](#-framework-selection)
- [Language-Specific Usage](#-language-specific-usage)
- [Advanced Workflows](#-advanced-workflows)
- [CI/CD Integration](#-cicd-integration)
- [Troubleshooting](#-troubleshooting)

## 🚀 Installation

### Option 1: Build from Source (Recommended)

```bash
# Prerequisites: Rust 1.70+
git clone https://github.com/your-repo/unified-test-framework
cd unified-test-framework
cargo install --path .

# Verify installation
uft --help
```

### Option 2: Development Mode

```bash
# For development/testing
cargo build --release

# Use with full path
./target/release/uft --help
```

## 🔧 Core Commands

### 1. `dir` - Directory Testing

Generate tests for all supported files in a local directory with smart framework selection.

```bash
# Basic usage
uft dir <directory-path>

# With custom language configs
uft dir <directory-path> --config-dir <config-directory>
```

**Examples:**
```bash
# Test current directory
uft dir .

# Test specific directory
uft dir /path/to/my-project

# Test with custom language configs
uft dir ./src --config-dir ./custom-configs
```

**What this does:**
- ✅ Recursively scans the specified directory
- ✅ Finds all supported files (`.js`, `.py`, `.rs`, `.java`, `.go`, etc.)
- ✅ Skips test directories (`tests/`, `__tests__/`, `spec/`, etc.)
- ✅ Skips build artifacts (`node_modules/`, `target/`, `build/`, etc.)
- ✅ Detects all languages present in the directory
- ✅ Prompts you to choose testing frameworks for each language
- ✅ Generates test files in appropriate language-specific locations
- ✅ Provides detailed progress and summary reporting

**Directory Testing Workflow:**

```bash
uft dir /path/to/my-project
```

**Step-by-step process:**

1. **🔍 Directory Scanning**
   ```
   🔍 Scanning directory: /path/to/my-project
   📝 Found 25 source files to test
   ```

2. **🔍 Language Detection**
   ```
   🔍 Detected languages: ["java", "javascript", "python"]
   ```

3. **📋 Framework Selection** (Interactive)
   ```
   📋 Choose testing framework for JAVA:
     1. junit5
     2. testng
   Enter choice (1-2): 1
   ```

4. **🔍 File Processing**
   ```
   🔍 Processing: src/utils.js
     ✅ Generated 3 tests -> __tests__/utils.test.js
   🔍 Processing: src/service.py
     ✅ Generated 5 tests -> tests/test_service.py
   🔍 Processing: lib/Helper.java
     ⏭️  Test already exists: lib/test/HelperTest.java
   ```

5. **📊 Summary Report**
   ```
   🎉 Test generation complete!
   📊 Summary:
      • Processed files: 20
      • Skipped files (tests exist): 5
      • Total test cases: 89
      • Directory: /path/to/my-project
   
   💡 Next steps:
      1. Review and implement test logic in generated files
      2. Run tests with your project's test command
   ```

## 🔍 Directory Testing

The `uft dir` command is perfect for testing existing projects or directories without needing Git initialization. Simply point it at any directory containing source code.

### Use Cases

- **Existing Projects**: Test legacy codebases that aren't in Git
- **Monorepos**: Test specific subdirectories within larger repositories  
- **Local Development**: Quick test generation for local experiments
- **CI/CD**: Process build artifacts or extracted code

### Examples

**Test a Multi-Language Project:**
```bash
# Directory structure:
my-project/
├── backend/
│   ├── UserService.java
│   └── ProductService.java  
├── frontend/
│   ├── utils.js
│   └── components.js
└── scripts/
    └── deploy.py

# Generate tests for everything
uft dir my-project/

# Results in:
# backend/test/UserServiceTest.java
# backend/test/ProductServiceTest.java
# frontend/__tests__/utils.test.js
# frontend/__tests__/components.test.js  
# scripts/tests/test_deploy.py
```

**Test Specific Subdirectories:**
```bash
# Test only backend Java code
uft dir my-project/backend/

# Test only frontend JavaScript code
uft dir my-project/frontend/

# Test only Python scripts
uft dir my-project/scripts/
```

### 2. `git-repo` - Repository Testing

Generate tests for entire Git repositories with smart framework selection.

```bash
# Basic usage
uft git-repo <repository-url>

# With options
uft git-repo <repository-url> [OPTIONS]
```

**Options:**
- `--branch <branch>`: Specify branch (default: main)
- `--config-dir <dir>`: Custom language configs (default: ./language_configs)

**Examples:**
```bash
# Test a Spring Boot project
uft git-repo https://github.com/spring-projects/spring-boot.git

# Test a Node.js project
uft git-repo https://github.com/expressjs/express.git

# Test a Python project with specific branch
uft git-repo https://github.com/pallets/flask.git --branch main
```

### 2. `generate` - Single File Testing

Generate tests for individual files.

```bash
# Basic usage
uft generate <file-path>

# With custom output directory
uft generate <file-path> --output <directory>
```

**Examples:**
```bash
# JavaScript file
uft generate src/utils.js

# Python file with custom output
uft generate src/service.py --output my-tests/

# Java file
uft generate src/main/java/UserService.java
```

### 3. `analyze` - Pattern Analysis

Analyze code without generating tests to see detected patterns.

```bash
uft analyze <file-path>
```

**Examples:**
```bash
# Analyze a JavaScript file
uft analyze src/components/UserForm.js

# Output:
# Found 3 patterns:
# - abc123 (Function: validateEmail) confidence: 0.90
# - def456 (Function: submitForm) confidence: 0.85
# - ghi789 (Form: email field) confidence: 0.80
```

### 4. `languages` - Supported Languages

List all supported languages and their configurations.

```bash
uft languages

# Output shows:
# 📝 JAVA
#    Extensions: .java
#    Test files: Test.java
#
# 📝 JAVASCRIPT  
#    Extensions: .js, .jsx
#    Test files: test.js
```

## 🔄 Git Repository Testing

The most powerful feature - automatically process entire Git repositories.

### Workflow Overview

```bash
uft git-repo https://github.com/user/awesome-project.git
```

**Step-by-step process:**

1. **🔄 Repository Cloning**
   ```
   🔄 Cloning repository: https://github.com/user/awesome-project.git
   ✅ Repository cloned to: awesome-project
   ```

2. **🔍 Language Detection**
   ```
   🔍 Detected languages: ["java", "javascript", "python"]
   ```

3. **📋 Framework Selection** (Interactive)
   ```
   📋 Choose testing framework for JAVA:
     1. junit5
     2. testng
   Enter choice (1-2): 1

   📋 Choose testing framework for JAVASCRIPT:
     1. jest
     2. mocha
   Enter choice (1-2): 1
   ```

4. **🔍 File Processing**
   ```
   📝 Found 25 source files to test
   🔍 Processing: src/main/java/UserService.java
     ✅ Generated 5 tests -> src/test/java/UserServiceTest.java
   🔍 Processing: src/main/java/ProductService.java
     ⏭️  Test already exists: src/test/java/ProductServiceTest.java
   ```

5. **📊 Summary Report**
   ```
   🎉 Test generation complete!
   📊 Summary:
      • Processed files: 20
      • Skipped files (tests exist): 5
      • Total test cases: 127
      • Repository: awesome-project
   
   💡 Next steps:
      1. cd awesome-project
      2. Review and implement test logic in generated files
      3. Run tests with your project's test command
   ```

### Repository Examples

**Java Spring Boot Project:**
```bash
uft git-repo https://github.com/jaygajera17/E-commerce-project-springBoot.git

# Results in:
# src/test/java/UserServiceTest.java
# src/test/java/ProductControllerTest.java
# src/test/java/OrderRepositoryTest.java
```

**Node.js Express Project:**
```bash
uft git-repo https://github.com/expressjs/express.git

# Results in:
# __tests__/lib/express.test.js
# __tests__/lib/router.test.js
# __tests__/lib/middleware.test.js
```

**Python Django Project:**
```bash
uft git-repo https://github.com/django/django.git

# Results in:  
# tests/test_models.py
# tests/test_views.py
# tests/test_utils.py
```

## 🎯 Framework Selection

The tool offers popular testing frameworks for each language:

### Interactive Selection

When processing repositories, you'll see prompts like:

```bash
📋 Choose testing framework for JAVA:
  1. junit5        # Modern JUnit with annotations
  2. testng        # TestNG with flexible configuration

📋 Choose testing framework for PYTHON:
  1. pytest       # Popular, feature-rich
  2. unittest     # Built-in Python testing

📋 Choose testing framework for JAVASCRIPT:
  1. jest         # Facebook's testing framework  
  2. mocha        # Flexible testing framework
```

### Framework Comparison

| Language | Framework | Pros | Generated Style |
|----------|-----------|------|-----------------|
| **Java** | JUnit 5 | Modern, annotations, parameterized tests | `@Test void shouldDoSomething()` |
| | TestNG | Flexible, groups, dependencies | `@Test public void testMethod()` |
| **JavaScript** | Jest | Built-in mocking, snapshots, coverage | `test('should work', () => {})` |
| | Mocha | Flexible, many assertion libraries | `it('should work', () => {})` |
| **Python** | pytest | Fixtures, parametrization, plugins | `def test_something(self):` |
| | unittest | Built-in, familiar to Java users | `def test_something(self):` |

## 🌍 Language-Specific Usage

### Java

**Supported Patterns:**
- Public/private methods with parameters
- Class constructors  
- Spring annotations (@Service, @Controller)
- JPA entity methods

**Test Generation:**
```bash
uft generate src/main/java/UserService.java

# Generates: src/test/java/UserServiceTest.java
```

**Generated JUnit 5 Example:**
```java
import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

class UserServiceTest {

    @Test
    void shouldValidateEmail() {
        // Test for method validateEmail
        // TODO: Implement test logic
    }

    @Test
    void shouldCreateUser() {
        // Test for method createUser
        // TODO: Implement test logic
    }
}
```

**Running Tests:**
```bash
cd your-project
mvn test
# or
./gradlew test
```

### JavaScript/TypeScript

**Supported Patterns:**
- Function declarations and expressions
- Arrow functions
- Class methods
- Form validation patterns
- Email input fields

**Test Generation:**
```bash
uft generate src/utils.js

# Generates: __tests__/utils.test.js
```

**Generated Jest Example:**
```javascript
describe('Utils Tests', () => {
  test('should validate email correctly', () => {
    // Test for method validateEmail
    // TODO: Implement test logic
  });

  test('should calculate sum correctly', () => {
    // Test for method calculateSum
    // TODO: Implement test logic
  });
});
```

**Running Tests:**
```bash
npm test
# or
yarn test
```

### Python

**Supported Patterns:**
- Function definitions (`def`)
- Class methods
- Django/Flask email fields
- Validation functions

**Test Generation:**
```bash
uft generate src/service.py

# Generates: tests/test_service.py
```

**Generated pytest Example:**
```python
import pytest

class TestService:
    def test_validate_email(self):
        """Test email validation"""
        # TODO: Implement test logic
        pass

    def test_process_data(self):
        """Test data processing"""  
        # TODO: Implement test logic
        pass
```

**Running Tests:**
```bash
pytest
# or
python -m pytest tests/
```

### Rust

**Supported Patterns:**
- Public and private functions
- Methods in impl blocks
- Module functions

**Test Generation:**
```bash  
uft generate src/lib.rs

# Generates: tests/test_lib.rs
```

**Generated Cargo Test Example:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_sum() {
        // Test for method calculate_sum
        // TODO: Implement test logic
    }

    #[test]
    fn test_validate_input() {
        // Test for method validate_input
        // TODO: Implement test logic
    }
}
```

**Running Tests:**
```bash
cargo test
```

### Go

**Supported Patterns:**
- Function declarations
- Method receivers
- Package functions

**Test Generation:**
```bash
uft generate utils.go

# Generates: utils_test.go (same directory)
```

**Generated Example:**
```go
package main

import (
    "testing"
)

func TestCalculateSum(t *testing.T) {
    // Test for method CalculateSum
    // TODO: Implement test logic
}

func TestValidateEmail(t *testing.T) {
    // Test for method ValidateEmail
    // TODO: Implement test logic
}
```

**Running Tests:**
```bash
go test
```

## 🔧 Advanced Workflows

### Batch Processing Multiple Repositories

```bash
#!/bin/bash
# process-multiple-repos.sh

REPOS=(
  "https://github.com/user/project1.git"
  "https://github.com/user/project2.git" 
  "https://github.com/user/project3.git"
)

for repo in "${REPOS[@]}"; do
  echo "Processing $repo..."
  uft git-repo "$repo"
  echo "Completed $repo"
  echo "---"
done
```

### Selective File Processing

```bash
# Process only changed files in git
git diff --name-only HEAD~1 | grep -E '\.(java|js|py|rs)$' | while read file; do
  if [ -f "$file" ]; then
    uft generate "$file"
  fi
done

# Process specific patterns
find src/ -name "*.java" -path "*/service/*" | while read file; do
  uft generate "$file" --output service-tests/
done
```

### Integration with Build Scripts

**Maven (pom.xml):**
```xml
<plugin>
  <groupId>org.codehaus.mojo</groupId>
  <artifactId>exec-maven-plugin</artifactId>
  <configuration>
    <executable>uft</executable>
    <arguments>
      <argument>generate</argument>
      <argument>src/main/java/UserService.java</argument>
    </arguments>
  </configuration>
</plugin>
```

**package.json (Node.js):**
```json
{
  "scripts": {
    "generate-tests": "find src/ -name '*.js' -exec uft generate {} \\;",
    "analyze-code": "find src/ -name '*.js' -exec uft analyze {} \\;"
  }
}
```

**Makefile:**
```makefile
.PHONY: generate-tests analyze-code

generate-tests:
	@find src/ -name "*.py" -exec uft generate {} --output tests/generated/ \;

analyze-code:
	@find src/ -name "*.py" -exec uft analyze {} \;

test-generated:
	pytest tests/generated/
```

## 🔄 CI/CD Integration

### GitHub Actions

```yaml
name: Auto Generate Tests
on:
  pull_request:
    paths:
      - 'src/**/*.java'
      - 'src/**/*.js'
      - 'src/**/*.py'

jobs:
  generate-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          
      - name: Install UFT
        run: |
          git clone https://github.com/your-repo/unified-test-framework
          cd unified-test-framework
          cargo install --path .
          
      - name: Generate Tests for Changed Files
        run: |
          git diff --name-only origin/main | grep -E '\.(java|js|py)$' | while read file; do
            uft generate "$file" || true
          done
          
      - name: Commit Generated Tests
        run: |
          git config --local user.email "action@github.com"
          git config --local user.name "GitHub Action"
          git add -A
          git diff --staged --quiet || git commit -m "Auto-generated tests"
          git push
```

### GitLab CI

```yaml
generate-tests:
  stage: test
  image: rust:latest
  before_script:
    - git clone https://github.com/your-repo/unified-test-framework
    - cd unified-test-framework && cargo install --path . && cd ..
  script:
    - git diff --name-only $CI_MERGE_REQUEST_TARGET_BRANCH_SHA | grep -E '\.(java|js|py)$' | while read file; do uft generate "$file"; done
  artifacts:
    paths:
      - "**/*Test.*"
      - "**/test_*.*"
  only:
    - merge_requests
```

## 🚨 Troubleshooting

### Common Issues

**1. Command not found: `uft`**
```bash
# Solutions:
# Install globally
cargo install --path .

# Or use full path
./target/release/uft --help

# Or add to PATH
export PATH="$PWD/target/release:$PATH"
```

**2. No patterns detected**
```bash
# Check if file has supported patterns
uft analyze src/empty-file.js

# Output: Found 0 patterns
# Solution: Ensure file has functions, classes, or other detectable patterns
```

**3. Permission denied on repository cloning**
```bash
# For private repositories
git config --global credential.helper store
# Or use SSH URLs
uft git-repo git@github.com:user/private-repo.git
```

**4. Tests not placed in expected directories**
```bash
# Check current directory structure
ls -la

# Ensure you're in the project root directory
cd your-project-directory
uft generate src/main/java/Service.java

# Java tests should appear in: src/test/java/ServiceTest.java
```

**5. Framework choice not remembered**
```bash
# Each run asks for framework choice
# This is intentional - allows different choices per project
# Future version may support configuration files
```

### Debug Mode

```bash
# Enable verbose logging
export RUST_BACKTRACE=1
export RUST_LOG=debug
uft analyze src/file.js

# Check version
uft --version

# Validate binary
which uft
```

### Getting Help

```bash
# General help
uft --help

# Command-specific help
uft git-repo --help
uft generate --help
uft analyze --help
```

## 💡 Best Practices

### 1. Repository Organization

```
project/
├── src/
│   ├── main/java/           # Source code
│   └── test/java/           # Manual + generated tests  
├── __tests__/               # JavaScript tests
├── tests/                   # Python tests
└── target/                  # Build artifacts (ignored)
```

### 2. Generated Test Review

Always review and enhance generated tests:

```java
// Generated
@Test
void shouldValidateEmail() {
    // TODO: Implement test logic
}

// Enhanced
@Test
void shouldValidateEmail() {
    // Given
    String validEmail = "test@example.com";
    String invalidEmail = "invalid-email";
    
    // When & Then
    assertTrue(userService.validateEmail(validEmail));
    assertFalse(userService.validateEmail(invalidEmail));
}
```

### 3. Workflow Integration

```bash
# Pre-commit hook
#!/bin/sh
# .git/hooks/pre-commit
git diff --cached --name-only | grep -E '\.(java|js|py)$' | while read file; do
  uft analyze "$file" >/dev/null || exit 1
done
```

### 4. Team Usage

- **Standardize**: Choose consistent frameworks across team
- **Document**: Keep README updated with chosen frameworks
- **Review**: Include generated tests in code reviews
- **Enhance**: Continuously improve generated test templates

---

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/your-repo/unified-test-framework/issues)
- **Discussions**: [GitHub Discussions](https://github.com/your-repo/unified-test-framework/discussions)  
- **CI/CD Help**: See [pipeline_actions/README.md](pipeline_actions/README.md)

---

*Last updated: 2024 - See [README.md](README.md) for quick reference*