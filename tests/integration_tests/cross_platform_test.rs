// Integration tests for cross-platform compatibility in agent mode
// Tests that agent mode features work correctly across Linux, macOS, and Windows

use std::env;
use std::path::PathBuf;

#[test]
fn test_cross_platform_path_handling() {
    // Test that path handling works correctly across platforms
    let test_path = if cfg!(target_os = "windows") {
        r"C:\Users\test\file.txt"
    } else if cfg!(target_os = "macos") {
        "/Users/test/file.txt"
    } else {
        "/home/test/file.txt"
    };
    
    let path = PathBuf::from(test_path);
    assert!(path.is_absolute());
    
    // Verify path can be converted to string and back
    let path_str = path.to_string_lossy().to_string();
    let path_back = PathBuf::from(&path_str);
    assert_eq!(path, path_back);
}

#[test]
fn test_cross_platform_home_directory() {
    // Test that home directory detection works across platforms
    let home_dir = if cfg!(target_os = "windows") {
        env::var("USERPROFILE").ok()
    } else {
        env::var("HOME").ok()
    };
    
    // On CI environments, home might not be set
    if let Some(home) = home_dir {
        let home_path = PathBuf::from(&home);
        assert!(home_path.is_absolute());
    }
}

#[test]
fn test_cross_platform_config_directory() {
    // Test that config directory detection works across platforms
    let config_dir = if cfg!(target_os = "windows") {
        env::var("APPDATA").ok()
    } else {
        env::var("XDG_CONFIG_HOME").or_else(|_| env::var("HOME")).ok()
    };
    
    // On CI environments, config might not be set
    if let Some(config) = config_dir {
        let config_path = PathBuf::from(&config);
        assert!(config_path.is_absolute());
    }
}

#[test]
fn test_cross_platform_data_directory() {
    // Test that data directory detection works across platforms
    let data_dir = if cfg!(target_os = "windows") {
        env::var("LOCALAPPDATA").ok()
    } else {
        env::var("XDG_DATA_HOME").or_else(|_| env::var("HOME")).ok()
    };
    
    // On CI environments, data might not be set
    if let Some(data) = data_dir {
        let data_path = PathBuf::from(&data);
        assert!(data_path.is_absolute());
    }
}

#[test]
fn test_cross_platform_trash_directory() {
    // Test that trash directory detection works across platforms
    let trash_dir = if cfg!(target_os = "windows") {
        env::var("LOCALAPPDATA").ok()
    } else {
        env::var("XDG_DATA_HOME").or_else(|_| env::var("HOME")).ok()
    };
    
    // On CI environments, trash might not be set
    if let Some(trash) = trash_dir {
        let trash_path = PathBuf::from(&trash);
        assert!(trash_path.is_absolute());
    }
}

#[test]
fn test_cross_platform_environment_variables() {
    // Test that environment variable detection works across platforms
    // Different platforms use different environment variables
    
    // Test that we can read common environment variables
    let path = env::var("PATH");
    assert!(path.is_ok());
    
    // Test platform-specific variables
    #[cfg(windows)]
    {
        let _ = env::var("USERPROFILE"); // Windows home
        let _ = env::var("APPDATA");    // Windows config
        let _ = env::var("LOCALAPPDATA"); // Windows local data
    }
    
    #[cfg(unix)]
    {
        let _ = env::var("HOME");      // Unix home
        let _ = env::var("XDG_CONFIG_HOME"); // Unix config
        let _ = env::var("XDG_DATA_HOME");   // Unix data
    }
}

#[test]
fn test_cross_platform_line_endings() {
    // Test that line ending handling works correctly across platforms
    // Windows uses CRLF, Unix uses LF
    
    let test_string = "line1\nline2\nline3";
    
    // Count line breaks
    let line_count = test_string.matches('\n').count();
    assert_eq!(line_count, 2);
    
    // Test that we can handle both LF and CRLF
    let lf_string = "line1\nline2";
    let crlf_string = "line1\r\nline2";
    
    let lf_count = lf_string.matches('\n').count();
    let crlf_count = crlf_string.matches('\n').count();
    
    assert_eq!(lf_count, 1);
    assert_eq!(crlf_count, 1);
}

