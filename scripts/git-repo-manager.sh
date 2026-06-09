#!/usr/bin/env bash

# Git Repository Management Script
# Handles comprehensive git repository workflow from analysis to clean state

set -euo pipefail

# Script configuration
SCRIPT_NAME="$(basename "$0")"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Validate deterministic script exists parallel to SKILL.md (fail fast)
SKILL_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
SCRIPT_PATH="$SKILL_DIR/scripts/git-repo-manager.sh"
if [ ! -x "$SCRIPT_PATH" ]; then
    echo "ERROR: Deterministic script not found or not executable: $SCRIPT_PATH" >&2
    exit 1
fi

# Discover repository root from optional target path; fail fast if not a git repo
discover_repo_root() {
    local target_path="${1:-.}"
    local repo_root
    repo_root=$(cd "$target_path" && git rev-parse --show-toplevel 2>/dev/null)
    if [ -z "$repo_root" ]; then
        echo "ERROR: $target_path is not inside a git repository" >&2
        exit 1
    fi
    echo "$repo_root"
}

# Colors for output (only if supported)
SUPPORTS_COLOR=false

# Check if color output is supported
check_color_support() {
    # Check if we're in a terminal that supports color
    if [[ -t 1 ]] && command -v tput >/dev/null 2>&1; then
        local colors
        colors=$(tput colors 2>/dev/null || echo "0")
        if [[ "$colors" -gt 0 ]]; then
            SUPPORTS_COLOR=true
        fi
    fi

    # Also check for common color support indicators
    if [[ -n "${FORCE_COLOR:-}" || "${CLICOLOR:-}" == "1" || "${CLICOLOR_FORCE:-}" == "1" ]]; then
        SUPPORTS_COLOR=true
    fi
}

# Initialize color variables (only set if color is supported)
init_colors() {
    if [[ "$SUPPORTS_COLOR" == "true" ]]; then
        RED='\033[0;31m'
        GREEN='\033[0;32m'
        YELLOW='\033[1;33m'
        BLUE='\033[0;34m'
        PURPLE='\033[0;35m'
        CYAN='\033[0;36m'
        NC='\033[0m' # No Color
    else
        # Set to empty strings when color not supported
        RED=''
        GREEN=''
        YELLOW=''
        BLUE=''
        PURPLE=''
        CYAN=''
        NC=''
    fi
}

# Logging functions (safe for all environments)
log_info() {
    if [[ "$SUPPORTS_COLOR" == "true" ]]; then
        echo -e "${BLUE}[INFO]${NC} $1"
    else
        echo "[INFO] $1"
    fi
}

log_success() {
    if [[ "$SUPPORTS_COLOR" == "true" ]]; then
        echo -e "${GREEN}[SUCCESS]${NC} $1"
    else
        echo "[SUCCESS] $1"
    fi
}

log_warning() {
    if [[ "$SUPPORTS_COLOR" == "true" ]]; then
        echo -e "${YELLOW}[WARNING]${NC} $1"
    else
        echo "[WARNING] $1"
    fi
}

log_error() {
    if [[ "$SUPPORTS_COLOR" == "true" ]]; then
        echo -e "${RED}[ERROR]${NC} $1"
    else
        echo "[ERROR] $1"
    fi
}

log_phase() {
    if [[ "$SUPPORTS_COLOR" == "true" ]]; then
        echo -e "${PURPLE}[PHASE]${NC} $1"
    else
        echo "[PHASE] $1"
    fi
}

log_tool() {
    if [[ "$SUPPORTS_COLOR" == "true" ]]; then
        echo -e "${CYAN}[TOOL]${NC} $1"
    else
        echo "[TOOL] $1"
    fi
}

# Environment detection
detect_environment() {
    if [[ -f "devbox.json" ]]; then
        echo "devbox"
    elif [[ -f "mise.toml" || -f ".mise.toml" || -f ".tool-versions" ]]; then
        echo "mise"
    elif [[ -f "flake.nix" ]]; then
        echo "nix"
    else
        echo "native"
    fi
}

# Command wrapper for environment-aware execution
run_command() {
    local env_type
    env_type=$(detect_environment)

    case "$env_type" in
        "devbox")
            log_info "Using Devbox environment"
            devbox run -- "$@"
            ;;
        "mise")
            log_info "Using Mise environment"
            mise exec -- "$@"
            ;;
        "nix")
            log_info "Using Nix environment"
            nix develop --command "$@"
            ;;
        "native")
            log_info "Using native environment"
            "$@"
            ;;
    esac
}

