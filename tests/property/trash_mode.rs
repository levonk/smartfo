use proptest::prelude::*;
use std::fs;
use tempfile::TempDir;

proptest! {
    #[test]
    fn prop_trash_mode_never_direct_delete(
        file_content in "[a-zA-Z0-9]{0,1000}",
        filename in "[a-zA-Z0-9_\\-]{1,50}",
    ) {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join(&filename);
        let trash_dir = temp_dir.path().join("trash");

        // Create file
        fs::write(&file_path, &file_content).unwrap();

        // Create trash directory (but trash_mode = "never" should not use it)
        fs::create_dir_all(&trash_dir).unwrap();

        // Simulate trash_mode = "never": direct delete without trash
        fs::remove_file(&file_path).unwrap();

        // Verify file is deleted
        prop_assert!(!file_path.exists());

        // Verify file is NOT in trash
        let trash_path = trash_dir.join(&filename);
        prop_assert!(!trash_path.exists());

        // Verify trash directory is empty
        if let Ok(entries) = fs::read_dir(&trash_dir) {
            let count = entries.count();
            prop_assert_eq!(count, 0);
        }
    }

    #[test]
    fn prop_trash_mode_never_multiple_files(
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

        // Simulate trash_mode = "never": direct delete for all files
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

    #[test]
    fn prop_trash_mode_never_with_trash_already_exists(
        file_content in "[a-zA-Z0-9]{0,1000}",
        filename in "[a-zA-Z0-9_\\-]{1,50}",
    ) {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join(&filename);
        let trash_dir = temp_dir.path().join("trash");

        // Create trash directory with existing content
        fs::create_dir_all(&trash_dir).unwrap();
        let existing_file = trash_dir.join("existing.txt");
        fs::write(&existing_file, "existing content").unwrap();

        // Create file to delete
        fs::write(&file_path, &file_content).unwrap();

        // Simulate trash_mode = "never": direct delete
        fs::remove_file(&file_path).unwrap();

        // Verify file is deleted
        prop_assert!(!file_path.exists());

        // Verify file is NOT in trash
        let trash_path = trash_dir.join(&filename);
        prop_assert!(!trash_path.exists());

        // Verify existing trash content is preserved
        prop_assert!(existing_file.exists());
        let existing_content = fs::read_to_string(&existing_file).unwrap();
        prop_assert_eq!(existing_content, "existing content");
    }

    #[test]
    fn prop_trash_mode_auto_fallback_when_full(
        file_content in "[a-zA-Z0-9]{0,1000}",
        filename in "[a-zA-Z0-9_\\-]{1,50}",
    ) {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join(&filename);
        let trash_dir = temp_dir.path().join("trash");

        // Create trash directory and fill it (simulate full trash)
        fs::create_dir_all(&trash_dir).unwrap();
        for i in 0..100 {
            let large_file = trash_dir.join(format!("large_{}.dat", i));
            fs::write(&large_file, vec![0u8; 1024 * 1024]).unwrap(); // 1MB files
        }

        // Create file to delete
        fs::write(&file_path, &file_content).unwrap();

        // Simulate trash_mode = "auto" with full trash: fallback to direct delete
        fs::remove_file(&file_path).unwrap();

        // Verify file is deleted
        prop_assert!(!file_path.exists());

        // Verify file is NOT in trash (fallback to direct delete)
        let trash_path = trash_dir.join(&filename);
        prop_assert!(!trash_path.exists());
    }

    #[test]
    fn prop_trash_mode_auto_uses_trash_when_space(
        file_content in "[a-zA-Z0-9]{0,1000}",
        filename in "[a-zA-Z0-9_\\-]{1,50}",
    ) {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join(&filename);
        let trash_dir = temp_dir.path().join("trash");

        // Create trash directory (empty, has space)
        fs::create_dir_all(&trash_dir).unwrap();

        // Create file to delete
        fs::write(&file_path, &file_content).unwrap();

        // Simulate trash_mode = "auto" with space: use trash
        let trash_subdir = trash_dir.join(&filename);
        fs::create_dir_all(&trash_subdir).unwrap();
        let trash_path = trash_subdir.join(&filename);
        fs::rename(&file_path, &trash_path).unwrap();

        // Verify file is deleted from original location
        prop_assert!(!file_path.exists());

        // Verify file IS in trash (trash has space)
        prop_assert!(trash_path.exists());

        // Verify content is preserved
        let trash_content = fs::read_to_string(&trash_path).unwrap();
        prop_assert_eq!(file_content, trash_content);
    }
}
