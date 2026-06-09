use smartfo::skill::{SkillGenerator, SkillMetadata, CommandDoc, FlagDoc, check_skill_stale};

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
fn test_skill_metadata() {
    let metadata = SkillMetadata {
        name: "test-skill".to_string(),
        description: "Test description".to_string(),
        version: "1.0.0".to_string(),
        triggers: vec!["test".to_string()],
    };
    
    assert_eq!(metadata.name, "test-skill");
    assert_eq!(metadata.description, "Test description");
    assert_eq!(metadata.version, "1.0.0");
    assert_eq!(metadata.triggers.len(), 1);
}

#[test]
fn test_command_doc() {
    let doc = CommandDoc {
        name: "test".to_string(),
        description: "Test command".to_string(),
        examples: vec!["test arg1".to_string()],
        flags: vec![],
    };
    
    // Just verify the struct can be created
    assert_eq!(doc.examples.len(), 1);
}

#[test]
fn test_flag_doc() {
    let flag = FlagDoc {
        name: "--test".to_string(),
        short: Some("-t".to_string()),
        description: "Test flag".to_string(),
    };
    
    // Just verify the struct can be created
    assert!(flag.short.is_some());
}

#[test]
fn test_extract_version_from_skill() {
    let skill_content = r#"---
name: smartfo
description: Test
version: 1.0.0
---
"#;
    
    let is_stale = check_skill_stale(skill_content).unwrap();
    // Version mismatch should be stale
    assert!(is_stale);
}

#[test]
fn test_check_skill_stale_current() {
    let current_version = env!("CARGO_PKG_VERSION");
    let skill_content = format!(r#"---
name: smartfo
description: Test
version: {}
---
"#, current_version);
    
    let result = check_skill_stale(&skill_content);
    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[test]
fn test_check_skill_stale_old() {
    let skill_content = r#"---
name: smartfo
description: Test
version: 0.0.1
---
"#;
    
    let result = check_skill_stale(skill_content);
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
fn test_check_skill_stale_missing_version() {
    let skill_content = r#"---
name: smartfo
description: Test
---
"#;
    
    let result = check_skill_stale(skill_content);
    assert!(result.is_err());
}

#[test]
fn test_skill_generator_add_command() {
    let mut generator = SkillGenerator::new();
    
    let doc = CommandDoc {
        name: "custom".to_string(),
        description: "Custom command".to_string(),
        examples: vec!["custom arg".to_string()],
        flags: vec![],
    };
    
    generator.add_command(doc);
    let skill = generator.generate().unwrap();
    
    // The skill should now include the custom command
    assert!(skill.body.contains("custom"));
}

#[test]
fn test_skill_triggers() {
    let generator = SkillGenerator::new();
    let skill = generator.generate().unwrap();
    
    assert!(!skill.metadata.triggers.is_empty());
    assert!(skill.metadata.triggers.contains(&"move files".to_string()));
    assert!(skill.metadata.triggers.contains(&"remove files".to_string()));
}

#[test]
fn test_skill_body_sections() {
    let mut generator = SkillGenerator::new();
    let skill = generator.generate_with_defaults().unwrap();
    
    assert!(skill.body.contains("# Smartfo Agent Skill"));
    assert!(skill.body.contains("## When to Use"));
    assert!(skill.body.contains("## Available Commands"));
    assert!(skill.body.contains("## Output Formats"));
    assert!(skill.body.contains("## Mode Selection"));
    assert!(skill.body.contains("## Field Selection"));
    assert!(skill.body.contains("## Content Truncation"));
    assert!(skill.body.contains("## Session Context"));
    assert!(skill.body.contains("## Installation"));
    assert!(skill.body.contains("## Notes"));
}

#[test]
fn test_skill_non_interactive_examples() {
    let mut generator = SkillGenerator::new();
    let skill = generator.generate_with_defaults().unwrap();
    
    // All examples should be non-interactive (no prompts)
    assert!(!skill.body.contains("prompt"));
    assert!(!skill.body.contains("ask"));
    assert!(!skill.body.contains("confirm"));
}

#[test]
fn test_skill_static_content() {
    let mut generator = SkillGenerator::new();
    let skill = generator.generate_with_defaults().unwrap();
    
    // Should not contain dynamic data like current directory, timestamps, etc.
    let current_dir = std::env::current_dir().unwrap().display().to_string();
    assert!(!skill.body.contains(&current_dir));
    assert!(!skill.body.contains("2025-")); // No hardcoded years
    assert!(!skill.body.contains("/Users/")); // No user paths
}