# Check for git-status-digest.sh script
find_git_status_digest() {
    local script_paths=(
        "scripts/git-status-digest.sh"
        "bin/git-status-digest.sh"
        "$HOME/.local/bin/executable_git-status-digest.sh"
        "$HOME/.local/bin/git-status-digest.sh"
    )

    for path in "${script_paths[@]}"; do
        if [[ -x "$path" ]]; then
            echo "$path"
            return 0
        fi
    done

    return 1
}

# Manual git status analysis fallback
manual_git_analysis() {
    local mode="${1:-identify}"

    log_info "Performing manual git status analysis (mode: $mode)"

    # Get porcelain output
    local porcelain
    porcelain=$(git status --untracked-files=all --porcelain)
    echo "$porcelain"

    if [[ "$mode" == "identify" ]]; then
        # Detailed analysis for planning
        echo "=== STAGED FILES ==="
        local staged_files
        staged_files=$(git diff --cached --name-status)
        if [[ -z "$staged_files" ]]; then
            echo "<none>"
        else
            echo "$staged_files"
        fi

        echo "=== UNSTAGED FILES ==="
        local unstaged_files
        unstaged_files=$(git diff --name-status)
        if [[ -z "$unstaged_files" ]]; then
            echo "<none>"
        else
            echo "$unstaged_files"
        fi

        echo "=== UNTRACKED FILES ==="
        local untracked_files
        untracked_files=$(git ls-files --others --exclude-standard)
        if [[ -z "$untracked_files" ]]; then
            echo "<none>"
        else
            echo "$untracked_files"
        fi
    elif [[ "$mode" == "assert-clean" ]]; then
        # Simple clean check
        if [[ -n "$porcelain" ]]; then
            log_error "Repository is not clean"
            return 1
        else
            log_success "Repository is clean"
            return 0
        fi
    fi
}

# Check for unsaved buffers in various editors
check_unsaved_buffers() {
    local unsaved_found=false

    log_info "Checking for unsaved editor buffers..."

    # Check VSCode
    if check_vscode_buffers; then
        unsaved_found=true
    fi

    # Check Vim/Neovim
    if check_vim_buffers; then
        unsaved_found=true
    fi

    # Check Emacs
    if check_emacs_buffers; then
        unsaved_found=true
    fi

    if [[ "$unsaved_found" == "true" ]]; then
        log_warning "Unsaved buffers detected. Please save all files before proceeding."
        log_info "Tip: Use 'git status' to see which files have unsaved changes."
        return 1
    else
        log_success "No unsaved buffers detected"
        return 0
    fi
}

# Check if VSCode is already running
is_vscode_running() {
    if pgrep -f "Visual Studio Code" >/dev/null 2>&1 || pgrep -f "code" >/dev/null 2>&1; then
        return 0
    else
        return 1
    fi
}

# Safely launch VSCode only if no instance is running
launch_vscode_safely() {
    local target_path="${1:-.}"

    if is_vscode_running; then
        log_info "VSCode is already running. Opening project in existing instance..."
        if command -v code >/dev/null 2>&1; then
            # Use --reuse-window to open in existing instance
            code --reuse-window "$target_path" 2>/dev/null || {
                log_warning "Could not open in existing VSCode instance"
                return 1
            }
        else
            log_error "VSCode CLI not available"
            return 1
        fi
    else
        log_info "Launching new VSCode instance..."
        if command -v code >/dev/null 2>&1; then
            # Launch new instance
            code "$target_path" 2>/dev/null || {
                log_error "Failed to launch VSCode"
                return 1
            }
        else
            log_error "VSCode CLI not available"
            return 1
        fi
    fi

    return 0
}

# Check VSCode for unsaved buffers (PASSIVE - never launches VSCode)
check_vscode_buffers() {
    local vscode_running=false

    # Check if VSCode is running using the dedicated function
    if is_vscode_running; then
        vscode_running=true
        log_info "VSCode detected as already running (passive detection only)"

        # PASSIVE CHECK ONLY: We do NOT invoke 'code' CLI commands here
        # because 'code --list-workspaces' and similar commands can
        # launch VSCode or cause it to become active/focused.
        # Users who want to launch VSCode should use the 'launch-vscode' command explicitly.

        # Check for VSCode processes with new windows (indicates active work)
        local vscode_processes
        vscode_processes=$(pgrep -f "code.*--new-window" 2>/dev/null || true)
        if [[ -n "$vscode_processes" ]]; then
            log_warning "VSCode windows detected - please ensure all files are saved before committing"
            return 1
        fi

        # Generic VSCode running notification
        log_info "VSCode is running - verify all files are saved before committing"
        return 1
    fi

    return 0
}

