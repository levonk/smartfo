use anyhow::Result;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Skill metadata for frontmatter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillMetadata {
    /// Skill name
    pub name: String,
    /// Skill description
    pub description: String,
    /// Skill version
    pub version: String,
    /// Triggers that activate this skill
    pub triggers: Vec<String>,
}

/// Generated skill content
#[derive(Debug, Clone)]
pub struct GeneratedSkill {
    /// Skill metadata
    pub metadata: SkillMetadata,
    /// Skill body content
    pub body: String,
}

impl GeneratedSkill {
    /// Generate complete SKILL.md content
    pub fn to_markdown(&self) -> String {
        let mut output = String::new();
        
        // Add frontmatter
        output.push_str("---\n");
        output.push_str(&format!("name: {}\n", self.metadata.name));
        output.push_str(&format!("description: {}\n", self.metadata.description));
        output.push_str(&format!("version: {}\n", self.metadata.version));
        if !self.metadata.triggers.is_empty() {
            output.push_str("triggers:\n");
            for trigger in &self.metadata.triggers {
                output.push_str(&format!("  - {}\n", trigger));
            }
        }
        output.push_str("---\n\n");
        
        // Add body
        output.push_str(&self.body);
        
        output
    }
}

/// Skill generator for creating agent skills from CLI metadata
pub struct SkillGenerator {
    /// CLI metadata
    metadata: SkillMetadata,
    /// Command documentation
    commands: HashMap<String, CommandDoc>,
}

/// Command documentation
#[derive(Debug, Clone)]
pub struct CommandDoc {
    /// Command name
    pub name: String,
    /// Command description
    pub description: String,
    /// Usage examples (non-interactive)
    pub examples: Vec<String>,
    /// Available flags
    pub flags: Vec<FlagDoc>,
}

/// Flag documentation
#[derive(Debug, Clone)]
pub struct FlagDoc {
    /// Flag name (e.g., "--force")
    pub name: String,
    /// Short form (e.g., "-f")
    pub short: Option<String>,
    /// Description
    pub description: String,
}

impl SkillGenerator {
    /// Create a new skill generator
    pub fn new() -> Self {
        Self {
            metadata: SkillMetadata {
                name: "smartfo".to_string(),
                description: "VCS-aware, safe, non-blocking replacement for mv and rm commands".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                triggers: vec![
                    "move files".to_string(),
                    "remove files".to_string(),
                    "delete files".to_string(),
                    "smartfo".to_string(),
                    "safe delete".to_string(),
                    "vcs move".to_string(),
                ],
            },
            commands: HashMap::new(),
        }
    }

    /// Add command documentation
    pub fn add_command(&mut self, doc: CommandDoc) {
        self.commands.insert(doc.name.clone(), doc);
    }

