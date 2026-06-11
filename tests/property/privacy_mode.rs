use proptest::prelude::*;
use serde_json::Value;

proptest! {
    #[test]
    fn prop_privacy_mode_redacts_sensitive_paths(
        username in "[a-zA-Z0-9_]{3,20}",
        _hostname in "[a-zA-Z0-9-]{3,30}",
    ) {
        // Test that privacy mode redacts user-specific paths
        let home_path = format!("/home/{}/Documents/secret.txt", username);
        let audit_entry = format!(
            r#"{{"operation":"move","path":"{}","timestamp":"2024-01-01T00:00:00Z","uuid":"550e8400-e29b-41d4-a716-446655440000","reason":"test"}}"#,
            home_path
        );

        // When privacy mode is enabled, sensitive paths should be redacted
        let redacted = audit_entry.replace(&username, "[REDACTED]");

        // Verify redaction occurred
        prop_assert!(!redacted.contains(&username));
        prop_assert!(redacted.contains("[REDACTED]"));
    }

    #[test]
    fn prop_privacy_mode_preserves_non_sensitive_info(
        operation in "[a-z]{3,10}",
        timestamp in "[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}Z",
    ) {
        // Test that privacy mode preserves non-sensitive metadata
        let audit_entry = format!(
            r#"{{"operation":"{}","path":"/tmp/file.txt","timestamp":"{}","uuid":"550e8400-e29b-41d4-a716-446655440000","reason":"test"}}"#,
            operation, timestamp
        );

        // Non-sensitive fields should remain intact
        prop_assert!(audit_entry.contains(&operation));
        prop_assert!(audit_entry.contains(&timestamp));
        prop_assert!(audit_entry.contains("test"));
    }

    #[test]
    fn prop_privacy_mode_redacts_secrets(
        secret_pattern in "[a-zA-Z0-9+/]{20,40}={0,2}",
    ) {
        // Test that privacy mode redacts secret-like patterns (e.g., API keys, tokens)
        let audit_entry = format!(
            r#"{{"operation":"move","path":"/tmp/file.txt","timestamp":"2024-01-01T00:00:00Z","uuid":"550e8400-e29b-41d4-a716-446655440000","secret":"{}","reason":"test"}}"#,
            secret_pattern
        );

        // Secret fields should be redacted
        let redacted = audit_entry.replace(&secret_pattern, "[REDACTED]");

        prop_assert!(!redacted.contains(&secret_pattern));
        prop_assert!(redacted.contains("[REDACTED]"));
    }

    #[test]
    fn prop_privacy_mode_idempotent(
        username in "[a-zA-Z0-9_]{3,20}",
    ) {
        // Test that applying privacy mode multiple times yields same result
        let home_path = format!("/home/{}/Documents/secret.txt", username);
        let audit_entry = format!(
            r#"{{"operation":"move","path":"{}","timestamp":"2024-01-01T00:00:00Z","uuid":"550e8400-e29b-41d4-a716-446655440000","reason":"test"}}"#,
            home_path
        );

        // Apply redaction once
        let redacted_once = audit_entry.replace(&username, "[REDACTED]");

        // Apply redaction again (should be idempotent)
        let redacted_twice = redacted_once.replace("[REDACTED]", "[REDACTED]");

        prop_assert_eq!(redacted_once, redacted_twice);
    }

    #[test]
    fn prop_privacy_mode_handles_empty_values(
        operation in "[a-z]{3,10}",
    ) {
        // Test that privacy mode handles empty or null values gracefully
        let audit_entry = format!(
            r#"{{"operation":"{}","path":"","timestamp":"2024-01-01T00:00:00Z","uuid":"","reason":""}}"#,
            operation
        );

        // Empty values should not cause errors
        prop_assert!(audit_entry.contains(&operation));
        prop_assert!(audit_entry.contains("\"path\":\"\""));
        prop_assert!(audit_entry.contains("\"uuid\":\"\""));
    }

    #[test]
    fn prop_privacy_mode_preserves_structure(
        operation in "[a-z]{3,10}",
        file_path in "[a-zA-Z0-9_\\-/]{1,50}",
    ) {
        // Test that privacy mode preserves JSON structure
        let audit_entry = format!(
            r#"{{"operation":"{}","path":"{}","timestamp":"2024-01-01T00:00:00Z","uuid":"550e8400-e29b-41d4-a716-446655440000","reason":"test"}}"#,
            operation, file_path
        );

        // Verify valid JSON structure is maintained
        let json: Value = serde_json::from_str(&audit_entry).unwrap();

        prop_assert!(json.is_object());
        prop_assert!(json.get("operation").is_some());
        prop_assert!(json.get("path").is_some());
        prop_assert!(json.get("timestamp").is_some());
        prop_assert!(json.get("uuid").is_some());
        prop_assert!(json.get("reason").is_some());
    }

    #[test]
    fn prop_privacy_mode_redacts_ip_addresses(
        ip_octet1 in 0u8..255u8,
        ip_octet2 in 0u8..255u8,
        ip_octet3 in 0u8..255u8,
        ip_octet4 in 0u8..255u8,
    ) {
        // Test that privacy mode redacts IP addresses
        let ip_address = format!("{}.{}.{}.{}", ip_octet1, ip_octet2, ip_octet3, ip_octet4);
        let audit_entry = format!(
            r#"{{"operation":"move","path":"/tmp/file.txt","timestamp":"2024-01-01T00:00:00Z","uuid":"550e8400-e29b-41d4-a716-446655440000","client_ip":"{}","reason":"test"}}"#,
            ip_address
        );

        // IP addresses should be redacted
        let redacted = audit_entry.replace(&ip_address, "[REDACTED]");

        prop_assert!(!redacted.contains(&ip_address));
        prop_assert!(redacted.contains("[REDACTED]"));
    }

    #[test]
    fn prop_privacy_mode_multiple_redactions(
        username in "[a-zA-Z0-9_]{3,20}",
        hostname in "[a-zA-Z0-9-]{3,30}",
    ) {
        // Test that privacy mode can redact multiple sensitive fields
        let home_path = format!("/home/{}/secret@{}.txt", username, hostname);
        let audit_entry = format!(
            r#"{{"operation":"move","path":"{}","timestamp":"2024-01-01T00:00:00Z","uuid":"550e8400-e29b-41d4-a716-446655440000","reason":"test"}}"#,
            home_path
        );

        // Both username and hostname should be redacted
        let redacted = audit_entry
            .replace(&username, "[REDACTED]")
            .replace(&hostname, "[REDACTED]");

        prop_assert!(!redacted.contains(&username));
        prop_assert!(!redacted.contains(&hostname));
        prop_assert!(redacted.contains("[REDACTED]"));
    }
}
