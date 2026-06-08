use proptest::prelude::*;
use std::fs;
use std::process::Command;
use tempfile::TempDir;

proptest! {
    #[test]
    fn prop_vcs_tracked_files_not_lost(
        file_content in "[a-zA-Z0-9]{0,1000}",
        filename in "[a-zA-Z0-9_\\-]{1,50}",
    ) {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join(&filename);
        let moved_path = temp_dir.path().join(format!("{}_moved", filename));

        // Initialize Git repo
        Command::new("git")
            .args(["init"])
            .current_dir(temp_dir.path())
            .output()
            .unwrap();

        // Configure Git
        Command::new("git")
            .args(["config", "user.email", "test@example.com"])
            .current_dir(temp_dir.path())
            .output()
            .unwrap();

        Command::new("git")
            .args(["config", "user.name", "Test User"])
            .current_dir(temp_dir.path())
            .output()
            .unwrap();

        // Write file content
        fs::write(&file_path, &file_content).unwrap();

        // Add and commit the file
        Command::new("git")
            .args(["add", &filename])
            .current_dir(temp_dir.path())
            .output()
            .unwrap();

        Command::new("git")
            .args(["commit", "-m", "Initial commit"])
            .current_dir(temp_dir.path())
            .output()
            .unwrap();

        // Perform VCS-native move (git mv)
        Command::new("git")
            .args(["mv", &filename, &format!("{}_moved", filename)])
            .current_dir(temp_dir.path())
            .output()
            .unwrap();

        // Verify the file is still tracked in Git after move
        let output = Command::new("git")
            .args(["ls-files"])
            .current_dir(temp_dir.path())
            .output()
            .unwrap();

        let tracked_files = String::from_utf8_lossy(&output.stdout);
        let moved_filename = format!("{}_moved", filename);
        prop_assert!(tracked_files.contains(&moved_filename));

        // Verify content is preserved
        let moved_content = fs::read_to_string(&moved_path).unwrap();
        prop_assert_eq!(file_content, moved_content);

        // Verify source no longer exists
        prop_assert!(!file_path.exists());
    }

    #[test]
    fn prop_uncommitted_changes_preserved(
        file_content in "[a-zA-Z0-9]{0,1000}",
        filename in "[a-zA-Z0-9_\\-]{1,50}",
    ) {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join(&filename);

        // Initialize Git repo
        Command::new("git")
            .args(["init"])
            .current_dir(temp_dir.path())
            .output()
            .unwrap();

        // Configure Git
        Command::new("git")
            .args(["config", "user.email", "test@example.com"])
            .current_dir(temp_dir.path())
            .output()
            .unwrap();

        Command::new("git")
            .args(["config", "user.name", "Test User"])
            .current_dir(temp_dir.path())
            .output()
            .unwrap();

        // Write initial content
        fs::write(&file_path, "initial").unwrap();

        // Add and commit
        Command::new("git")
            .args(["add", &filename])
            .current_dir(temp_dir.path())
            .output()
            .unwrap();

        Command::new("git")
            .args(["commit", "-m", "Initial commit"])
            .current_dir(temp_dir.path())
            .output()
            .unwrap();

        // Modify file (uncommitted change)
        fs::write(&file_path, &file_content).unwrap();

        // Verify file has uncommitted changes
        let output = Command::new("git")
            .args(["status", "--porcelain"])
            .current_dir(temp_dir.path())
            .output()
            .unwrap();

        let status = String::from_utf8_lossy(&output.stdout);
        prop_assert!(status.contains("M"));

        // Note: This is a placeholder - actual implementation will verify that
        // smartfo's VCS-aware delete preserves uncommitted changes in trash
    }
}
