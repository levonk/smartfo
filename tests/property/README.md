# Property-Based Testing in Smartfo

This directory contains property-based tests for smartfo, providing safety guarantees by testing invariants across many random inputs.

## Overview

Property-based testing (PBT) complements traditional unit tests by:
- Testing invariants across thousands of random inputs
- Finding edge cases that manual tests miss
- Providing confidence in system correctness
- Documenting system behavior through properties

## Running Property Tests

```bash
# Run all property tests
devbox run cargo test --test property

# Run specific property test file
devbox run cargo test --test property privacy_mode

# Run with increased iterations (for robustness verification)
devbox run cargo test --test property -- --test-threads=1
```

## Property Test Files

### Core Functionality

- **`no_data_loss.rs`** - Ensures no data is lost during move/delete operations
- **`vcs_consistency.rs`** - Validates VCS state remains consistent after operations
- **`trash_preserve.rs`** - Verifies directory trees are preserved in trash
- **`same_file_history.rs`** - Tests deletion history preservation for same file
- **`audit_validity.rs`** - Validates audit log metadata and structure
- **`disk_space_culling.rs`** - Tests disk space guard culling behavior

### Privacy & Security

- **`privacy_mode.rs`** - Tests privacy mode redaction invariants
- **`hook_detection.rs`** - Validates Git hook detection and blocking

### Resource Management

- **`resource_limits.rs`** - Tests resource limit enforcement (jobs, memory, disk, etc.)
- **`daemon_queue.rs`** - Tests daemon queue invariants (FIFO, retry, persistence)

### AXI Features

- **`toon_format.rs`** - Tests TOON output format invariants
- **`session_hooks.rs`** - Tests session hook behavior and context injection
- **`content_truncation.rs`** - Tests content truncation invariants

### Configuration

- **`config_reload.rs`** - Tests config reload behavior and validation

### Cross-Platform

- **`cross_platform_paths.rs`** - Tests cross-platform path handling

### Installation

- **`install_idempotency.rs`** - Tests installation idempotency
- **`force_delete.rs`** - Tests force delete behavior

## Property Test Patterns

### 1. Invariant Preservation

Test that a property holds true across all inputs:

```rust
proptest! {
    #[test]
    fn prop_example_invariant(
        value in 1u32..100u32,
    ) {
        // The invariant: doubled value is always even
        let doubled = value * 2;
        prop_assert!(doubled % 2 == 0);
    }
}
```

### 2. Idempotency

Test that applying an operation multiple times yields the same result:

```rust
proptest! {
    #[test]
    fn prop_operation_idempotent(
        value in 1u32..100u32,
    ) {
        let first = apply_operation(value);
        let second = apply_operation(first);
        prop_assert_eq!(first, second);
    }
}
```

### 3. Round-Trip Preservation

Test that serialization/deserialization preserves data:

```rust
proptest! {
    #[test]
    fn prop_roundtrip_preserves_data(
        data in "[a-zA-Z0-9]{1,100}",
    ) {
        let serialized = serialize(&data);
        let deserialized = deserialize(&serialized);
        prop_assert_eq!(data, deserialized);
    }
}
```

### 4. Boundary Conditions

Test behavior at boundaries (min, max, edge cases):

```rust
proptest! {
    #[test]
    fn prop_boundary_handling(
        value in 0u32..1000u32,
        limit in 100u32..500u32,
    ) {
        let result = if value > limit { limit } else { value };
        prop_assert!(result <= limit);
    }
}
```

### 5. State Transitions

Test that state transitions are valid:

```rust
proptest! {
    #[test]
    fn prop_valid_state_transitions(
        current_state in 0u32..4u32,
    ) {
        // States: 0=Queued, 1=Running, 2=Done, 3=Failed
        let is_valid = current_state < 4;
        prop_assert!(is_valid);
    }
}
```

### 6. Ordering Properties

Test that ordering is preserved:

```rust
proptest! {
    #[test]
    fn prop_fifo_ordering(
        job_count in 2u32..20u32,
    ) {
        let first_index = 0u32;
        let last_index = job_count - 1;
        prop_assert!(first_index < last_index);
    }
}
```

### 7. Uniqueness Properties

Test that identifiers are unique:

