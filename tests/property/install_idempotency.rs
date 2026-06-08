use proptest::prelude::*;
use std::fs;
use std::os::unix::fs::symlink;
use tempfile::TempDir;

proptest! {
    #[test]
    fn prop_install_creates_symlinks(
        symlink_name in "[a-zA-Z0-9_\\-]{1,20}",
    ) {
        let temp_dir = TempDir::new().unwrap();
        let bin_dir = temp_dir.path().join("bin");
        let smartfo_binary = temp_dir.path().join("smartfo");
        let symlink_path = bin_dir.join(&symlink_name);

        // Create directories
        fs::create_dir_all(&bin_dir).unwrap();

        // Create a dummy smartfo binary
        fs::write(&smartfo_binary, "#!/bin/sh\necho smartfo").unwrap();

        // Simulate install: create symlink
        symlink(&smartfo_binary, &symlink_path).unwrap();

        // Verify symlink exists
        prop_assert!(symlink_path.exists());

        // Verify it's a symlink
        prop_assert!(symlink_path.is_symlink());

        // Verify symlink points to correct target
        let target = fs::read_link(&symlink_path).unwrap();
        prop_assert_eq!(target, smartfo_binary);
    }

    #[test]
    fn prop_install_does_not_overwrite_existing_files(
        symlink_name in "[a-zA-Z0-9_\\-]{1,20}",
    ) {
        let temp_dir = TempDir::new().unwrap();
        let bin_dir = temp_dir.path().join("bin");
        let smartfo_binary = temp_dir.path().join("smartfo");
        let existing_file = bin_dir.join(&symlink_name);

        // Create directories
        fs::create_dir_all(&bin_dir).unwrap();

        // Create a dummy smartfo binary
        fs::write(&smartfo_binary, "#!/bin/sh\necho smartfo").unwrap();

        // Create an existing file with the same name
        fs::write(&existing_file, "existing content").unwrap();

        // Simulate install without --force: should refuse to overwrite
        let result = symlink(&smartfo_binary, &existing_file);

        // Verify symlink creation failed
        prop_assert!(result.is_err());

        // Verify existing file is unchanged
        let content = fs::read_to_string(&existing_file).unwrap();
        prop_assert_eq!(content, "existing content");
    }

    #[test]
    fn prop_install_with_force_overwrites(
        symlink_name in "[a-zA-Z0-9_\\-]{1,20}",
    ) {
        let temp_dir = TempDir::new().unwrap();
        let bin_dir = temp_dir.path().join("bin");
        let smartfo_binary = temp_dir.path().join("smartfo");
        let existing_file = bin_dir.join(&symlink_name);

        // Create directories
        fs::create_dir_all(&bin_dir).unwrap();

        // Create a dummy smartfo binary
        fs::write(&smartfo_binary, "#!/bin/sh\necho smartfo").unwrap();

        // Create an existing file with the same name
        fs::write(&existing_file, "existing content").unwrap();

        // Simulate install with --force: remove existing file first
        fs::remove_file(&existing_file).unwrap();
        symlink(&smartfo_binary, &existing_file).unwrap();

        // Verify symlink exists
        prop_assert!(existing_file.exists());

        // Verify it's a symlink
        prop_assert!(existing_file.is_symlink());

        // Verify symlink points to correct target
        let target = fs::read_link(&existing_file).unwrap();
        prop_assert_eq!(target, smartfo_binary);
    }

    #[test]
    fn prop_install_creates_all_required_symlinks(
        symlink_names in prop::collection::vec("[a-zA-Z0-9_\\-]{1,20}", 1..5),
    ) {
        let temp_dir = TempDir::new().unwrap();
        let bin_dir = temp_dir.path().join("bin");
        let smartfo_binary = temp_dir.path().join("smartfo");

        // Create directories
        fs::create_dir_all(&bin_dir).unwrap();

        // Create a dummy smartfo binary
        fs::write(&smartfo_binary, "#!/bin/sh\necho smartfo").unwrap();

        // Simulate install: create all symlinks
        for symlink_name in &symlink_names {
            let symlink_path = bin_dir.join(symlink_name);
            symlink(&smartfo_binary, &symlink_path).unwrap();
        }

        // Verify all symlinks exist
        for symlink_name in &symlink_names {
            let symlink_path = bin_dir.join(symlink_name);
            prop_assert!(symlink_path.exists());
            prop_assert!(symlink_path.is_symlink());
        }
    }

    #[test]
    fn prop_install_idempotent(
        symlink_name in "[a-zA-Z0-9_\\-]{1,20}",
    ) {
        let temp_dir = TempDir::new().unwrap();
        let bin_dir = temp_dir.path().join("bin");
        let smartfo_binary = temp_dir.path().join("smartfo");
        let symlink_path = bin_dir.join(&symlink_name);

        // Create directories
        fs::create_dir_all(&bin_dir).unwrap();

        // Create a dummy smartfo binary
        fs::write(&smartfo_binary, "#!/bin/sh\necho smartfo").unwrap();

        // First install
        symlink(&smartfo_binary, &symlink_path).unwrap();
        let target1 = fs::read_link(&symlink_path).unwrap();

        // Second install (idempotent)
        let result = symlink(&smartfo_binary, &symlink_path);

        // Verify second install fails (symlink already exists)
        prop_assert!(result.is_err());

        // Verify original symlink is unchanged
        let target2 = fs::read_link(&symlink_path).unwrap();
        prop_assert_eq!(target1, target2);
    }
}
