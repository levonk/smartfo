# Smartfo Test Framework

This directory contains the test framework for smartfo, including unit tests, integration tests, property tests, and test utilities.

## Test Organization

```
tests/
├── cli_standards_utils.rs      # CLI standards test utilities
├── cli_tests.rs                # Basic CLI tests
├── integration_tests/          # Integration tests
│   ├── mod.rs
│   ├── fixtures.rs             # Test fixtures
│   ├── mv_scenarios.rs         # Move scenario tests
│   ├── cross_device.rs         # Cross-device move tests
│   ├── async_mv.rs             # Async move tests
│   ├── async_rm.rs             # Async remove tests
│   ├── crash_recovery.rs       # Crash recovery tests
│   ├── overwrite_modes.rs     # Overwrite mode tests
│   ├── install.rs              # Installation tests
│   ├── force_delete.rs         # Force delete tests
│   ├── trash_mode.rs           # Trash mode tests
│   └── ...
├── property/                   # Property-based tests
│   ├── mod.rs
│   ├── audit_validity.rs       # Audit log validity tests
│   ├── disk_space_culling.rs   # Disk space culling tests
│   ├── force_delete.rs         # Force delete property tests
│   ├── hook_detection.rs       # Hook detection tests
│   ├── install_idempotency.rs  # Installation idempotency tests
│   ├── no_data_loss.rs         # No data loss property tests
│   ├── same_file_history.rs    # Same file history tests
│   ├── trash_mode.rs           # Trash mode property tests
│   └── trash_preserve.rs       # Trash preservation tests
└── *.rs                        # Other test modules
```

## CLI Standards Test Utilities

The `cli_standards_utils.rs` module provides helper functions for testing CLI standards compliance. These utilities are designed to be reusable across all test modules.

### Binary Creation

- `smartfo_cmd()` - Create a Command instance for smartfo
- `smartfo_symlink_cmd(name, tmp_dir)` - Create a Command instance for a symlinked binary
- `symlink_binary(name)` - Create a symlink to the smartfo binary

### Assertion Helpers

- `assert_success_with_code(assert, expected_code)` - Assert command succeeds with specific exit code
- `assert_failure_with_code(assert, expected_code)` - Assert command fails with specific exit code
- `assert_stdout_contains(assert, expected)` - Assert stdout contains substring
- `assert_stderr_contains(assert, expected)` - Assert stderr contains substring
- `assert_stdout_matches(assert, pattern)` - Assert stdout matches regex pattern
- `assert_stderr_matches(assert, pattern)` - Assert stderr matches regex pattern

### Output Format Validation

- `assert_valid_json(assert)` - Assert output is valid JSON
- `assert_json_field(assert, field, expected)` - Assert JSON field has expected value
- `assert_json_has_field(assert, field)` - Assert JSON contains field
- `assert_toon_format(assert)` - Assert output is in TOON format (compact, token-efficient)
- `assert_human_format(assert)` - Assert output is in human-readable format

### Flag Testing

- `test_flag_acceptance(flag, expected_in_output)` - Test that a CLI flag is accepted
- `test_flag_combination(flags, expected_in_output)` - Test that flag combination is accepted
- `test_invalid_flag_rejection(flag)` - Test that invalid flag is rejected
- `test_missing_flag_value(flag)` - Test that flag without value is rejected

### Command Behavior

- `assert_dry_run_no_changes(cmd, original_state)` - Assert --dry-run makes no changes
- `assert_verbose_output(cmd)` - Assert --verbose produces detailed output
- `assert_quiet_output(cmd)` - Assert --quiet produces minimal output
- `assert_json_output(cmd)` - Assert --json produces JSON output

### Feature Testing

- `test_daemon_mode(cmd)` - Test daemon mode operations
- `test_health_check()` - Test health check endpoint
- `test_privacy_mode(cmd)` - Test privacy mode behavior
- `test_config_reload()` - Test config reload behavior (SIGHUP)
- `test_session_hooks()` - Test session hooks output
- `test_skill_generation()` - Test skill generation
- `test_cross_platform_path(path)` - Test cross-platform path handling
- `test_tui_mode()` - Test TUI mode interactions

### Test Fixtures

- `temp_dir()` - Create a temporary directory
- `temp_file_with_content(dir, name, content)` - Create a temporary file with content
- `temp_dir_structure(dir, structure)` - Create a temporary directory structure

### Usage Example

```rust
use crate::cli_standards_utils::*;

#[test]
fn test_help_flag() {
    let mut cmd = smartfo_cmd();
    cmd.arg("--help");
    let assert = cmd.assert().success();
    assert_stdout_contains(assert, "Usage:");
}

#[test]
fn test_json_output() {
    let mut cmd = smartfo_cmd();
    cmd.arg("--json");
    assert_json_output(&mut cmd);
}

#[test]
fn test_flag_combination() {
    test_flag_combination(&["--verbose", "--json"], Some("operation"));
}
```

## Running Tests

### Run All Tests

```bash
devbox run cargo test
```

### Run Specific Test Module

```bash
devbox run cargo test --test cli_standards_utils
devbox run cargo test --test integration_tests
devbox run cargo test --test property
```

### Run Specific Test

```bash
devbox run cargo test test_name
```

### Run Tests with Output

```bash
devbox run cargo test -- --nocapture
```

### Run Tests in Release Mode

```bash
devbox run cargo test --release
```

## Property-Based Testing

Property tests use the `proptest` crate to test invariants across many random inputs. These tests provide safety guarantees by verifying that certain properties always hold true.

Property tests are located in the `tests/property/` directory and cover:

- Audit log validity
- Disk space culling
- Force delete behavior
- Hook detection
- Installation idempotency
- No data loss guarantees
- Same file deletion history
- Trash mode behavior
- Trash preservation

## Integration Testing

Integration tests are located in `tests/integration_tests/` and test end-to-end scenarios including:

- Move scenarios (tracked/untracked, cross-repo, cross-device)
- Async operations (move and remove)
- Crash recovery
- Overwrite modes
- Installation and symlinks
- Git hooks
- Cross-platform behavior

## Test Coverage

The test framework aims for high coverage of:

- CLI flag parsing and validation
- Output format correctness (JSON, TOON, human)
- Exit code behavior
- Daemon operations
- VCS integration (Git, Mercurial, SVN, Jujutsu)
- Trash operations
- Privacy mode
- Config reload
- Session hooks
- Skill generation
- Cross-platform path handling

## Adding New Tests

When adding new tests:

1. Use the CLI standards utilities from `cli_standards_utils.rs` when applicable
2. Follow the existing test organization structure
3. Add unit tests for new test utilities
4. Update this README with new test categories
5. Ensure tests are deterministic and don't depend on external state
6. Use temporary directories and files for filesystem tests
7. Clean up resources in test teardown

## Test Dependencies

The test framework uses:

- `assert_cmd` - Command line testing
- `predicates` - Predicate-based assertions
- `tempfile` - Temporary file/directory management
- `proptest` - Property-based testing
- `serde_json` - JSON parsing and validation

## CI/CD Integration

Tests run automatically in CI on:

- Linux
- macOS
- Windows

The CI matrix is configured in `.github/workflows/ci.yml`.
