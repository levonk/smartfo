# Feature: Install Alias Warnings

## Overview
When `--install` is called, smartfo should check for existing shell aliases that cover the `mv`, `rm`, `smv`, or `srm` commands. If such aliases are found, the user should be warned with instructions on how to remove them.

## Requirements

### Alias Detection
- Detect shell aliases for the following commands: `mv`, `rm`, `smv`, `srm`
- Support detection in common shells: bash, zsh, fish
- Check both current shell session and shell configuration files

### Warning Messages
- When aliases are detected, display a warning message
- Warning format: "Warning: to remove existing aliases, run `unalias <command>`"
- List all detected aliases with their removal commands
- Provide guidance on removing persistent aliases from shell config files

### Integration
- Perform alias check after symlink creation in the install process
- Do not fail installation if aliases are found (only warn)
- Support a `--force` flag to bypass alias warnings

## Implementation Notes

### Shell Detection
- Detect current shell from `$SHELL` environment variable
- Fall back to common shell locations if `$SHELL` is not set

### Alias Sources
- Check current shell session using `alias` command
- Check shell configuration files:
  - bash: `~/.bashrc`, `~/.bash_profile`, `~/.profile`
  - zsh: `~/.zshrc`, `~/.zprofile`
  - fish: `~/.config/fish/config.fish`

### Alias Patterns
- Look for alias definitions matching the target commands
- Handle different alias syntax across shells
- Ignore aliases that point to smartfo itself (no warning needed)

## Acceptance Criteria
- [x] Alias detection works for bash, zsh, and fish
- [x] Warning messages are displayed for detected aliases
- [x] Installation continues despite alias warnings
- [x] `--force` flag bypasses alias warnings
- [x] Aliases pointing to smartfo are not flagged
- [x] Documentation is updated to describe alias warnings