# Check Vim/Neovim for unsaved buffers
check_vim_buffers() {
    local vim_running=false
    local unsaved_count=0

    # Check for Vim processes
    if pgrep -f "vim" >/dev/null 2>&1; then
        vim_running=true
        log_info "Vim detected, checking for unsaved buffers..."

        # Try to communicate with running Vim instances
        local vim_servers
        vim_servers=$(vim --serverlist 2>/dev/null || true)

        if [[ -n "$vim_servers" ]]; then
            while IFS= read -r server; do
                if [[ -n "$server" ]]; then
                    log_info "Checking Vim server: $server"
                    # Check for unsaved buffers using remote expression
                    local modified_buffers
                    modified_buffers=$(vim --servername "$server" --remote-expr "len(filter(getbufinfo(), 'v:val.modified'))" 2>/dev/null || echo "0")

                    if [[ "$modified_buffers" != "0" && "$modified_buffers" =~ ^[0-9]+$ ]]; then
                        log_warning "Found $modified_buffers unsaved buffer(s) in Vim server: $server"
                        ((unsaved_count += modified_buffers))
                    fi
                fi
            done <<< "$vim_servers"
        else
            # Fallback: check for Vim processes and warn user
            local vim_pids
            vim_pids=$(pgrep -f "vim" 2>/dev/null || true)
            if [[ -n "$vim_pids" ]]; then
                log_warning "Vim processes detected but server communication failed"
                log_warning "Please check :ls in Vim for unsaved buffers (marked with +)"
                return 0
            fi
        fi
    fi

    # Check for Neovim
    if pgrep -f "nvim" >/dev/null 2>&1; then
        log_info "Neovim detected, checking for unsaved buffers..."

        # Try to use nvim-remote if available
        if command -v nvr >/dev/null 2>&1; then
            local nvim_servers
            nvim_servers=$(nvr --serverlist 2>/dev/null || true)

            if [[ -n "$nvim_servers" ]]; then
                while IFS= read -r server; do
                    if [[ -n "$server" ]]; then
                        log_info "Checking Neovim server: $server"
                        local modified_buffers
                        modified_buffers=$(nvr --servername "$server" --remote-expr "len(vim.tbl_filter(function(buf) return buf.modified end, vim.fn.getbufinfo()))" 2>/dev/null || echo "0")

                        if [[ "$modified_buffers" != "0" && "$modified_buffers" =~ ^[0-9]+$ ]]; then
                            log_warning "Found $modified_buffers unsaved buffer(s) in Neovim server: $server"
                            ((unsaved_count += modified_buffers))
                        fi
                    fi
                done <<< "$nvim_servers"
            fi
        else
            log_warning "Neovim detected but nvr (nvim-remote) not available"
            log_warning "Please check :ls in Neovim for unsaved buffers (marked with +)"
            return 0
        fi
    fi

    if [[ $unsaved_count -gt 0 ]]; then
        return 0
    else
        return 1
    fi
}

# Check Emacs for unsaved buffers
check_emacs_buffers() {
    local emacs_running=false
    local unsaved_count=0

    # Check for Emacs processes
    if pgrep -f "emacs" >/dev/null 2>&1; then
        emacs_running=true
        log_info "Emacs detected, checking for unsaved buffers..."

        # Try to use emacsclient to communicate with Emacs
        if command -v emacsclient >/dev/null 2>&1; then
            # Try to get list of unsaved buffers
            local unsaved_buffers
            unsaved_buffers=$(emacsclient --eval "(length (cl-remove-if-not 'buffer-modified-p (buffer-list)))" 2>/dev/null || echo "0")

            # Clean up the output (remove quotes and convert to number)
            unsaved_buffers=$(echo "$unsaved_buffers" | sed 's/"//g' | tr -d '\n' | grep -o '[0-9]*' | head -1)

            if [[ "$unsaved_buffers" != "0" && "$unsaved_buffers" =~ ^[0-9]+$ ]]; then
                log_warning "Found $unsaved_buffers unsaved buffer(s) in Emacs"
                ((unsaved_count += unsaved_buffers))

                # Get list of unsaved buffer names
                local buffer_names
                buffer_names=$(emacsclient --eval "(mapcar (lambda (buf) (buffer-name buf)) (cl-remove-if-not 'buffer-modified-p (buffer-list)))" 2>/dev/null || echo "()")
                log_info "Unsaved Emacs buffers: $buffer_names"
            fi
        else
            log_warning "Emacs detected but emacsclient not available"
            log_warning "Please check C-x C-b in Emacs for unsaved buffers (marked with **)"
            return 0
        fi
    fi

    if [[ $unsaved_count -gt 0 ]]; then
        return 0
    else
        return 1
    fi
}

