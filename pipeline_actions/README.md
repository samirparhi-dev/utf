# Unified Testing Framework - CI/CD Pipeline Integration

This directory contains ready-to-use CI/CD pipeline configurations for integrating the Unified Testing Framework into your continuous integration workflows.

## 🚀 Quick Start

### GitHub Actions

**Option 1: Use the Composite Action (Recommended)**
```yaml
# .github/workflows/unified-testing.yml
name: Unified Testing Framework
on: [push, pull_request]

jobs:
  test-generation:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Run Unified Testing Framework
        uses: unified-testing/unified-test-framework@v1
        with:
          generate-tests: true
          languages: 'javascript,python,rust'
```

**Option 2: Copy the Full Workflow**
Copy `github/unified-testing.yml` to `.github/workflows/unified-testing.yml`

### GitLab CI

**Option 1: Use the Template (Recommended)**
```yaml
# .gitlab-ci.yml
include:
  - remote: 'https://raw.githubusercontent.com/unified-testing/unified-test-framework/main/pipeline_actions/gitlab/unified-testing-template.yml'

variables:
  GENERATE_TESTS: "true"
```

**Option 2: Copy the Full Pipeline**
Copy `gitlab/.gitlab-ci.yml` to your repository root as `.gitlab-ci.yml`

### Azure DevOps

**Option 1: Use the Template**
```yaml
# azure-pipelines.yml
resources:
  repositories:
    - repository: unified-testing
      type: github
      name: unified-testing/unified-test-framework

extends:
  template: pipeline_actions/azure/unified-testing-template.yml@unified-testing
  parameters:
    generateTests: true
```

**Option 2: Copy the Full Pipeline**
Copy `azure/azure-pipelines.yml` to your repository root

## 📁 Directory Structure

```
pipeline_actions/
├── github/
│   ├── action.yml                 # Composite action for reuse
│   ├── unified-testing.yml        # Full workflow
│   └── example-usage.yml          # Usage examples
├── gitlab/
│   ├── .gitlab-ci.yml             # Full pipeline
│   ├── unified-testing-template.yml # Reusable template
│   └── example-usage.yml          # Usage examples
├── azure/
│   ├── azure-pipelines.yml        # Full pipeline
│   └── unified-testing-template.yml # Reusable template
└── README.md                      # This file
```

## ⚙️ Configuration Options

### Common Variables/Inputs

| Option | Description | Default | Platforms |
|--------|-------------|---------|-----------|
| `generate-tests` | Enable test generation | `true` | All |
| `languages` | Languages to process | `auto` | GitHub, Azure |
| `build-plugins` | Build IDE plugins | `false` | All |
| `output-dir` | Output directory for tests | `generated-tests` | GitHub, Azure |
| `custom-files` | Specific files to analyze | `""` | All |

### Platform-Specific Configuration

#### GitHub Actions
```yaml
- uses: unified-testing/unified-test-framework@v1
  with:
    generate-tests: true
    languages: 'javascript,python,rust'
    output-dir: 'my-tests'
    build-plugins: false
    fail-on-analysis-error: false
```

#### GitLab CI
```yaml
variables:
  GENERATE_TESTS: "true"
  BUILD_PLUGINS: "false"
  UNIFIED_TESTING_FILES: "src/main.js src/utils.py"
```

#### Azure DevOps
```yaml
parameters:
  - name: generateTests
    type: boolean
    default: true
  - name: languages
    type: string
    default: 'javascript,python,rust'
```

## 🎯 Use Cases

### 1. Automatic Test Generation on Pull Requests

**GitHub:**
```yaml
on:
  pull_request:
    branches: [main]
    
jobs:
  generate-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: unified-testing/unified-test-framework@v1
        with:
          generate-tests: true
      - uses: peter-evans/create-pull-request@v5
        with:
          title: 'Auto-generated tests'
          body: 'Generated tests for PR changes'
```

### 2. Language-Specific Analysis

**GitLab:**
```yaml
analyze:python-only:
  extends: .analyze_template
  variables:
    ANALYZE_PATTERN: "*.py"
  rules:
    - changes:
      - "**/*.py"
```

### 3. Plugin Building for Releases

**Azure:**
```yaml
- stage: BuildPlugins
  condition: eq(variables['Build.SourceBranch'], 'refs/heads/main')
  jobs:
    - template: plugin-build-template.yml
      parameters:
        buildPlugins: true
```

## 🔍 Analysis and Test Generation

The pipelines automatically:

1. **Detect Languages**: Scans repository for supported file types
2. **Analyze Patterns**: Identifies testable patterns (functions, forms, APIs)
3. **Generate Tests**: Creates framework-specific test files
4. **Store Artifacts**: Saves results for download/review

### Supported File Types
- **JavaScript**: `.js`, `.jsx`, `.ts`, `.tsx`
- **Python**: `.py`
- **Rust**: `.rs`

### Generated Test Frameworks
- **JavaScript**: Jest tests (`.test.js`)
- **Python**: Pytest tests (`test_*.py`)
- **Rust**: Cargo test modules

## 📊 Artifacts and Outputs

### Analysis Results
- Pattern detection reports
- Confidence scores
- File-specific analysis

### Generated Tests
- Framework-specific test files
- Organized by language
- Ready to run with standard test runners

### IDE Plugins
- Zed plugin source
- VSCode extension
- Spring IDE plugin

## 🔧 Advanced Configuration

### Custom File Selection

**GitHub:**
```yaml
- uses: unified-testing/unified-test-framework@v1
  with:
    files: 'src/main.js src/utils.py lib/core.rs'
```

**GitLab:**
```yaml
analyze:custom:
  extends: .analyze_template
  variables:
    ANALYZE_FILES: "src/main.js src/utils.py lib/core.rs"
```

### Conditional Execution

**GitHub:**
```yaml
- if: contains(github.event.head_commit.message, '[generate-tests]')
  uses: unified-testing/unified-test-framework@v1
```

**GitLab:**
```yaml
rules:
  - if: $CI_COMMIT_MESSAGE =~ /\[generate-tests\]/
```

**Azure:**
```yaml
condition: contains(variables['Build.SourceVersionMessage'], '[generate-tests]')
```

## 🚨 Troubleshooting

### Common Issues

1. **Binary Build Failures**
   - Ensure Rust 1.70+ is available
   - Check internet connectivity for cloning
   - Verify sufficient disk space

2. **File Detection Issues**
   - Check file extensions match supported types
   - Verify files are not in ignored directories
   - Use custom file selection for edge cases

3. **Permission Issues**
   - Ensure binary has execute permissions
   - Check write permissions for output directories

### Debug Mode

Enable verbose logging:

**GitHub:**
```yaml
env:
  RUST_BACKTRACE: full
  RUST_LOG: debug
```

**GitLab:**
```yaml
variables:
  RUST_BACKTRACE: "full"
  RUST_LOG: "debug"
```

## 📈 Performance Tips

1. **Cache Dependencies**: Use caching for Rust builds
2. **Parallel Execution**: Run analysis jobs in parallel
3. **Selective Triggers**: Use path-based triggers
4. **Artifact Management**: Set appropriate retention periods

## 🤝 Contributing

To improve these pipeline configurations:

1. Test changes with your repository
2. Ensure backward compatibility
3. Update documentation
4. Submit pull request with examples

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/unified-testing/unified-test-framework/issues)
- **Discussions**: [GitHub Discussions](https://github.com/unified-testing/unified-test-framework/discussions)
- **Documentation**: [Full Documentation](https://docs.unified-testing.com)

## 📄 License

These pipeline configurations are provided under the same license as the Unified Testing Framework - MIT License.