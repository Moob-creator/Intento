#!/bin/bash
# Integration Test Runner for Intento
# This script runs all integration tests with proper configuration

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}=== Intento Integration Test Suite ===${NC}\n"

# Check for .env file
if [ ! -f "../.env" ]; then
    echo -e "${YELLOW}Warning: .env file not found. AI-related tests will be skipped.${NC}"
    echo -e "${YELLOW}To enable AI tests, create a .env file with API keys.${NC}\n"
fi

# Function to run a test category
run_test_category() {
    local category=$1
    local test_file=$2
    local flags=$3

    echo -e "${GREEN}Running ${category} tests...${NC}"
    if cargo test --test "$test_file" $flags -- --test-threads=1 --nocapture; then
        echo -e "${GREEN}✓ ${category} tests passed${NC}\n"
    else
        echo -e "${RED}✗ ${category} tests failed${NC}\n"
        return 1
    fi
}

# Run comprehensive integration tests
echo -e "${GREEN}1. Comprehensive Integration Tests${NC}"
echo "   - Task CRUD operations"
echo "   - Summary generation"
echo "   - Settings management"
echo "   - Error handling"
echo "   - Performance tests"
run_test_category "Comprehensive" "test_comprehensive_integration" ""

# Run notification integration tests
echo -e "${GREEN}2. Notification Integration Tests${NC}"
echo "   - Task reminders"
echo "   - Expiring task notifications"
echo "   - Notification settings"
run_test_category "Notification" "test_notification_integration" ""

# Run command integration tests
echo -e "${GREEN}3. Tauri Command Integration Tests${NC}"
echo "   - Task commands"
echo "   - Settings commands"
echo "   - Error handling"
echo "   - Concurrent operations"
run_test_category "Command" "test_command_integration" ""

# Run AI integration tests (if .env exists)
if [ -f "../.env" ]; then
    echo -e "${GREEN}4. AI Integration Tests (requires API keys)${NC}"
    echo "   - Text parsing"
    echo "   - Image parsing"
    echo "   - Summary generation"
    echo -e "${YELLOW}Note: These tests make actual API calls and may incur costs${NC}"

    read -p "Run AI integration tests? (y/N) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        cargo test --test test_comprehensive_integration -- --ignored --test-threads=1 --nocapture || echo -e "${YELLOW}Some AI tests may have failed (check API keys)${NC}"
    else
        echo -e "${YELLOW}Skipping AI integration tests${NC}"
    fi
else
    echo -e "${YELLOW}4. AI Integration Tests - SKIPPED (no .env file)${NC}"
fi

echo -e "\n${GREEN}=== Test Summary ===${NC}"
echo "Run 'cargo test --all-targets' to run all tests including unit tests"
echo "Run 'cargo test --test <test_name> -- --nocapture' for verbose output"
echo "Run 'cargo test --test <test_name> -- --ignored' to run ignored (AI) tests"

echo -e "\n${GREEN}Integration test suite completed!${NC}"
