use proptest::prelude::*;
use std::fs;
use tempfile::TempDir;

proptest! {
    #[test]
    fn prop_same_file_deletion_history_preserved(
        file_content in "[a-zA-Z0-9]{0,1000}",
        filename in "[a-zA-Z0-9_\\-]{1,50}",
        num_deletions in 1..10usize,
    ) {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join(&filename);
        let trash_dir = temp_dir.path().join("trash");

        // Create trash directory
        fs::create_dir_all(&trash_dir).unwrap();

        // Simulate multiple deletions of the same file
        for i in 0..num_deletions {
            // Recreate the file
            fs::write(&file_path, &file_content).unwrap();

            // Move to trash with timestamped subdirectory
            let timestamp = format!("2024-01-01T00:00:{}", i);
            let trash_subdir = trash_dir.join(&filename).join(&timestamp);
            fs::create_dir_all(&trash_subdir).unwrap();

            let trash_path = trash_subdir.join(&filename);
            fs::rename(&file_path, &trash_path).unwrap();

            // Verify content is preserved in trash
            let trash_content = fs::read_to_string(&trash_path).unwrap();
            prop_assert_eq!(file_content.clone(), trash_content);
        }

        // Verify all versions exist in trash
        let entries = fs::read_dir(trash_dir.join(&filename)).unwrap();
        let count = entries.count();
        prop_assert_eq!(count, num_deletions);

        // Verify original no longer exists
        prop_assert!(!file_path.exists());
    }

    #[test]
    fn prop_deletion_history_with_different_content(
        base_content in "[a-zA-Z0-9]{0,100}",
        filename in "[a-zA-Z0-9_\\-]{1,50}",
        num_deletions in 1..5usize,
    ) {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join(&filename);
        let trash_dir = temp_dir.path().join("trash");

        // Create trash directory
        fs::create_dir_all(&trash_dir).unwrap();

        // Simulate multiple deletions with different content
        for i in 0..num_deletions {
            // Create file with different content each time
            let content = format!("{}-{}", base_content, i);
            fs::write(&file_path, &content).unwrap();

            // Move to trash with timestamped subdirectory
            let timestamp = format!("2024-01-01T00:00:{}", i);
            let trash_subdir = trash_dir.join(&filename).join(&timestamp);
            fs::create_dir_all(&trash_subdir).unwrap();

            let trash_path = trash_subdir.join(&filename);
            fs::rename(&file_path, &trash_path).unwrap();

            // Verify specific content is preserved
            let trash_content = fs::read_to_string(&trash_path).unwrap();
            prop_assert_eq!(content, trash_content);
        }

        // Verify all versions exist in trash
        let entries = fs::read_dir(trash_dir.join(&filename)).unwrap();
        let count = entries.count();
        prop_assert_eq!(count, num_deletions);
    }
}
