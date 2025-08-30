# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Development Commands

### Basic Cargo Commands
- `cargo build` - Build the project
- `cargo test --lib` - Run library tests
- `utf <command>` - Run the CLI tool (globally installed)
- `cargo check` - Quick syntax and type checking
- `cargo clippy` - Linting with clippy
- `cargo fmt` - Format code

### CLI Usage
- `utf analyze <file>` - Analyze code patterns
- `utf generate <file>` - Generate tests with comprehensive logic and industry-standard coverage
- `utf generate <file> --output <dir>` - Generate tests to specific directory
- `utf languages` - List all supported languages and frameworks
- `utf dir <directory>` - Generate tests for all supported files in a directory

**Note**: The CLI is installed globally as `utf` command. No need for `cargo run` anymore!

## Architecture Overview

This is a minimal, modular Rust-based unified testing framework that automatically generates tests for multiple programming languages.

### Core Components

#### Main Orchestrator (`src/core/mod.rs`)
- `TestOrchestrator` - Central coordinator for all language adapters
- `TestGenerator` trait - Interface that all language adapters must implement
- Core data structures: `TestablePattern`, `TestCase`, `TestSuite`

#### Language Adapters (`src/adapters/`)
- **JavaScript adapter** - Regex-based pattern detection for functions and forms
- **Python adapter** - Function and email validation pattern detection  
- **Rust adapter** - Function pattern detection

#### Pattern Types Supported
- Form validation patterns (email fields)
- Function patterns (detected via regex)
- API endpoint patterns (basic structure)

#### CLI Tool (`src/bin/unified-testing.rs`)
Main commands:
- `generate <path>` - Generate test files for source code
- `analyze <path>` - Analyze and display detected patterns

### Key Dependencies

The project uses minimal dependencies for a clean, fast implementation:
- **regex** - Pattern matching for code analysis
- **serde/serde_json** - Serialization for data structures
- **tokio** - Async runtime
- **anyhow** - Error handling
- **clap** - CLI argument parsing
- **uuid** - Unique identifier generation

### Language Support

Current language adapters support:
- **JavaScript**: Functions, email form fields → Jest tests
- **Python**: Functions, email validation → Pytest tests  
- **Rust**: Functions → Cargo tests

### Test Generation Strategy

1. **Pattern Recognition**: Regex-based pattern detection for functions and common validation patterns
2. **Confidence Scoring**: Simple heuristic-based confidence (0.7-0.9)
3. **Test Case Generation**: Framework-specific test file templates
4. **File Output**: Generates test files in appropriate format for each language

### Project Structure

```
src/
├── lib.rs                 # Main library exports
├── core/
│   └── mod.rs            # Core traits and data structures
├── adapters/
│   ├── mod.rs            # Adapter exports
│   ├── javascript.rs     # JavaScript/TypeScript support
│   ├── python.rs         # Python support
│   └── rust.rs           # Rust support
└── bin/
    └── unified-testing.rs # CLI application

examples/                  # Sample code files for testing
tests/                     # Generated test output directory
```

### Usage Examples

```bash
# Analyze a JavaScript file
utf analyze examples/sample.js

# Generate comprehensive tests for Python code (85% coverage target)
utf generate examples/sample.py

# Generate tests to custom directory
utf generate examples/sample.rs --output my-tests/

# Generate tests for entire directory
utf dir src/

# List all supported languages and their coverage targets
utf languages
```