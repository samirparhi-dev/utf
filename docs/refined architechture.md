# Unified Test Framework (payments)

> **Comprehensive Configuration-Driven Testing Framework for UPI and Payment APIs**

A complete, plug-and-play testing solution that provides automated test case generation, execution, and reporting for UPI (Unified Payments Interface) services. Works as both a CLI tool and VS Code extension with full CI/CD pipeline integration.

## 🎯 Overview

The UPI Test Framework is designed to solve the challenge of comprehensive API testing for payment systems. It provides:

- **Automated Test Generation**: Configuration-driven test case creation
- **Multi-Layer Testing**: Unit, Integration, Functional, Load, and Security tests
- **Dual Interface**: CLI tool for pipelines + VS Code extension for development
- **Universal Design**: Extensible to IMPS, NEFT, RTGS, and other payment systems
- **Complete Automation**: From test generation to reporting

## 🏗️ Architecture

```
UTF
├── 🔧 Core Engine
│   ├── Configuration Parser/langage Adapter
│   ├── Test Case Generator
│   ├── Data Factory (Faker Integration)
│   └── Validation Engine
├── 🚀 Execution Layer
│   ├── Unit Test Runner
│   ├── Integration Test Runner
│   ├── Functional Test Runner
│   └── Load Test Runner
├── 🔌 Interfaces
│   ├── CLI Tool
│   ├── VS Code Extension
│   └── REST API Gateway
└── 📊 Reporting
    ├── HTML Reports
    ├── JSON Export
    ├── XML Output
    └── CI/CD Integration
```

## 📦 Installation

### CLI Tool
```bash
# Install globally via npm
curl -fsSL https://raw.gitlab.txninfra.com/utf/main/docs/installation/install.sh | bash

```
### build from Source:

```
git clone https://github.com/your-repo/utf
cd utf
cargo install --path .
```

### VS Code Extension
1. Open VS Code
2. Go to Extensions (Ctrl+Shift+X)
3. Search for "utf"
4. Click Install

### Docker
```bash
# Pull the Docker image
docker pull utf-framework:latest

# Run in container
docker run -v $(pwd):/workspace utf-framework:latest
```

## 🚀 Quick Start

### 1. Initialize Project

```bash
# Create new UPI test project
mkdir my-utfs && cd my-utfs

# Initialize configuration (interactive)
utf init

# Or create with template
utf init --template advanced
```

### 2. Configure Your API

The initialization creates `utf-config.yaml`:

```yaml
global_config:
  environment: "development"
  base_url: "https://your-api.com/v1"
  timeout: 30000
  retry_count: 3

endpoints:
  payment_initiate:
    path: "/payments/initiate"
    method: "POST"
    authentication: "bearer_token"
    rate_limit: 100

  payment_status:
    path: "/payments/{payment_id}/status"
    method: "GET"
    authentication: "bearer_token"

test_generation:
  unit_tests:
    enabled: true
    coverage_threshold: 85
    scenarios:
      payment_initiate:
        - name: "valid_payment_request"
          data_template: "valid_payment"
          expected_status: 200
        - name: "invalid_amount"
          data_template: "invalid_amount_payment"
          expected_status: 400
```

### 3. Generate and Run Tests (refined more)

```bash
# Generate test cases
utf generate --format jest --output ./tests

# Run all tests
utf run --coverage --report html

# Run specific test types
utf run --type unit --env staging
utf run --type integration --parallel
utf run --type functional --timeout 60000
```

🎯 Quick Reference Summary
# Supported Code Coverage Tools

| Language | **Best Tool** | **Installation** | **Basic Usage** | **Output Formats** | **Threshold Support** | **CI/CD Ready** |
|----------|---------------|------------------|-----------------|-------------------|---------------------|-----------------|
| **JavaScript** | Istanbul/NYC | `npm install --save-dev nyc` | `nyc jest` | HTML, LCOV, Text, JSON | ✅ | ✅ |
| **Java** | JaCoCo | Maven/Gradle Plugin | `mvn test` | HTML, XML, CSV | ✅ | ✅ |
| **TypeScript** | C8 | `npm install --save-dev c8` | `c8 jest` | HTML, LCOV, Text | ✅ | ✅ |
| **Python** | Coverage.py | `pip install coverage` | `coverage run -m pytest` | HTML, XML, Text | ✅ | ✅ |
| **Go** | Built-in | None (Go standard) | `go test -cover ./...` | HTML, Text | ⚠️ Manual | ✅ |
| **Rust** | Tarpaulin | `cargo install cargo-tarpaulin` | `cargo tarpaulin` | HTML, LCOV, XML | ✅ | ✅ |

---

## 📋 Configuration Reference