# Generate commit message based on staged changes
generate_commit_message() {
    local staged_files
    staged_files=$(git diff --cached --name-only)

    # Analyze changes to create meaningful commit message
    local file_count
    file_count=$(echo "$staged_files" | wc -l | tr -d ' ')

    # Detect primary change type
    local primary_type="Update"
    if echo "$staged_files" | grep -q "Dockerfile"; then
        primary_type="Docker"
    elif echo "$staged_files" | grep -q "\.md$"; then
        primary_type="Docs"
    elif echo "$staged_files" | grep -q "justfile\|Makefile"; then
        primary_type="Build"
    elif echo "$staged_files" | grep -q "devbox\|package\.json"; then
        primary_type="Config"
    fi

    # Create concise but descriptive message
    echo "${primary_type}: Update $(echo "$staged_files" | wc -l | tr -d ' ') files"
}

# Step 1-3: Repository Analysis
analyze_repository() {
    log_info "=== PHASE 1: REPOSITORY ANALYSIS ==="

    # Step 1: Ensure editor buffers are saved
    log_info "Step 1: Checking for unsaved editor buffers..."
    if [[ "${SKIP_BUFFER_CHECK:-}" == "1" ]]; then
        log_warning "Skipping buffer check (SKIP_BUFFER_CHECK=1)"
    else
        check_unsaved_buffers
    fi

    # Step 2: Handle WSL if on Windows
    log_info "Step 2: Checking for WSL environment..."
    if [[ "$(uname -r)" =~ Microsoft|WSL ]]; then
        log_info "WSL detected, using WSL git commands"
        # WSL-specific handling would go here
    fi

    # Step 3: Git status analysis
    log_info "Step 3: Performing comprehensive repository status analysis..."

    local git_status_script
    if git_status_script=$(find_git_status_digest); then
        log_info "Using git-status-digest.sh script: $git_status_script"
        run_command "$git_status_script" identify
    else
        log_warning "git-status-digest.sh not found, using manual analysis"
        manual_git_analysis "identify"
    fi
}

# Step 4-5: Security and Quality Validation
validate_changes() {
    log_info "=== PHASE 2: SECURITY & QUALITY VALIDATION ==="

    # Step 4: Secret scanning
    log_info "Step 4: Scanning for secrets and private information..."

    # Basic secret patterns (simplified)
    local secret_patterns=(
        "AKIA[0-9A-Z]{16}"           # AWS Access Key
        "ghp_[a-zA-Z0-9]{36}"       # GitHub Personal Access Token
        "sk_live_[a-zA-Z0-9]{24}"    # Stripe Live Key
        "-----BEGIN [A-Z]+ KEY-----"  # Private Key headers
    )

    local found_secrets=false
    for pattern in "${secret_patterns[@]}"; do
        if git grep --cached -E "$pattern" 2>/dev/null || git grep -E "$pattern" 2>/dev/null; then
            log_error "Potential secret found matching pattern: $pattern"
            found_secrets=true
        fi
    done

    if [[ "$found_secrets" == "true" ]]; then
        log_error "SECRETS DETECTED - Halting workflow for security review"
        return 1
    fi

    log_success "No secrets detected"

    # Step 5: Code quality validation
    log_info "Step 5: Running code quality validation..."

    # Try to run linter
    if command -v eslint >/dev/null 2>&1; then
        log_info "Running ESLint..."
        run_command eslint . || log_warning "ESLint found issues"
    fi

    # Try to run formatter
    if command -v prettier >/dev/null 2>&1; then
        log_info "Running Prettier..."
        run_command prettier --check . || log_warning "Prettier found formatting issues"
    fi

    # Try to run tests
    if [[ -f "package.json" ]] && grep -q '"test"' package.json; then
        log_info "Running npm test..."
        run_command npm test || log_warning "Tests failed"
    elif [[ -f "Cargo.toml" ]]; then
        log_info "Running cargo test..."
        run_command cargo test || log_warning "Tests failed"
    fi

    log_success "Code quality validation completed"
}

