use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;
use tracing::{info, warn, debug};

/// Install/uninstall operations for smartfo
pub struct Installer {
    bin_dir: PathBuf,
    config_dir: PathBuf,
    man_dir: PathBuf,
    completion_dirs: CompletionDirs,
}

struct CompletionDirs {
    bash: PathBuf,
    zsh: PathBuf,
    fish: PathBuf,
}

impl Installer {
    pub fn new() -> Result<Self> {
        let bin_dir = Self::get_bin_dir()?;
        let config_dir = Self::get_config_dir()?;
        let man_dir = Self::get_man_dir()?;
        let completion_dirs = Self::get_completion_dirs()?;

        Ok(Self {
            bin_dir,
            config_dir,
            man_dir,
            completion_dirs,
        })
    }

    fn get_bin_dir() -> Result<PathBuf> {
        if let Ok(dir) = std::env::var("XDG_BIN_HOME") {
            Ok(PathBuf::from(dir))
        } else {
            let home = std::env::var("HOME").context("HOME environment variable not set")?;
            Ok(PathBuf::from(home).join(".local").join("bin"))
        }
    }

    fn get_config_dir() -> Result<PathBuf> {
        if let Ok(dir) = std::env::var("XDG_CONFIG_HOME") {
            Ok(PathBuf::from(dir).join("smartfo"))
        } else {
            let home = std::env::var("HOME").context("HOME environment variable not set")?;
            Ok(PathBuf::from(home).join(".config").join("smartfo"))
        }
    }

    fn get_man_dir() -> Result<PathBuf> {
        Ok(PathBuf::from("/usr/local/share/man"))
    }

    fn get_completion_dirs() -> Result<CompletionDirs> {
        let home = std::env::var("HOME").context("HOME environment variable not set")?;
        let home_path = PathBuf::from(home);

        Ok(CompletionDirs {
            bash: home_path.join(".local").join("share").join("bash-completion").join("completions"),
            zsh: home_path.join(".local").join("share").join("zsh").join("site-functions"),
            fish: home_path.join(".local").join("share").join("fish").join("completions"),
        })
    }

    /// Install smartfo: create symlinks, generate completions, initialize config
    pub fn install(&self) -> Result<()> {
        info!("Starting smartfo installation");

        // Create directories
        self.create_directories()?;

        // Create symlinks
        self.create_symlinks()?;

        // Generate shell completions
        self.generate_completions()?;

        // Initialize default config
        self.initialize_config()?;

        // Install man pages
        self.install_man_pages()?;

        info!("Installation complete");
        self.print_install_instructions();

        Ok(())
    }

    /// Uninstall smartfo: remove symlinks, completions, man pages
    pub fn uninstall(&self, force: bool) -> Result<()> {
        info!("Starting smartfo uninstallation");

        // Remove symlinks
        self.remove_symlinks()?;

        // Remove completions
        self.remove_completions()?;

        // Remove man pages
        self.remove_man_pages()?;

        // Optionally remove config
        if force {
            self.remove_config()?;
        } else {
            self.prompt_config_removal()?;
        }

        info!("Uninstallation complete");
        Ok(())
    }

    fn create_directories(&self) -> Result<()> {
        fs::create_dir_all(&self.bin_dir).context("Failed to create bin directory")?;
        fs::create_dir_all(&self.config_dir).context("Failed to create config directory")?;
        fs::create_dir_all(&self.completion_dirs.bash).context("Failed to create bash completion directory")?;
        fs::create_dir_all(&self.completion_dirs.zsh).context("Failed to create zsh completion directory")?;
        fs::create_dir_all(&self.completion_dirs.fish).context("Failed to create fish completion directory")?;
        Ok(())
    }

    fn create_symlinks(&self) -> Result<()> {
        let smartfo_path = std::env::current_exe().context("Failed to get current executable path")?;

        let symlinks = ["mv", "rm", "smv", "srm"];
        for link_name in symlinks {
            let link_path = self.bin_dir.join(link_name);
            if link_path.exists() {
                warn!("Symlink {} already exists, skipping", link_name);
                continue;
            }
            std::os::unix::fs::symlink(&smartfo_path, &link_path)
                .with_context(|| format!("Failed to create symlink {}", link_name))?;
            debug!("Created symlink: {}", link_path.display());
        }
        Ok(())
    }

    fn generate_completions(&self) -> Result<()> {
        // TODO: Generate completions using clap
        // This will be implemented in subsequent sub-tasks
        info!("Shell completion generation not yet implemented");
        Ok(())
    }

    fn initialize_config(&self) -> Result<()> {
        let config_path = self.config_dir.join("config.toml");
        if config_path.exists() {
            info!("Config file already exists, skipping initialization");
            return Ok(());
        }

        // TODO: Initialize default config
        // This will be implemented in subsequent sub-tasks
        info!("Config initialization not yet implemented");
        Ok(())
    }

    fn install_man_pages(&self) -> Result<()> {
        // TODO: Install man pages
        // This will be implemented in subsequent sub-tasks
        info!("Man page installation not yet implemented");
        Ok(())
    }

    fn remove_symlinks(&self) -> Result<()> {
        let symlinks = ["mv", "rm", "smv", "srm"];
        for link_name in symlinks {
            let link_path = self.bin_dir.join(link_name);
            if link_path.exists() {
                fs::remove_file(&link_path)
                    .with_context(|| format!("Failed to remove symlink {}", link_name))?;
                debug!("Removed symlink: {}", link_name);
            }
        }
        Ok(())
    }

    fn remove_completions(&self) -> Result<()> {
        // TODO: Remove completion scripts
        // This will be implemented in subsequent sub-tasks
        info!("Completion removal not yet implemented");
        Ok(())
    }

    fn remove_man_pages(&self) -> Result<()> {
        // TODO: Remove man pages
        // This will be implemented in subsequent sub-tasks
        info!("Man page removal not yet implemented");
        Ok(())
    }

    fn remove_config(&self) -> Result<()> {
        if self.config_dir.exists() {
            fs::remove_dir_all(&self.config_dir)
                .with_context(|| format!("Failed to remove config directory {}", self.config_dir.display()))?;
            debug!("Removed config directory: {}", self.config_dir.display());
        }
        Ok(())
    }

    fn prompt_config_removal(&self) -> Result<()> {
        if !self.config_dir.exists() {
            return Ok(());
        }

        // TODO: Implement interactive prompt
        // For now, skip config removal without force flag
        info!("Config directory preserved. Use --force to remove config files.");
        Ok(())
    }

    fn print_install_instructions(&self) {
        println!("smartfo installed successfully!");
        println!();
        println!("Binaries installed to: {}", self.bin_dir.display());
        println!("Config directory: {}", self.config_dir.display());
        println!();
        println!("Environment variables:");
        println!("  SMARTFO_CONFIG_HOME={}", self.config_dir.display());
        println!();
        println!("Make sure {} is in your PATH.", self.bin_dir.display());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_installer_creation() {
        let installer = Installer::new().unwrap();
        assert!(!installer.bin_dir.as_os_str().is_empty());
        assert!(!installer.config_dir.as_os_str().is_empty());
    }

    #[test]
    fn test_get_bin_dir() {
        let bin_dir = Installer::get_bin_dir().unwrap();
        assert!(bin_dir.ends_with("bin") || bin_dir.ends_with(".local/bin"));
    }

    #[test]
    fn test_get_config_dir() {
        let config_dir = Installer::get_config_dir().unwrap();
        assert!(config_dir.ends_with("smartfo"));
    }
}
