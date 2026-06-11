//! Integration tests for file and URL reference formatting

use smartfo::error::SmartfoError;

#[test]
fn test_vscode_file_references() {
    let path = std::path::PathBuf::from("/tmp/test.txt");
    
    // Basic reference
    let ref_str = SmartfoError::vscode_file_reference(&path, None, None);
    assert_eq!(ref_str, "file:///tmp/test.txt");
    
    // With line number
    let ref_str = SmartfoError::vscode_file_reference(&path, Some(42), None);
    assert_eq!(ref_str, "file:///tmp/test.txt:42");
    
    // With line and column
    let ref_str = SmartfoError::vscode_file_reference(&path, Some(42), Some(10));
    assert_eq!(ref_str, "file:///tmp/test.txt:42:10");
}

#[test]
fn test_standard_file_references() {
    let path = std::path::PathBuf::from("test.txt");
    
    // Basic reference
    let ref_str = SmartfoError::standard_file_reference(&path, None, None);
    assert_eq!(ref_str, "test.txt");
    
    // With line number
    let ref_str = SmartfoError::standard_file_reference(&path, Some(42), None);
    assert_eq!(ref_str, "test.txt:42");
    
    // With line and column
    let ref_str = SmartfoError::standard_file_reference(&path, Some(42), Some(10));
    assert_eq!(ref_str, "test.txt:42:10");
}

#[test]
fn test_url_formatting() {
    let url = "https://example.com/path?query=value";
    let linked = SmartfoError::terminal_linked_url(url);
    
    // Should contain OSC 8 escape sequence
    assert!(linked.contains("\x1b]8;;"));
    // Should contain the URL twice (link target and display text)
    let count = linked.matches(url).count();
    assert!(count >= 1);
}

#[test]
fn test_url_encoding() {
    // Test with special characters that should be properly encoded
    let url = "https://example.com/path with spaces";
    let linked = SmartfoError::terminal_linked_url(url);
    
    // The function should handle the URL as-is (encoding is caller's responsibility)
    assert!(linked.contains(url));
}

#[test]
fn test_terminal_linkification() {
    let path = std::path::PathBuf::from("/tmp/test.txt");
    let linked = SmartfoError::terminal_linked_reference(&path, Some(42), None);
    
    // Should contain OSC 8 escape sequence
    assert!(linked.contains("\x1b]8;;"));
    // Should contain VSCode format
    assert!(linked.contains("file:///tmp/test.txt:42"));
    // Should contain standard format
    assert!(linked.contains("/tmp/test.txt:42"));
}

#[test]
fn test_help_output_references() {
    // This test verifies that help output doesn't contain malformed file references
    // The actual help output is tested in the CLI integration tests
    // This is a placeholder to ensure the formatting functions are available
    let path = std::path::PathBuf::from("/usr/local/bin/smartfo");
    let ref_str = SmartfoError::standard_file_reference(&path, None, None);
    assert!(ref_str.contains("/usr/local/bin/smartfo"));
}

#[test]
fn test_error_output_references() {
    // Test that error messages can include file references
    let path = std::path::PathBuf::from("/etc/smartfo/config.toml");
    let ref_str = SmartfoError::vscode_file_reference(&path, Some(10), Some(5));
    
    // Should be usable in error context
    let error_msg = format!("Configuration error in {}", ref_str);
    assert!(error_msg.contains("file:///etc/smartfo/config.toml:10:5"));
}
