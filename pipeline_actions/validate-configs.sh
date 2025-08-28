#!/bin/bash

# Validation script for CI/CD pipeline configurations
# This script validates the syntax and structure of the pipeline files

set -e

echo "🔍 Validating CI/CD Pipeline Configurations..."
echo "=============================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to check if a file exists
check_file() {
    if [ -f "$1" ]; then
        echo -e "${GREEN}✓${NC} Found: $1"
        return 0
    else
        echo -e "${RED}✗${NC} Missing: $1"
        return 1
    fi
}

# Function to validate YAML syntax
validate_yaml() {
    local file="$1"
    if command -v yq >/dev/null 2>&1; then
        if yq eval '.' "$file" >/dev/null 2>&1; then
            echo -e "${GREEN}✓${NC} Valid YAML syntax: $file"
            return 0
        else
            echo -e "${RED}✗${NC} Invalid YAML syntax: $file"
            return 1
        fi
    elif command -v python3 >/dev/null 2>&1; then
        if python3 -c "import yaml; yaml.safe_load(open('$file'))" >/dev/null 2>&1; then
            echo -e "${GREEN}✓${NC} Valid YAML syntax: $file"
            return 0
        else
            echo -e "${RED}✗${NC} Invalid YAML syntax: $file"
            return 1
        fi
    else
        echo -e "${YELLOW}⚠${NC} Cannot validate YAML syntax (yq or python3 not found): $file"
        return 0
    fi
}

# Function to check for required sections in GitHub Actions
validate_github_action() {
    local file="$1"
    echo "Validating GitHub Actions file: $file"
    
    if grep -q "name:" "$file" && 
       grep -q "runs:" "$file" && 
       grep -q "using:" "$file"; then
        echo -e "${GREEN}✓${NC} GitHub Action structure looks good"
        return 0
    else
        echo -e "${RED}✗${NC} Missing required GitHub Action sections"
        return 1
    fi
}

# Function to check for required sections in GitLab CI
validate_gitlab_ci() {
    local file="$1"
    echo "Validating GitLab CI file: $file"
    
    if grep -q "stages:" "$file" || grep -q "script:" "$file"; then
        echo -e "${GREEN}✓${NC} GitLab CI structure looks good"
        return 0
    else
        echo -e "${RED}✗${NC} Missing required GitLab CI sections"
        return 1
    fi
}

# Function to check for required sections in Azure Pipelines
validate_azure_pipeline() {
    local file="$1"
    echo "Validating Azure Pipeline file: $file"
    
    if grep -q "stages:" "$file" && 
       grep -q "jobs:" "$file"; then
        echo -e "${GREEN}✓${NC} Azure Pipeline structure looks good"
        return 0
    else
        echo -e "${RED}✗${NC} Missing required Azure Pipeline sections"
        return 1
    fi
}

# Main validation
echo
echo "📁 Checking file structure..."
files_ok=true

# GitHub files
check_file "github/action.yml" || files_ok=false
check_file "github/unified-testing.yml" || files_ok=false  
check_file "github/example-usage.yml" || files_ok=false

# GitLab files
check_file "gitlab/.gitlab-ci.yml" || files_ok=false
check_file "gitlab/unified-testing-template.yml" || files_ok=false
check_file "gitlab/example-usage.yml" || files_ok=false

# Azure files
check_file "azure/azure-pipelines.yml" || files_ok=false
check_file "azure/unified-testing-template.yml" || files_ok=false

# Documentation
check_file "README.md" || files_ok=false

echo
echo "🔍 Validating YAML syntax..."
yaml_ok=true

for file in github/*.yml gitlab/*.yml azure/*.yml; do
    if [ -f "$file" ]; then
        validate_yaml "$file" || yaml_ok=false
    fi
done

echo
echo "📋 Validating pipeline structures..."
structure_ok=true

# Validate GitHub Action
if [ -f "github/action.yml" ]; then
    validate_github_action "github/action.yml" || structure_ok=false
fi

# Validate GitLab CI
if [ -f "gitlab/.gitlab-ci.yml" ]; then
    validate_gitlab_ci "gitlab/.gitlab-ci.yml" || structure_ok=false
fi

# Validate Azure Pipeline
if [ -f "azure/azure-pipelines.yml" ]; then
    validate_azure_pipeline "azure/azure-pipelines.yml" || structure_ok=false
fi

echo
echo "🔧 Checking for required keywords..."
keywords_ok=true

# Check for unified-testing references
if ! grep -r "unified-testing" github/ gitlab/ azure/ >/dev/null 2>&1; then
    echo -e "${RED}✗${NC} No 'unified-testing' references found"
    keywords_ok=false
else
    echo -e "${GREEN}✓${NC} Found 'unified-testing' references"
fi

# Check for supported languages
languages=("javascript" "python" "rust")
for lang in "${languages[@]}"; do
    if grep -r "$lang" github/ gitlab/ azure/ >/dev/null 2>&1; then
        echo -e "${GREEN}✓${NC} Found support for: $lang"
    else
        echo -e "${YELLOW}⚠${NC} Limited support found for: $lang"
    fi
done

echo
echo "📊 Validation Summary"
echo "===================="

if $files_ok && $yaml_ok && $structure_ok && $keywords_ok; then
    echo -e "${GREEN}🎉 All validations passed!${NC}"
    echo
    echo "Your CI/CD pipeline configurations are ready to use:"
    echo "• Copy the appropriate files to your repositories"
    echo "• Customize variables/parameters as needed"  
    echo "• Test with a small repository first"
    exit 0
else
    echo -e "${RED}❌ Some validations failed${NC}"
    echo
    echo "Issues found:"
    $files_ok || echo "• Missing required files"
    $yaml_ok || echo "• YAML syntax errors"
    $structure_ok || echo "• Invalid pipeline structure"
    $keywords_ok || echo "• Missing required keywords"
    echo
    echo "Please fix the issues above before using these configurations."
    exit 1
fi