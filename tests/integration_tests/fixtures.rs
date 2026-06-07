use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile::TempDir;

/// Fixture for creating a temporary Git repository
pub fn create_git_repo() -> TempDir {
    let tmp = TempDir::new().unwrap();
    let repo_path = tmp.path();

    Command::new("git")
        .arg("init")
        .current_dir(repo_path)
        .output()
        .expect("Failed to init git repo");

    Command::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(repo_path)
        .output()
        .expect("Failed to set git user.name");

    Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(repo_path)
        .output()
        .expect("Failed to set git user.email");

    tmp
}

/// Fixture for creating a temporary Mercurial repository
pub fn create_hg_repo() -> TempDir {
    let tmp = TempDir::new().unwrap();
    let repo_path = tmp.path();

    Command::new("hg")
        .arg("init")
        .current_dir(repo_path)
        .output()
        .expect("Failed to init hg repo");

    Command::new("hg")
        .args(["config", "ui.username", "Test User <test@example.com>"])
        .current_dir(repo_path)
        .output()
        .expect("Failed to set hg username");

    tmp
}

/// Fixture for creating a temporary SVN repository
pub fn create_svn_repo() -> TempDir {
    let tmp = TempDir::new().unwrap();
    let repo_path = tmp.path();

    Command::new("svnadmin")
        .arg("create")
        .arg(repo_path)
        .output()
        .expect("Failed to create svn repo");

    tmp
}

/// Fixture for creating a temporary Jujutsu repository
pub fn create_jj_repo() -> TempDir {
    let tmp = TempDir::new().unwrap();
    let repo_path = tmp.path();

    Command::new("jj")
        .arg("init")
        .current_dir(repo_path)
        .output()
        .expect("Failed to init jj repo");

    Command::new("jj")
        .args(["config", "set", "user.name", "Test User"])
        .current_dir(repo_path)
        .output()
        .expect("Failed to set jj user.name");

    Command::new("jj")
        .args(["config", "set", "user.email", "test@example.com"])
        .current_dir(repo_path)
        .output()
        .expect("Failed to set jj user.email");

    tmp
}

/// Create a tracked file in a Git repository
pub fn create_tracked_file(repo_path: &Path, name: &str, content: &str) {
    let file_path = repo_path.join(name);
    fs::write(&file_path, content).expect("Failed to write file");

    Command::new("git")
        .args(["add", name])
        .current_dir(repo_path)
        .output()
        .expect("Failed to git add");

    Command::new("git")
        .args(["commit", "-m", &format!("Add {}", name)])
        .current_dir(repo_path)
        .output()
        .expect("Failed to git commit");
}

/// Create an untracked file in a repository
pub fn create_untracked_file(repo_path: &Path, name: &str, content: &str) {
    let file_path = repo_path.join(name);
    fs::write(&file_path, content).expect("Failed to write file");
}

/// Create an ignored file in a Git repository
pub fn create_ignored_file(repo_path: &Path, name: &str, content: &str) {
    let gitignore_path = repo_path.join(".gitignore");
    fs::write(&gitignore_path, name).expect("Failed to write .gitignore");

    Command::new("git")
        .args(["add", ".gitignore"])
        .current_dir(repo_path)
        .output()
        .expect("Failed to git add .gitignore");

    Command::new("git")
        .args(["commit", "-m", "Add .gitignore"])
        .current_dir(repo_path)
        .output()
        .expect("Failed to commit .gitignore");

    let file_path = repo_path.join(name);
    fs::write(&file_path, content).expect("Failed to write file");
}