### Complete Configuration Structure

```yaml
# Global Settings
global_config:
  environment: "development"           # Target environment
  base_url: "https://api.example.com" # API base URL  
  timeout: 30000                      # Request timeout (ms)
  retry_count: 3                      # Retry failed tests
  parallel_execution: true            # Enable parallel testing
  test_data_source: "synthetic"       # synthetic, database, file

# API Endpoint Definitions
endpoints:
  payment_initiate:
    path: "/payments/initiate"
    method: "POST"
    authentication: "bearer_token"
    rate_limit: 100                    # Requests per minute
    
  payment_status:
    path: "/payments/{payment_id}/status"
    method: "GET" 
    authentication: "bearer_token"
    
  account_balance:
    path: "/accounts/{vpa}/balance"
    method: "GET"
    authentication: "bearer_token"
    
  transaction_history:
    path: "/transactions"
    method: "GET"
    authentication: "bearer_token"
    pagination: true

# Test Generation Rules
test_generation:
  unit_tests:
    enabled: true
    coverage_threshold: 85
    mock_external_services: true
    
    test_types:
      - input_validation
      - business_logic
      - error_handling
      - edge_cases
      
    scenarios:
      payment_initiate:
        - name: "valid_payment_request"
          description: "Test successful payment initiation"
          data_template: "valid_payment"
          expected_status: 200
          
        - name: "invalid_amount"
          description: "Test payment with invalid amount"
          data_template: "invalid_amount_payment"
          expected_status: 400
          expected_error: "INVALID_AMOUNT"
          
        - name: "insufficient_balance"
          description: "Test payment with insufficient balance"
          data_template: "insufficient_balance_payment"
          expected_status: 402
          expected_error: "INSUFFICIENT_BALANCE"

  integration_tests:
    enabled: true
    database_required: true
    external_services:
      - bank_service
      - notification_service
      - fraud_detection
      
    test_flows:
      complete_payment_flow:
        steps:
          - endpoint: "payment_initiate"
            data_template: "valid_payment"
            store_response: "payment_id"
            
          - endpoint: "payment_status"
            path_params:
              payment_id: "${payment_id}"
            wait_condition:
              field: "status"
              value: "completed"
              timeout: 30000
              
        validations:
          - database_state_check
          - external_service_calls
          - notification_sent

  functional_tests:
    enabled: true
    user_scenarios: true
    cross_platform: true
    
    user_journeys:
      peer_to_peer_payment:
        description: "Complete P2P payment journey"
        actors:
          - sender: "user1@paytm"
          - receiver: "user2@phonepe"
        steps:
          - login_sender
          - check_balance
          - initiate_payment
          - authenticate_payment
          - verify_receiver_notification
          - check_transaction_history

# Test Data Templates
data_templates:
  valid_payment:
    payer_vpa: "${faker.upi_vpa}"
    payee_vpa: "${faker.upi_vpa}"
    amount: "${faker.currency_amount(1, 10000)}"
    currency: "INR"
    description: "${faker.payment_description}"
    reference_id: "${faker.uuid}"
    
  invalid_amount_payment:
    extends: "valid_payment"
    amount: -100  # Invalid negative amount
    
  insufficient_balance_payment:
    extends: "valid_payment"
    amount: 999999  # Amount exceeding balance
    
  high_value_payment:
    extends: "valid_payment"
    amount: "${faker.currency_amount(200000, 500000)}"
    additional_auth_required: true

# Data Generation Rules
data_generation:
  faker_providers:
    upi_vpa:
      pattern: "${name}@${provider}"
      providers: ["paytm", "phonepe", "googlepay", "bhim"]
      
    payment_description:
      templates:
        - "Payment for ${product}"
        - "Bill payment - ${service}"
        - "Transfer to ${name}"
        
    currency_amount:
      decimal_places: 2
      constraints:
        min: 1
        max: 200000

# Validation Rules
validations:
  response_schema:
    payment_initiate_success:
      type: "object"
      required: ["payment_id", "status", "timestamp"]
      properties:
        payment_id:
          type: "string"
          pattern: "^[A-Z0-9]{12}$"
        status:
          type: "string"
          enum: ["initiated", "pending", "completed", "failed"]
        amount:
          type: "number"
          minimum: 0.01
          maximum: 200000
          
    error_response:
      type: "object"
      required: ["error_code", "error_message", "timestamp"]
      
  business_rules:
    payment_limits:
      daily_limit: 100000
      monthly_limit: 1000000
      single_transaction_limit: 200000
      
    validation_rules:
      - name: "vpa_format"
        field: "payer_vpa"
        regex: "^[a-zA-Z0-9._-]+@[a-zA-Z0-9.-]+$"
        
      - name: "amount_precision"
        field: "amount"
        decimal_places: 2

# Load Testing Configuration
load_testing:
  enabled: true
  scenarios:
    normal_load:
      users: 100
      duration: "10m"
      ramp_up: "2m"
      
    peak_load:
      users: 1000
      duration: "15m"
      ramp_up: "5m"
      
    stress_test:
      users: 2000
      duration: "20m"
      ramp_up: "10m"

# Security Testing
security_tests:
  enabled: true
  test_types:
    - authentication_bypass
    - authorization_checks
    - input_sanitization
    - sql_injection
    - xss_prevention
    
  sensitive_data_tests:
    - mask_vpa_in_logs
    - encrypt_payment_details
    - secure_token_handling

# Environment-specific Overrides
environments:
  development:
    base_url: "http://localhost:8080"
    database_cleanup: true
    mock_external_services: true
    
  staging:
    base_url: "https://staging-api.upi-service.com"
    database_cleanup: false
    mock_external_services: false
    
  production:
    base_url: "https://api.upi-service.com"
    test_types: ["smoke", "health_check"]
    load_testing:
      enabled: false
```

