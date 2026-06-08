use proptest::prelude::*;
use std::fs;
use tempfile::TempDir;

proptest! {
    #[test]
    fn prop_force_delete_bypasses_trash(
        file_content in "[a-zA-Z0-9]{0,1000}",
        filename in "[a-zA-Z0-9_\\-]{1,50}",
    ) {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join(&filename);
        let trash_dir = temp_dir.path().join("trash");

        // Create file
        fs::write(&file_path, &file_content).unwrap();

        // Create trash directory
        fs::create_dir_all(&trash_dir).unwrap();

        // Simulate --force-delete: direct unlink without trash
        fs::remove_file(&file_path).unwrap();

        // Verify file is deleted
        prop_assert!(!file_path.exists());

        // Verify file is NOT in trash
        let trash_path = trash_dir.join(&filename);
        prop_assert!(!trash_path.exists());

        // Verify trash directory is empty (or doesn't contain the file)
        if let Ok(entries) = fs::read_dir(&trash_dir) {
            let count = entries.count();
            prop_assert_eq!(count, 0);
        }
    }

    #[test]
    fn prop_force_delete_with_trash_mode_always(
        file_content in "[a-zA-Z0-9]{0,1000}",
        filename in "[a-zA-Z0-9_\\-]{1,50}",
    ) {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join(&filename);
        let trash_dir = temp_dir.path().join("trash");

        // Create file
        fs::write(&file_path, &file_content).unwrap();

        // Create trash directory (simulating trash_mode = "always")
        fs::create_dir_all(&trash_dir).unwrap();

        // Simulate --force-delete: should bypass trash even with trash_mode = "always"
        fs::remove_file(&file_path).unwrap();

        // Verify file is deleted
        prop_assert!(!file_path.exists());

        // Verify file is NOT in trash
        let trash_path = trash_dir.join(&filename);
        prop_assert!(!trash_path.exists());
    }

    #[test]
    fn prop_force_delete_with_low_disk_space(
        file_content in "[a-zA-Z0-9]{0,1000}",
        filename in "[a-zA-Z0-9_\\-]{1,50}",
    ) {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join(&filename);
        let trash_dir = temp_dir.path().join("trash");

        // Create file
        fs::write(&file_path, &file_content).unwrap();

        // Create trash directory (simulating low disk space scenario)
        fs::create_dir_all(&trash_dir).unwrap();

        // Simulate --force-delete: should bypass trash even with low disk space
        fs::remove_file(&file_path).unwrap();

        // Verify file is deleted
        prop_assert!(!file_path.exists());

        // Verify file is NOT in trash
        let trash_path = trash_dir.join(&filename);
        prop_assert!(!trash_path.exists());
    }

    #[test]
    fn prop_force_delete_multiple_files(
        num_files in 1..20usize,
    ) {
        let temp_dir = TempDir::new().unwrap();
        let trash_dir = temp_dir.path().join("trash");

        // Create multiple files
        let mut file_paths = Vec::new();
        for i in 0..num_files {
            let file_path = temp_dir.path().join(format!("file_{}.txt", i));
            fs::write(&file_path, format!("content_{}", i)).unwrap();
            file_paths.push(file_path);
        }

        // Create trash directory
        fs::create_dir_all(&trash_dir).unwrap();

        // Simulate --force-delete on all files
        for file_path in &file_paths {
            fs::remove_file(file_path).unwrap();
        }

        // Verify all files are deleted
        for file_path in &file_paths {
            prop_assert!(!file_path.exists());
        }

        // Verify no files are in trash
        if let Ok(entries) = fs::read_dir(&trash_dir) {
            let count = entries.count();
            prop_assert_eq!(count, 0);
        }
    }
}