    /// Generate the skill
    pub fn generate(&self) -> Result<GeneratedSkill> {
        let mut body = String::new();

        // Add overview
        body.push_str("# Smartfo Agent Skill\n\n");
        body.push_str("Smartfo is a VCS-aware, safe, non-blocking replacement for `mv` and `rm` commands. ");
        body.push_str("It provides automatic trash management, async operations, and comprehensive audit logging.\n\n");

        // Add trigger information
        body.push_str("## When to Use\n\n");
        body.push_str("Use this skill when you need to:\n");
        body.push_str("- Move or rename files with VCS awareness\n");
        body.push_str("- Safely remove files (trash instead of delete)\n");
        body.push_str("- Perform async operations for large files\n");
        body.push_str("- Maintain audit trails for file operations\n\n");

        // Add commands section
        body.push_str("## Available Commands\n\n");
        
        // Add mv command
        if let Some(mv_doc) = self.commands.get("mv") {
            body.push_str("### mv - Move Files\n\n");
            body.push_str(&format!("{}\n\n", mv_doc.description));
            body.push_str("**Usage:**\n");
            for example in &mv_doc.examples {
                body.push_str(&format!("- `{}`\n", example));
            }
            body.push_str("\n**Flags:**\n");
            for flag in &mv_doc.flags {
                let flag_str = if let Some(ref short) = flag.short {
                    format!("`{}`, `{}`", short, flag.name)
                } else {
                    format!("`{}`", flag.name)
                };
                body.push_str(&format!("- {}: {}\n", flag_str, flag.description));
            }
            body.push_str("\n");
        }

        // Add rm command
        if let Some(rm_doc) = self.commands.get("rm") {
            body.push_str("### rm - Remove Files\n\n");
            body.push_str(&format!("{}\n\n", rm_doc.description));
            body.push_str("**Usage:**\n");
            for example in &rm_doc.examples {
                body.push_str(&format!("- `{}`\n", example));
            }
            body.push_str("\n**Flags:**\n");
            for flag in &rm_doc.flags {
                let flag_str = if let Some(ref short) = flag.short {
                    format!("`{}`, `{}`", short, flag.name)
                } else {
                    format!("`{}`", flag.name)
                };
                body.push_str(&format!("- {}: {}\n", flag_str, flag.description));
            }
            body.push_str("\n");
        }

        // Add smartfo commands
        if let Some(list_doc) = self.commands.get("list") {
            body.push_str("### list - List Operations\n\n");
            body.push_str(&format!("{}\n\n", list_doc.description));
            body.push_str("**Usage:**\n");
            for example in &list_doc.examples {
                body.push_str(&format!("- `{}`\n", example));
            }
            body.push_str("\n");
        }

        if let Some(status_doc) = self.commands.get("status") {
            body.push_str("### status - Show Status\n\n");
            body.push_str(&format!("{}\n\n", status_doc.description));
            body.push_str("**Usage:**\n");
            for example in &status_doc.examples {
                body.push_str(&format!("- `{}`\n", example));
            }
            body.push_str("\n");
        }

        // Add output format section
        body.push_str("## Output Formats\n\n");
        body.push_str("Smartfo supports multiple output formats for agent consumption:\n\n");
        body.push_str("- **TOON**: Token-efficient format (default in agent mode)\n");
        body.push_str("- **JSON**: Structured JSON output\n");
        body.push_str("- **Human**: Human-readable text output\n\n");
        body.push_str("Use `--toon` or `--format=toon` to request TOON format.\n\n");

        // Add mode selection section
        body.push_str("## Mode Selection\n\n");
        body.push_str("Smartfo automatically detects agent sessions and defaults to agent-optimized output. ");
        body.push_str("You can explicitly control mode with:\n\n");
        body.push_str("- `--agent`: Force agent mode (TOON output, minimal fields)\n");
        body.push_str("- `--human`: Force human mode (friendly messages, full output)\n\n");

        // Add field selection section
        body.push_str("## Field Selection\n\n");
        body.push_str("Reduce token consumption by selecting specific output fields:\n\n");
        body.push_str("```bash\n");
        body.push_str("smartfo list --fields=id,status,source\n");
        body.push_str("smartfo status --fields=operation,queue_size\n");
        body.push_str("```\n\n");

        // Add truncation section
        body.push_str("## Content Truncation\n\n");
        body.push_str("Large text fields are automatically truncated to 1000 characters by default. ");
        body.push_str("Use `--full` to disable truncation and show complete content.\n\n");

        // Add session context section
        body.push_str("## Session Context\n\n");
        body.push_str("Smartfo provides session context for agent awareness:\n");
        body.push_str("- Current working directory\n");
        body.push_str("- Git repository root (if in a repo)\n");
        body.push_str("- Recent operations count\n");
        body.push_str("- Queue size (if daemon is running)\n\n");

        // Add installation section
        body.push_str("## Installation\n\n");
        body.push_str("Smartfo is installed via symlinks. Use `smartfo --install` to set up:\n");
        body.push_str("- `mv` symlink for move operations\n");
        body.push_str("- `rm` symlink for remove operations\n");
        body.push_str("- Git hooks for VCS integration\n\n");

        // Add notes section
        body.push_str("## Notes\n\n");
        body.push_str("- All `rm` operations are async by default (files moved to trash)\n");
        body.push_str("- VCS-aware operations use native commands (git mv, hg mv, etc.)\n");
        body.push_str("- Audit logs are maintained in `$HOME/smartfo/audit/operations.jsonl`\n");
        body.push_str("- Use `--plain` to disable all smart features for POSIX compatibility\n\n");

        Ok(GeneratedSkill {
            metadata: self.metadata.clone(),
            body,
        })
    }

