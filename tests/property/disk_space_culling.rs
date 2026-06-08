use proptest::prelude::*;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

fn get_entry_timestamps(trash_dir: &Path) -> Vec<(String, std::time::SystemTime)> {
    let mut timestamps = Vec::new();
    if let Ok(entries) = fs::read_dir(trash_dir) {
        for entry in entries.flatten() {
            if let Ok(metadata) = entry.metadata() {
                if let Ok(modified) = metadata.modified() {
                    timestamps.push((entry.file_name().to_string_lossy().to_string(), modified));
                }
            }
        }
    }
    timestamps.sort_by_key(|(_, time)| *time);
    timestamps
}

proptest! {
    #[test]
    fn prop_disk_space_culls_oldest_first(
        num_entries in 5..20usize,
    ) {
        let temp_dir = TempDir::new().unwrap();
        let trash_dir = temp_dir.path().join("trash");

        // Create trash entries with different timestamps
        fs::create_dir_all(&trash_dir).unwrap();
        for i in 0..num_entries {
            let entry_dir = trash_dir.join(format!("entry_{}", i));
            fs::create_dir(&entry_dir).unwrap();
            let file = entry_dir.join("file.txt");
            fs::write(&file, format!("content_{}", i)).unwrap();

            // Add delay to ensure different timestamps
            std::thread::sleep(std::time::Duration::from_millis(10));
        }

        // Verify entries were created
        let entries = fs::read_dir(&trash_dir).unwrap();
        let count = entries.count();
        prop_assert_eq!(count, num_entries);

        // Get timestamps before culling
        let timestamps_before = get_entry_timestamps(&trash_dir);
        prop_assert_eq!(timestamps_before.len(), num_entries);

        // Simulate culling: remove oldest 3 entries
        let num_to_cull = 3.min(num_entries);
        for (entry_name, _) in timestamps_before.iter().take(num_to_cull) {
            let entry_path = trash_dir.join(entry_name);
            fs::remove_dir_all(&entry_path).unwrap();
        }

        // Verify oldest entries were removed
        let timestamps_after = get_entry_timestamps(&trash_dir);
        prop_assert_eq!(timestamps_after.len(), num_entries - num_to_cull);

        // Verify remaining entries are newer
        if let (Some(oldest_before), Some(newest_after)) = (timestamps_before.first(), timestamps_after.last()) {
            prop_assert!(oldest_before.1 < newest_after.1);
        }
    }

    #[test]
    fn prop_culling_respects_allow_last_version(
        num_entries in 3..10usize,
    ) {
        let temp_dir = TempDir::new().unwrap();
        let trash_dir = temp_dir.path().join("trash");

        // Create trash entries for the same file
        fs::create_dir_all(&trash_dir).unwrap();
        let filename = "test.txt";
        for i in 0..num_entries {
            let entry_dir = trash_dir.join(filename).join(format!("version_{}", i));
            fs::create_dir_all(&entry_dir).unwrap();
            let file = entry_dir.join(filename);
            fs::write(&file, format!("content_{}", i)).unwrap();

            std::thread::sleep(std::time::Duration::from_millis(10));
        }

        // Verify all versions exist
        let versions_dir = trash_dir.join(filename);
        let entries = fs::read_dir(&versions_dir).unwrap();
        let count = entries.count();
        prop_assert_eq!(count, num_entries);

        // Simulate culling with allow_last_version_cull = false
        // Remove all but the newest version
        let entries = fs::read_dir(&versions_dir).unwrap();
        let mut version_dirs: Vec<_> = entries.filter_map(|e| e.ok()).collect();
        version_dirs.sort_by_key(|e| e.file_name());

        // Remove all but the last (newest) version
        for version_dir in version_dirs.iter().take(num_entries.saturating_sub(1)) {
            fs::remove_dir_all(version_dir.path()).unwrap();
        }

        // Verify only newest version remains
        let entries = fs::read_dir(&versions_dir).unwrap();
        let count = entries.count();
        prop_assert_eq!(count, 1);
    }
}