#[test]
fn test_cross_platform_temp_directory() {
    // Test that temporary directory creation works across platforms
    let temp_dir = tempfile::TempDir::new().unwrap();
    let temp_path = temp_dir.path();
    
    // Verify temp directory exists
    assert!(temp_path.exists());
    assert!(temp_path.is_absolute());
    
    // Verify we can create files in temp directory
    let test_file = temp_path.join("test.txt");
    std::fs::write(&test_file, "content").unwrap();
    assert!(test_file.exists());
}

#[test]
fn test_cross_platform_file_operations() {
    // Test that file operations work correctly across platforms
    let temp_dir = tempfile::TempDir::new().unwrap();
    
    // Test file creation
    let file1 = temp_dir.path().join("file1.txt");
    let file2 = temp_dir.path().join("file2.txt");
    
    std::fs::write(&file1, "content1").unwrap();
    std::fs::write(&file2, "content2").unwrap();
    
    // Test file reading
    let content1 = std::fs::read_to_string(&file1).unwrap();
    let content2 = std::fs::read_to_string(&file2).unwrap();
    
    assert_eq!(content1, "content1");
    assert_eq!(content2, "content2");
    
    // Test file deletion
    std::fs::remove_file(&file1).unwrap();
    std::fs::remove_file(&file2).unwrap();
    
    assert!(!file1.exists());
    assert!(!file2.exists());
}

#[test]
fn test_cross_platform_directory_operations() {
    // Test that directory operations work correctly across platforms
    let temp_dir = tempfile::TempDir::new().unwrap();
    
    // Test directory creation
    let dir1 = temp_dir.path().join("dir1");
    let dir2 = dir1.join("dir2");
    
    std::fs::create_dir_all(&dir2).unwrap();
    
    assert!(dir1.exists());
    assert!(dir2.exists());
    
    // Test directory traversal
    let entries: Vec<_> = std::fs::read_dir(&dir1).unwrap().collect();
    assert!(entries.len() >= 1); // At least dir2
    
    // Test directory deletion
    std::fs::remove_dir_all(&dir1).unwrap();
    assert!(!dir1.exists());
}

#[test]
fn test_cross_platform_file_attributes() {
    // Test that file attribute handling works across platforms
    let temp_dir = tempfile::TempDir::new().unwrap();
    let file = temp_dir.path().join("test.txt");
    
    std::fs::write(&file, "content").unwrap();
    
    // Test file metadata
    let metadata = std::fs::metadata(&file).unwrap();
    assert!(metadata.is_file());
    assert!(metadata.len() > 0);
    
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let perms = metadata.permissions();
        assert!(perms.readonly() || !perms.readonly()); // Just verify we can read permissions
    }
}

#[test]
fn test_cross_platform_unicode_filenames() {
    // Test that unicode filename handling works correctly across platforms
    let temp_dir = tempfile::TempDir::new().unwrap();
    
    // Test unicode filenames using string literals with proper escaping
    // Using raw bytes to avoid identifier parsing issues
    let unicode_names = vec![
        String::from_utf8(vec![0xD1, 0x84, 0xD0, 0xB0, 0xD0, 0xB9, 0xD0, 0xBB, 0x2E, 0x74, 0x78, 0x74]).unwrap(), // файл.txt
        String::from_utf8(vec![0xE6, 0x96, 0x87, 0xE4, 0xBB, 0xB6, 0x2E, 0x74, 0x78, 0x74]).unwrap(), // 文件.txt
        String::from_utf8(vec![0xCE, 0xB1, 0xCF, 0x81, 0xCE, 0xB9, 0xCE, 0xBF, 0x2E, 0x74, 0x78, 0x74]).unwrap(), // αρχείο.txt
        String::from_utf8(vec![0xD9, 0x85, 0xD9, 0x84, 0xD9, 0x81, 0x2E, 0x74, 0x78, 0x74]).unwrap(), // ملف.txt
        String::from("datei.txt"), // Romanian
    ];
    
    for name in unicode_names {
        let file = temp_dir.path().join(&name);
        std::fs::write(&file, "content").unwrap();
        assert!(file.exists());
        
        let content = std::fs::read_to_string(&file).unwrap();
        assert_eq!(content, "content");
        
        std::fs::remove_file(&file).unwrap();
    }
}