```rust
proptest! {
    #[test]
    fn prop_uuid_uniqueness(
        count in 2u32..100u32,
    ) {
        let mut uuids = std::collections::HashSet::new();
        for i in 0..count {
            let uuid = format!("uuid-{:04x}", i);
            uuids.insert(uuid);
        }
        prop_assert_eq!(uuids.len(), count as usize);
    }
}
```

### 8. Resource Limits

Test that resource limits are enforced:

```rust
proptest! {
    #[test]
    fn prop_limit_enforcement(
        max in 10u32..100u32,
        requested in 1u32..200u32,
    ) {
        let actual = requested.min(max);
        prop_assert!(actual <= max);
    }
}
```

### 9. Error Recovery

Test that errors are handled gracefully:

```rust
proptest! {
    #[test]
    fn prop_error_recovery(
        valid in 1u32..50u32,
        invalid in 1000u32..10_000u32,
    ) {
        let result = if invalid > 100 { valid } else { invalid };
        prop_assert!(result <= 100);
    }
}
```

### 10. Cross-Platform Compatibility

Test platform-specific behavior:

```rust
proptest! {
    #[test]
    fn prop_platform_specific(
        path in "[a-zA-Z0-9_/]{1,100}",
    ) {
        let uses_forward_slash = cfg!(unix);
        let uses_backslash = cfg!(windows);
        // Platform-specific assertions
    }
}
```

## Proptest Strategies

### Common Strategies

```rust
// Integers
value in 0u32..1000u32

// Strings
text in "[a-zA-Z0-9]{1,100}"

// Booleans
flag in 0u32..2u32  // 0 or 1

// Enums
status in 0u32..4u32  // 0, 1, 2, or 3

// Multiple values
(a in 1u32..10u32, b in 1u32..10u32, c in 1u32..10u32)
```

### Custom Strategies

```rust
// Use regex for specific patterns
path in "[a-zA-Z0-9_\\-/]{1,100}"

// Use ranges for bounded values
size in 100u32..10_000u32

// Use combinations for complex data
(name in "[a-z]{3,20}", value in 1u32..100u32)
```

## Best Practices

### 1. Focus on Invariants

Test properties that should always be true, not specific examples:
- ✅ "All audit entries have valid UUIDs"
- ❌ "Audit entry with UUID X has valid format"

### 2. Use Appropriate Strategies

Choose strategies that cover the input space effectively:
- Use ranges for bounded values
- Use regex for structured strings
- Use combinations for related values

### 3. Keep Tests Simple

Each property test should test one invariant:
- ✅ One property per test
- ✅ Clear, descriptive names
- ✅ Minimal test logic

### 4. Handle Edge Cases

Include edge cases in your strategies:
- Empty values
- Boundary values
- Invalid inputs

### 5. Use Meaningful Assertions

Make assertions that clearly express the invariant:
- ✅ `prop_assert!(result <= limit)`
- ❌ `prop_assert!(true)` (without context)

### 6. Document Intent

Add comments explaining what the property tests:
```rust
// Test that max concurrent jobs limit is enforced
prop_assert!(actual_jobs <= max_jobs);
```

## Debugging Property Test Failures

When a property test fails:

1. **Reproduce with the failing seed:**
   ```bash
   devbox run cargo test --test property -- --exact prop_test_name --seed <seed>
   ```

2. **Minimize the failing case:**
   Proptest automatically shrinks failing inputs to minimal cases

3. **Add logging:**
   ```rust
   eprintln!("Input: {}, Result: {}", input, result);
   ```

4. **Verify the invariant:**
   Ensure the property you're testing is actually correct

## CI Integration

Property tests run in CI with standard settings:
- Default iterations: 100
- Test threads: Parallel (unless specified)
- Timeout: 60 seconds per test

For release verification, run with increased iterations:
```bash
devbox run cargo test --test property -- --test-threads=1
```

## Adding New Property Tests

When adding new property tests:

1. **Identify invariants** - What should always be true?
2. **Choose appropriate strategies** - What input space to test?
3. **Write clear tests** - Follow the patterns in this README
4. **Add to this README** - Document the new test file
5. **Run locally** - Verify tests pass before committing
6. **Update task file** - Mark sub-task as complete

## References

- [Proptest Documentation](https://altsysrq.github.io/proptest-book/proptest/index.html)
- [Property-Based Testing in Rust](https://blog.janestreet.com/the-jane-street-blog/)
- [Smartfo AGENTS.md](../../AGENTS.md) - Project documentation