## 🔧 utf CLI Commands Reference (Incomplete )

- generate is command is alreday there.

### Basic Commands

```bash
# Initialize project
utf init                           # Interactive setup
utf init --template basic          # Use basic template
utf init --template advanced       # Use advanced template
utf init --template production     # Use production template

# Generate tests
utf generate                       # Generate with default settings

##### TODO Yet########
utf generate --format jest         # Generate Jest tests
utf generate --format postman      # Generate Postman collection
utf generate --format yaml         # Generate YAML test cases
utf generate --output ./tests      # Specify output directory
utf generate --verbose             # Verbose output

# Run tests
utf run                            # Run all tests
utf run --type unit                # Run only unit tests
utf run --type integration         # Run only integration tests
utf run --type functional          # Run only functional tests
utf run --type load                # Run load tests
utf run --type security            # Run security tests

# Environment and execution options
utf run --env development          # Run on development environment
utf run --env staging              # Run on staging environment
utf run --parallel                 # Enable parallel execution
utf run --no-parallel              # Disable parallel execution
utf run --timeout 60000            # Set timeout to 60 seconds
utf run --retries 5                # Set retry count to 5

# Coverage and reporting
utf run --coverage                 # Generate coverage report
utf run --coverage --threshold 90  # Set coverage threshold
utf run --report html              # Generate HTML report
utf run --report json              # Generate JSON report
utf run --report xml               # Generate XML report

# Validation and diagnostics
utf validate                       # Validate API endpoints
utf doctor                         # Diagnose environment
utf config --validate              # Validate configuration file

# Interactive and watch modes
utf interactive                    # Interactive test runner
utf watch                          # Watch mode for continuous testing

# Advanced features
utf export --format newman         # Export for Newman (Postman CLI)
utf export --format k6             # Export for K6 load testing
utf import --from postman          # Import from Postman collection
utf import --from swagger          # Import from OpenAPI/Swagger spec
```

### Pipeline Integration (to be done from pipeline)

```bash
# CI/CD friendly commands
utf run --ci                       # CI mode (no interactive prompts)
utf run --junit-output results.xml # Output JUnit XML for CI
utf run --exit-code                # Exit with error code on failure

# Docker usage
docker run -v $(pwd):/workspace utf-framework utf run --ci
```

## 🔌 VS Code Extension Features (given to Developer)

### Available Commands

| Command | Description | Example |
|---------|-------------|---------|
| `utf git-repo <url>` | Generate tests for entire Git repository | `utf git-repo https://github.com/user/repo.git` |
| `utf generate <file>` | Generate tests for a single file | `utf generate src/main.js --output tests/` |
| `utf analyze <file>` | Analyze code patterns | `utf analyze src/utils.py` |
| `utf languages` | List supported languages | `utf languages` |

### Extension Settings

```json
{
  "upiTest.autoRefresh": true,
  "upiTest.defaultTimeout": 30000,
  "upiTest.parallelExecution": true,
  "upiTest.coverageThreshold": 85,
  "upiTest.reportFormat": "html",
  "upiTest.debugMode": false
}
```

### Test Explorer Integration

The extension provides a dedicated Test Explorer view showing:
- 📁 Unit Tests
  - ✅ Valid Payment Request
  - ❌ Invalid Amount Test
  - ⏭️ Insufficient Balance Test
- 📁 Integration Tests
  - 🔄 Complete Payment Flow
  - 🔄 External Service Integration
- 📁 Functional Tests
  - 👤 User Payment Journey