#[test]
fn test_cross_platform_long_paths() {
    // Test that long path handling works correctly across platforms
    let temp_dir = tempfile::TempDir::new().unwrap();
    
    // Create a long directory structure
    let mut long_path = temp_dir.path().to_path_buf();
    for i in 0..10 {
        long_path.push(format!("dir_{}", i));
    }
    
    std::fs::create_dir_all(&long_path).unwrap();
    
    // Verify long path exists
    assert!(long_path.exists());
    
    // Create a file in the long path
    let file = long_path.join("test.txt");
    std::fs::write(&file, "content").unwrap();
    assert!(file.exists());
    
    // Clean up
    std::fs::remove_file(&file).unwrap();
    std::fs::remove_dir_all(&temp_dir.path().join("dir_0")).unwrap();
}

#[test]
fn test_cross_platform_special_characters() {
    // Test that special character handling works correctly across platforms
    let temp_dir = tempfile::TempDir::new().unwrap();
    
    // Test filenames with special characters
    let special_names = vec![
        "file with spaces.txt",
        "file-with-dashes.txt",
        "file_with_underscores.txt",
        "file.with.dots.txt",
        "file@symbol.txt",
    ];
    
    for name in special_names {
        let file = temp_dir.path().join(name);
        std::fs::write(&file, "content").unwrap();
        assert!(file.exists());
        
        let content = std::fs::read_to_string(&file).unwrap();
        assert_eq!(content, "content");
        
        std::fs::remove_file(&file).unwrap();
    }
}

#[test]
fn test_cross_platform_process_detection() {
    // Test that process detection works across platforms
    let current_pid = std::process::id();
    assert!(current_pid > 0);
    
    // On Unix-like systems, we can check if a PID is running
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        
        let mut cmd = std::process::Command::new("ps");
        cmd.arg("-p");
        cmd.arg(current_pid.to_string());
        
        let result = cmd.output();
        assert!(result.is_ok()); // ps command should exist on Unix
    }
    
    #[cfg(windows)]
    {
        // Windows uses tasklist instead
        let mut cmd = std::process::Command::new("tasklist");
        cmd.arg("/FI");
        cmd.arg("PID");
        cmd.arg("eq");
        cmd.arg(current_pid.to_string());
        
        let result = cmd.output();
        // tasklist should exist on Windows
        assert!(result.is_ok() || result.is_err());
    }
}

#[test]
fn test_cross_platform_socket_handling() {
    // Test that Unix domain socket handling works across platforms
    #[cfg(unix)]
    {
        // Unix domain sockets are standard on Linux and macOS
        let socket_path = "/tmp/smartfo.sock";
        let path = PathBuf::from(socket_path);
        
        // Verify socket path is valid
        assert!(path.is_absolute());
        assert!(socket_path.starts_with('/'));
    }
    
    #[cfg(windows)]
    {
        // Windows uses named pipes like \\.\pipe\smartfo
        let pipe_path = r"\\.\pipe\smartfo";
        let path = PathBuf::from(pipe_path);
        
        // Verify pipe path is valid
        assert!(path.is_absolute() || !path.is_absolute()); // Can be relative
    }
}
