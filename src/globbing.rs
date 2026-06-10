use anyhow::{Context, Result};
use glob::Pattern;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use tracing::debug;

/// Expand glob patterns in a list of paths.
/// Supports recursive `**/*` patterns and standard glob patterns.
pub fn expand_globs(paths: &[PathBuf]) -> Result<Vec<PathBuf>> {
    let mut expanded_paths = Vec::new();
    
    for path in paths {
        let path_str = path.to_string_lossy();
        
        // Check if this is a glob pattern
        if contains_glob_pattern(&path_str) {
            debug!("Expanding glob pattern: {}", path_str);
            let matches = expand_single_glob(path)?;
            expanded_paths.extend(matches);
        } else {
            // Not a glob pattern, keep as-is
            expanded_paths.push(path.clone());
        }
    }
    
    Ok(expanded_paths)
}

/// Check if a string contains glob pattern characters
fn contains_glob_pattern(s: &str) -> bool {
    s.contains('*') || s.contains('?') || s.contains('[')
}

/// Expand a single glob pattern
fn expand_single_glob(pattern: &Path) -> Result<Vec<PathBuf>> {
    let pattern_str = pattern.to_string_lossy();
    
    // Handle recursive **/* patterns
    let glob_pattern = if pattern_str.contains("**") {
        // Convert to glob pattern format
        pattern_str.to_string()
    } else {
        // Standard glob pattern
        pattern_str.to_string()
    };
    
    let glob = Pattern::new(&glob_pattern)
        .with_context(|| format!("Invalid glob pattern: {}", glob_pattern))?;
    
    let mut matches = Vec::new();
    
    // If pattern is absolute, search from root
    // If pattern is relative, search from current directory
    let base_path = if pattern.is_absolute() {
        Path::new("/")
    } else {
        Path::new(".")
    };
    
    // Use glob crate to find matches
    let glob_pattern_with_base = if pattern.is_absolute() {
        glob_pattern.clone()
    } else {
        format!("./{}", glob_pattern)
    };
    
    let glob_matches = glob::glob(&glob_pattern_with_base)
        .with_context(|| format!("Failed to read glob pattern: {}", glob_pattern))?;
    
    for entry in glob_matches {
        match entry {
            Ok(path) => {
                // Canonicalize the path to remove any ./ prefix
                if let Ok(canonical) = path.canonicalize() {
                    matches.push(canonical);
                } else {
                    matches.push(path);
                }
            }
            Err(e) => {
                debug!("Glob match error: {}", e);
                // Continue with other matches
            }
        }
    }
    
    // Sort matches for consistent behavior
    matches.sort();
    
    debug!("Glob pattern '{}' matched {} files", glob_pattern, matches.len());
    
    Ok(matches)
}

/// Read from stdin and return as a list of paths (one per line)
pub fn read_stdin_paths() -> Result<Vec<PathBuf>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)
        .context("Failed to read from stdin")?;
    
    let paths: Vec<PathBuf> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| PathBuf::from(line.trim()))
        .collect();
    
    debug!("Read {} paths from stdin", paths.len());
    
    Ok(paths)
}

/// Check if stdin is being piped (not a TTY)
pub fn is_stdin_piped() -> bool {
    !atty::is(atty::Stream::Stdin)
}

/// Process input arguments, handling both glob patterns and stdin
/// If stdin is piped, read paths from stdin and ignore file arguments
/// Otherwise, expand glob patterns in file arguments
pub fn process_input_args(file_args: &[PathBuf], stdin_flag: bool) -> Result<Vec<PathBuf>> {
    if stdin_flag {
        // Explicit stdin requested via `-` argument
        debug!("Reading paths from stdin (explicit - flag)");
        return read_stdin_paths();
    }
    
    if is_stdin_piped() {
        // stdin is piped, read from it
        debug!("Reading paths from piped stdin");
        return read_stdin_paths();
    }
    
    // No stdin, expand glob patterns in file arguments
    debug!("Expanding glob patterns in file arguments");
    expand_globs(file_args)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;
    
    #[test]
    fn test_contains_glob_pattern() {
        assert!(contains_glob_pattern("*.txt"));
        assert!(contains_glob_pattern("test?.rs"));
        assert!(contains_glob_pattern("file[0-9].md"));
        assert!(contains_glob_pattern("**/*.rs"));
        assert!(!contains_glob_pattern("file.txt"));
        assert!(!contains_glob_pattern("/path/to/file"));
    }
    
    #[test]
    fn test_expand_globs_simple() {
        let temp_dir = TempDir::new().unwrap();
        let dir = temp_dir.path();
        
        // Create test files
        fs::write(dir.join("file1.txt"), "test1").unwrap();
        fs::write(dir.join("file2.txt"), "test2").unwrap();
        fs::write(dir.join("other.md"), "test3").unwrap();
        
        let pattern = PathBuf::from(format!("{}/*.txt", dir.display()));
        let expanded = expand_globs(&[pattern]).unwrap();
        
        assert_eq!(expanded.len(), 2);
        assert!(expanded.iter().any(|p| p.ends_with("file1.txt")));
        assert!(expanded.iter().any(|p| p.ends_with("file2.txt")));
    }
    
    #[test]
    fn test_expand_globs_no_match() {
        let temp_dir = TempDir::new().unwrap();
        let dir = temp_dir.path();
        
        let pattern = PathBuf::from(format!("{}/*.nonexistent", dir.display()));
        let expanded = expand_globs(&[pattern]).unwrap();
        
        assert_eq!(expanded.len(), 0);
    }
    
    #[test]
    fn test_expand_globs_non_glob() {
        let temp_dir = TempDir::new().unwrap();
        let dir = temp_dir.path();
        
        let file_path = dir.join("specific_file.txt");
        fs::write(&file_path, "test").unwrap();
        
        let expanded = expand_globs(&[file_path.clone()]).unwrap();
        
        assert_eq!(expanded.len(), 1);
        assert_eq!(expanded[0], file_path);
    }
    
    #[test]
    fn test_process_input_args_with_stdin_flag() {
        // This test would require mocking stdin, so we skip it for now
        // In real usage, this would test the stdin_flag path
    }
}