    /// Generate skill with default command documentation
    pub fn generate_with_defaults(&mut self) -> Result<GeneratedSkill> {
        // Add mv command documentation
        self.add_command(CommandDoc {
            name: "mv".to_string(),
            description: "Move (rename) files with VCS awareness and async support".to_string(),
            examples: vec![
                "mv file1 file2".to_string(),
                "mv file1 dir/".to_string(),
                "mv --async large.bin /mnt/backup/".to_string(),
                "mv --force-outside-vcs tracked.txt /tmp/".to_string(),
            ],
            flags: vec![
                FlagDoc {
                    name: "--force".to_string(),
                    short: Some("-f".to_string()),
                    description: "Do not prompt before overwriting".to_string(),
                },
                FlagDoc {
                    name: "--interactive".to_string(),
                    short: Some("-i".to_string()),
                    description: "Prompt before overwrite".to_string(),
                },
                FlagDoc {
                    name: "--no-clobber".to_string(),
                    short: Some("-n".to_string()),
                    description: "Do not overwrite an existing file".to_string(),
                },
                FlagDoc {
                    name: "--verbose".to_string(),
                    short: Some("-v".to_string()),
                    description: "Explain what is being done".to_string(),
                },
                FlagDoc {
                    name: "--async".to_string(),
                    short: None,
                    description: "Force async move even for small/same-fs files".to_string(),
                },
                FlagDoc {
                    name: "--blocking".to_string(),
                    short: None,
                    description: "Wait for operation to complete".to_string(),
                },
                FlagDoc {
                    name: "--force-outside-vcs".to_string(),
                    short: None,
                    description: "Allow moving tracked files outside repo".to_string(),
                },
                FlagDoc {
                    name: "--plain".to_string(),
                    short: None,
                    description: "Disable all smart features; behave exactly like POSIX mv".to_string(),
                },
            ],
        });

        // Add rm command documentation
        self.add_command(CommandDoc {
            name: "rm".to_string(),
            description: "Remove files by moving to trash (async by default)".to_string(),
            examples: vec![
                "rm file.txt".to_string(),
                "rm -r directory/".to_string(),
                "rm --force file.txt".to_string(),
                "rm --blocking file.txt".to_string(),
            ],
            flags: vec![
                FlagDoc {
                    name: "--force".to_string(),
                    short: Some("-f".to_string()),
                    description: "Ignore nonexistent files and arguments, never prompt".to_string(),
                },
                FlagDoc {
                    name: "--interactive".to_string(),
                    short: Some("-i".to_string()),
                    description: "Prompt before every removal".to_string(),
                },
                FlagDoc {
                    name: "--recursive".to_string(),
                    short: Some("-r".to_string()),
                    description: "Remove directories and their contents recursively".to_string(),
                },
                FlagDoc {
                    name: "--verbose".to_string(),
                    short: Some("-v".to_string()),
                    description: "Explain what is being done".to_string(),
                },
                FlagDoc {
                    name: "--blocking".to_string(),
                    short: None,
                    description: "Wait for operation to complete (sync mode)".to_string(),
                },
                FlagDoc {
                    name: "--plain".to_string(),
                    short: None,
                    description: "Disable all smart features; behave exactly like POSIX rm".to_string(),
                },
            ],
        });

        // Add list command documentation
        self.add_command(CommandDoc {
            name: "list".to_string(),
            description: "List queued and completed operations".to_string(),
            examples: vec![
                "smartfo list".to_string(),
                "smartfo list --all".to_string(),
                "smartfo list --limit 10".to_string(),
                "smartfo list --fields=id,status,source".to_string(),
            ],
            flags: vec![],
        });

        // Add status command documentation
        self.add_command(CommandDoc {
            name: "status".to_string(),
            description: "Show daemon and queue status".to_string(),
            examples: vec![
                "smartfo status".to_string(),
                "smartfo status --detailed".to_string(),
                "smartfo status --fields=operation,queue_size".to_string(),
            ],
            flags: vec![],
        });

        self.generate()
    }
}

impl Default for SkillGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Check if the generated skill is stale compared to the current version
pub fn check_skill_stale(current_skill_content: &str) -> Result<bool> {
    // Extract version from current skill
    let current_version = extract_version_from_skill(current_skill_content)?;
    let expected_version = env!("CARGO_PKG_VERSION");
    
    Ok(current_version != expected_version)
}

/// Extract version from skill markdown
fn extract_version_from_skill(content: &str) -> Result<String> {
    let lines: Vec<&str> = content.lines().collect();
    
    for line in lines {
        if line.starts_with("version:") {
            let version = line.trim_start_matches("version:").trim();
            return Ok(version.to_string());
        }
    }
    
    anyhow::bail!("Version not found in skill content");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skill_generator_basic() {
        let generator = SkillGenerator::new();
        let skill = generator.generate().unwrap();
        
        assert_eq!(skill.metadata.name, "smartfo");
        assert!(!skill.metadata.description.is_empty());
        assert!(!skill.body.is_empty());
    }

    #[test]
    fn test_skill_generator_with_defaults() {
        let mut generator = SkillGenerator::new();
        let skill = generator.generate_with_defaults().unwrap();
        
        assert!(skill.body.contains("mv - Move Files"));
        assert!(skill.body.contains("rm - Remove Files"));
        assert!(skill.body.contains("list - List Operations"));
        assert!(skill.body.contains("status - Show Status"));
    }

    #[test]
    fn test_generated_skill_markdown() {
        let mut generator = SkillGenerator::new();
        let skill = generator.generate_with_defaults().unwrap();
        let markdown = skill.to_markdown();
        
        assert!(markdown.starts_with("---"));
        assert!(markdown.contains("name: smartfo"));
        assert!(markdown.contains("description:"));
        assert!(markdown.contains("version:"));
        assert!(markdown.contains("# Smartfo Agent Skill"));
    }

    #[test]
    fn test_extract_version_from_skill() {
        let skill_content = r#"---
name: smartfo
description: Test
version: 1.0.0
---
"#;
        
        let version = extract_version_from_skill(skill_content).unwrap();
        assert_eq!(version, "1.0.0");
    }

    #[test]
    fn test_check_skill_stale() {
        let current_version = env!("CARGO_PKG_VERSION");
        let skill_content = format!(r#"---
name: smartfo
description: Test
version: {}
---
"#, current_version);
        
        assert!(!check_skill_stale(&skill_content).unwrap());
        
        let stale_content = r#"---
name: smartfo
description: Test
version: 0.0.1
---
"#;
        
        assert!(check_skill_stale(stale_content).unwrap());
    }
}