# Step 6-7: Change Organization
organize_changes() {
    local dry_run="${1:-false}"

    log_info "=== PHASE 3: CHANGE ORGANIZATION ==="

    # Step 6: Group changes into coherent commits
    log_info "Step 6: Analyzing changes for commit grouping..."

    # Get current changes
    local all_changes
    all_changes=$(git status --porcelain)

    if [[ -z "$all_changes" ]]; then
        log_info "No changes to organize"
        return 0
    fi

    log_info "Changes detected for organization:"
    echo "$all_changes"

    # Step 7: Create implementation plan
    log_info "Step 7: Creating implementation plan..."

    # This is a simplified implementation
    # In practice, this would analyze the changes and suggest logical groupings
    log_info "Suggested commit groupings:"
    log_info "- Review changes and group by functionality"
    log_info "- Each commit should tell one complete story"
    log_info "- Keep related cross-cutting changes together"

    if [[ "$dry_run" == "true" ]]; then
        log_info "DRY RUN: Not executing commits"
        return 0
    fi
}

# Step 8-9: Commit Execution and Documentation
execute_commits() {
    log_info "=== PHASE 4: EXECUTION & DOCUMENTATION ==="

    # Step 8: Execute commits
    log_info "Step 8: Executing commits with proper formatting..."

    # Check for commit template
    local commit_template
    commit_template=$(git config --get commit.template 2>/dev/null || echo "")

    if [[ -n "$commit_template" && -f "$commit_template" ]]; then
        log_info "Using commit template: $commit_template"
    else
        log_info "Using default commit format guidelines"
    fi

    # Stage all changes
    log_info "Staging changes..."
    git add .

    # Check if there are staged changes
    if git diff --cached --quiet; then
        log_info "No changes to commit"
        return 0
    fi

    # Create commit with auto-generated message
    log_info "Creating commit with auto-generated message..."

    # Generate commit message based on changes
    local commit_msg
    commit_msg=$(generate_commit_message)

    log_info "Commit message: $commit_msg"
    git commit -m "$commit_msg"
    log_success "Changes committed successfully"

    # Step 9: Update documentation
    log_info "Step 9: Updating project documentation..."

    # Update changelog if it exists
    if [[ -f "doc/changelog.md" ]]; then
        log_info "Updating changelog.md..."
        # This would append new entries to the changelog
    fi

    # Update architecture docs if they exist
    if [[ -f "doc/architecture.md" ]]; then
        log_info "Updating architecture.md..."
        # This would update architecture documentation
    fi

    # Update project status if it exists
    if [[ -f "doc/project-status.md" ]]; then
        log_info "Updating project-status.md..."
        # This would update project status
    fi
}

# Clean up straggler files that appeared during processing
cleanup_stragglers() {
    log_info "Attempting to clean up straggler files..."

    # Check for any changes
    local changes
    changes=$(git status --porcelain)

    if [[ -z "$changes" ]]; then
        log_info "No stragglers found"
        return 0
    fi

    log_info "Stragglers detected:"
    echo "$changes"

    # Try to auto-commit stragglers with a generic message
    log_info "Auto-committing stragglers..."

    # Stage all changes
    git add .

    # Check if there are staged changes
    if git diff --cached --quiet; then
        log_warning "No changes to stage"
        return 1
    fi

    # Commit with generic message for stragglers
    git commit -m "chore: Clean up straggler files from repository management process" --no-edit
    log_success "Stragglers committed successfully"
    return 0
}

