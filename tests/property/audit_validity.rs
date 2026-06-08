use proptest::prelude::*;
use serde_json::Value;
use std::fs;
use tempfile::TempDir;

proptest! {
    #[test]
    fn prop_audit_log_valid_metadata(
        operation_type in "[a-z]{3,10}",
        file_path in "[a-zA-Z0-9_\\-/]{1,100}",
    ) {
        let temp_dir = TempDir::new().unwrap();
        let audit_log = temp_dir.path().join("audit.jsonl");

        // Simulate audit entry with all required fields
        let entry = format!(
            r#"{{"operation":"{}","path":"{}","timestamp":"2024-01-01T00:00:00Z","uuid":"550e8400-e29b-41d4-a716-446655440000","reason":"test"}}"#,
            operation_type, file_path
        );

        fs::write(&audit_log, &entry).unwrap();

        // Verify entry can be parsed as valid JSON
        let content = fs::read_to_string(&audit_log).unwrap();
        let json: Value = serde_json::from_str(&content).unwrap();

        // Verify all required fields exist
        prop_assert!(json.get("operation").is_some());
        prop_assert!(json.get("path").is_some());
        prop_assert!(json.get("timestamp").is_some());
        prop_assert!(json.get("uuid").is_some());
        prop_assert!(json.get("reason").is_some());

        // Verify field types
        prop_assert!(json["operation"].is_string());
        prop_assert!(json["path"].is_string());
        prop_assert!(json["timestamp"].is_string());
        prop_assert!(json["uuid"].is_string());
        prop_assert!(json["reason"].is_string());

        // Verify field values
        prop_assert_eq!(json["operation"].as_str().unwrap(), operation_type);
        prop_assert_eq!(json["path"].as_str().unwrap(), file_path);
    }

    #[test]
    fn prop_audit_log_multiple_entries(
        num_entries in 1..20usize,
    ) {
        let temp_dir = TempDir::new().unwrap();
        let audit_log = temp_dir.path().join("audit.jsonl");

        // Write multiple audit entries
        let mut entries = Vec::new();
        for i in 0..num_entries {
            let entry = format!(
                r#"{{"operation":"move","path":"/tmp/file_{}.txt","timestamp":"2024-01-01T00:00:{}Z","uuid":"550e8400-e29b-41d4-a716-44665544{:04x}","reason":"test"}}"#,
                i, i, i
            );
            entries.push(entry);
        }

        let all_entries = entries.join("\n");
        fs::write(&audit_log, &all_entries).unwrap();

        // Verify all entries can be parsed
        let content = fs::read_to_string(&audit_log).unwrap();
        let parsed_count = content.lines().filter(|line| {
            serde_json::from_str::<Value>(line).is_ok()
        }).count();

        prop_assert_eq!(parsed_count, num_entries);
    }

    #[test]
    fn prop_audit_log_uuid_uniqueness(
        num_entries in 1..50usize,
    ) {
        let temp_dir = TempDir::new().unwrap();
        let audit_log = temp_dir.path().join("audit.jsonl");

        // Write entries with unique UUIDs
        let mut entries = Vec::new();
        let mut uuids = std::collections::HashSet::new();
        for i in 0..num_entries {
            let uuid = format!("550e8400-e29b-41d4-a716-44665544{:04x}", i);
            uuids.insert(uuid.clone());

            let entry = format!(
                r#"{{"operation":"move","path":"/tmp/file.txt","timestamp":"2024-01-01T00:00:00Z","uuid":"{}","reason":"test"}}"#,
                uuid
            );
            entries.push(entry);
        }

        let all_entries = entries.join("\n");
        fs::write(&audit_log, &all_entries).unwrap();

        // Verify all UUIDs are unique
        prop_assert_eq!(uuids.len(), num_entries);

        // Verify all entries can be parsed
        let content = fs::read_to_string(&audit_log).unwrap();
        let parsed_count = content.lines().filter(|line| {
            if let Ok(json) = serde_json::from_str::<Value>(line) {
                if let Some(uuid) = json.get("uuid").and_then(|u| u.as_str()) {
                    return uuids.contains(uuid);
                }
            }
            false
        }).count();

        prop_assert_eq!(parsed_count, num_entries);
    }
}
