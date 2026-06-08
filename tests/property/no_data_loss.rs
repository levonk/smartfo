use proptest::prelude::*;
use std::fs;
use tempfile::TempDir;

proptest! {
    #[test]
    fn prop_no_data_loss_on_move(
        file_content in "[a-zA-Z0-9]{0,1000}",
        filename in "[a-zA-Z0-9_\\-]{1,50}",
    ) {
        let temp_dir = TempDir::new().unwrap();
        let src = temp_dir.path().join(&filename);
        let dest = temp_dir.path().join(format!("{}_moved", filename));

        // Write original content
        fs::write(&src, &file_content).unwrap();

        // Simulate move (atomic rename for same filesystem)
        fs::rename(&src, &dest).unwrap();

        // Verify content is preserved
        let moved_content = fs::read_to_string(&dest).unwrap();
        prop_assert_eq!(file_content, moved_content);

        // Verify source no longer exists
        prop_assert!(!src.exists());
    }

    #[test]
    fn prop_no_data_loss_on_copy(
        file_content in "[a-zA-Z0-9]{0,1000}",
        filename in "[a-zA-Z0-9_\\-]{1,50}",
    ) {
        let temp_dir = TempDir::new().unwrap();
        let src = temp_dir.path().join(&filename);
        let dest = temp_dir.path().join(format!("{}_copy", filename));

        // Write original content
        fs::write(&src, &file_content).unwrap();

        // Simulate copy
        fs::copy(&src, &dest).unwrap();

        // Verify both files have same content
        let src_content = fs::read_to_string(&src).unwrap();
        let dest_content = fs::read_to_string(&dest).unwrap();
        prop_assert_eq!(file_content.clone(), src_content);
        prop_assert_eq!(file_content, dest_content);
    }

    #[test]
    fn prop_no_data_loss_on_delete_with_trash(
        file_content in "[a-zA-Z0-9]{0,1000}",
        filename in "[a-zA-Z0-9_\\-]{1,50}",
    ) {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join(&filename);
        let trash_dir = temp_dir.path().join("trash");

        // Write original content
        fs::write(&file_path, &file_content).unwrap();

        // Create trash directory
        fs::create_dir_all(&trash_dir).unwrap();

        // Simulate trash move (delete with trash)
        let trash_path = trash_dir.join(&filename);
        fs::rename(&file_path, &trash_path).unwrap();

        // Verify content is preserved in trash
        let trash_content = fs::read_to_string(&trash_path).unwrap();
        prop_assert_eq!(file_content, trash_content);

        // Verify original no longer exists
        prop_assert!(!file_path.exists());
    }

    #[test]
    fn prop_no_data_loss_cross_device(
        file_content in "[a-zA-Z0-9]{0,1000}",
        filename in "[a-zA-Z0-9_\\-]{1,50}",
    ) {
        let temp_dir = TempDir::new().unwrap();
        let src = temp_dir.path().join(&filename);
        let dest = temp_dir.path().join("subdir").join(&filename);

        // Write original content
        fs::write(&src, &file_content).unwrap();

        // Create destination directory
        fs::create_dir_all(dest.parent().unwrap()).unwrap();

        // Simulate cross-device copy (copy + delete)
        fs::copy(&src, &dest).unwrap();
        fs::remove_file(&src).unwrap();

        // Verify content is preserved at destination
        let dest_content = fs::read_to_string(&dest).unwrap();
        prop_assert_eq!(file_content, dest_content);

        // Verify source no longer exists
        prop_assert!(!src.exists());
    }
}