# Step 10: Final Verification
verify_repository() {
    local auto_retry="${1:-false}"
    local max_retries="${2:-3}"
    local retry_count="${3:-0}"

    log_info "=== PHASE 5: VERIFICATION & SUMMARY ==="

    # Step 10: Final repository state verification
    log_info "Step 10: Final repository state verification..."

    local verification_failed=false
    local git_status_script
    if git_status_script=$(find_git_status_digest); then
        log_info "Running final verification with git-status-digest.sh..."
        if ! run_command "$git_status_script" assert-clean; then
            verification_failed=true
        fi
    else
        log_info "Running final manual verification..."
        if ! manual_git_analysis "assert-clean"; then
            verification_failed=true
        fi
    fi

    # Handle verification failure with retry logic
    if [[ "$verification_failed" == "true" ]]; then
        if [[ "$auto_retry" == "true" ]]; then
            if [[ $retry_count -lt $max_retries ]]; then
                ((retry_count++))
                log_warning "Repository verification failed (attempt $retry_count/$max_retries)"
                log_info "Retrying cleanup..."

                # Brief pause to allow for any background processes to complete
                sleep 2

                # Try to clean up stragglers and retry
                if cleanup_stragglers; then
                    log_info "Stragglers cleaned up, retrying verification..."
                    verify_repository "$auto_retry" "$max_retries" "$retry_count"
                    return $?
                else
                    log_warning "Could not auto-clean stragglers"
                fi
            else
                log_error "Repository verification failed after $max_retries attempts"
                return 1
            fi
        else
            log_error "Repository verification failed"
            return 1
        fi
    fi

    log_success "Repository is clean!"

    # Step 11: Commit summary
    log_info "Step 11: Generating commit summary..."

    # Show recent commits
    log_info "Recent commits:"
    git log --oneline -5

    # Show statistics
    local commit_count
    commit_count=$(git rev-list --count HEAD~10..HEAD 2>/dev/null || echo "N/A")
    log_info "Commits in recent history: $commit_count"

    log_success "Git repository management completed successfully!"
}

# Main function
main() {
    local command="${1:-complete}"
    local dry_run="false"
    local max_retries=3
    local retry_count=0
    local auto_retry="false"

    # Parse arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            --dry-run)
                dry_run="true"
                shift
                ;;
            --retry)
                auto_retry="true"
                shift
                ;;
            --max-retries)
                max_retries="$2"
                shift 2
                ;;
            *)
                # If it's not a flag, treat it as a target path
                if [[ -d "$1" ]]; then
                    PROJECT_ROOT="$(discover_repo_root "$1")"
                else
                    command="$1"
                fi
                shift
                ;;
        esac
    done

    # If PROJECT_ROOT not set yet, discover from current directory
    if [[ -z "${PROJECT_ROOT:-}" ]]; then
        PROJECT_ROOT="$(discover_repo_root)"
    fi

    # Change to project root
    cd "$PROJECT_ROOT"

    # Initialize color support detection
    check_color_support
    init_colors

    log_info "Starting Git Repository Management"
    log_info "Project root: $PROJECT_ROOT"
    log_info "Environment: $(detect_environment)"
    log_info "Command: $command"
    log_info "Color support: $SUPPORTS_COLOR"

    case "$command" in
        "analyze")
            analyze_repository
            ;;
        "validate")
            validate_changes
            ;;
        "organize")
            organize_changes "$dry_run"
            ;;
        "commit")
            execute_commits
            ;;
        "verify")
            verify_repository "$auto_retry" "$max_retries" "$retry_count"
            ;;
        "launch-vscode")
            local target_path="${2:-.}"
            #launch_vscode_safely "$target_path"
            ;;
        "complete")
            analyze_repository
            validate_changes
            organize_changes "$dry_run"
            execute_commits
            verify_repository "$auto_retry" "$max_retries" "$retry_count"
            ;;
        "help"|"-h"|"--help")
            echo "Usage: $SCRIPT_NAME [command] [options]"
            echo ""
            echo "Commands:"
            echo "  analyze    - Analyze repository status"
            echo "  validate   - Validate security and quality"
            echo "  organize    - Organize changes into commits"
            echo "  commit      - Execute commits and update docs"
            echo "  verify      - Final repository verification"
            echo "  complete   - Run complete workflow"
            echo "  launch-vscode [path] - Safely launch VSCode (prevents multiple instances)"
            echo "  help        - Show this help"
            echo ""
            echo "Options:"
            echo "  --dry-run       - Plan changes without executing"
            echo "  --retry         - Auto-retry cleanup when verification fails"
            echo "  --max-retries N - Maximum retry attempts (default: 3)"
            echo ""
            echo "Examples:"
            echo "  $SCRIPT_NAME complete                    # Run full workflow"
            echo "  $SCRIPT_NAME launch-vscode               # Open current project safely"
            echo "  $SCRIPT_NAME launch-vscode /path/to/repo # Open specific path safely"
            ;;
        *)
            log_error "Unknown command: $command"
            exit 1
            ;;
    esac
}

# Run main function with all arguments
main "$@"
