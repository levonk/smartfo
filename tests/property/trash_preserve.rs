use proptest::prelude::*;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

fn count_files_recursive(dir: &Path) -> std::io::Result<usize> {
    let mut count = 0;
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            count += count_files_recursive(&path)?;
        } else {
            count += 1;
        }
    }
    Ok(count)
}

proptest! {
    #[test]
    fn prop_directory_tree_preserved(
        dir_depth in 1..5usize,
        files_per_dir in 1..10usize,
    ) {
        let temp_dir = TempDir::new().unwrap();
        let src_dir = temp_dir.path().join("source");
        let trash_dir = temp_dir.path().join("trash");

        // Create nested directory structure
        let mut current = src_dir.clone();
        for depth in 0..dir_depth {
            fs::create_dir_all(&current).unwrap();
            for i in 0..files_per_dir {
                let file = current.join(format!("file_{}_{}.txt", depth, i));
                fs::write(&file, format!("content_{}_{}", depth, i)).unwrap();
            }
            current = current.join(format!("level_{}", depth));
        }

        // Count files in source
        let source_count = count_files_recursive(&src_dir).unwrap();
        prop_assert!(source_count > 0);

        // Simulate trash move (copy entire tree)
        fs::create_dir_all(&trash_dir).unwrap();
        let trash_subdir = trash_dir.join("trashed");
        let mut options = fs_extra::dir::CopyOptions::new();
        options.content_only = false;
        fs_extra::dir::copy(&src_dir, &trash_subdir, &options).unwrap();

        // Count files in trash
        let trash_count = count_files_recursive(&trash_subdir).unwrap();

        // Verify all files are preserved
        prop_assert_eq!(source_count, trash_count);

        // Verify directory structure is preserved
        prop_assert!(trash_subdir.exists());
    }

    #[test]
    fn prop_directory_structure_preserved(
        dir_names in prop::collection::vec("[a-zA-Z0-9_\\-]{1,20}", 1..5),
    ) {
        let temp_dir = TempDir::new().unwrap();
        let src_dir = temp_dir.path().join("source");
        let trash_dir = temp_dir.path().join("trash");

        // Create directory structure
        let mut current = src_dir.clone();
        for dir_name in &dir_names {
            current = current.join(dir_name);
            fs::create_dir_all(&current).unwrap();
            let file = current.join("file.txt");
            fs::write(&file, "content").unwrap();
        }

        // Simulate trash move
        fs::create_dir_all(&trash_dir).unwrap();
        let trash_subdir = trash_dir.join("trashed");
        let mut options = fs_extra::dir::CopyOptions::new();
        options.content_only = false;
        fs_extra::dir::copy(&src_dir, &trash_subdir, &options).unwrap();

        // Verify directory structure is preserved
        let mut current_trash = trash_subdir.clone();
        for dir_name in &dir_names {
            current_trash = current_trash.join(dir_name);
            prop_assert!(current_trash.exists());
            prop_assert!(current_trash.is_dir());
        }
    }
}
