#!/usr/bin/env bash
# Platform-specific test environment setup for Unix platforms (Linux, macOS)
# This script sets up the test environment for cross-platform testing

set -euo pipefail

# Create test directories
mkdir -p /tmp/smartfo-test
mkdir -p /tmp/smartfo-test/sockets
mkdir -p /tmp/smartfo-test/pids
mkdir -p /tmp/smartfo-test/trash

# Set environment variables for testing
export SMARTFO_TEST_MODE=1
export SMARTFO_TRASH_ROOT=/tmp/smartfo-test/trash
export SMARTFO_PATHS_AUDIT_LOG=/tmp/smartfo-test/audit.log

echo "Unix test environment setup complete"
echo "Test directories created in /tmp/smartfo-test"
