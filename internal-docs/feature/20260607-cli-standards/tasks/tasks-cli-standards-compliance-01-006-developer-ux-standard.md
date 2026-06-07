---
story_id: "01-006"
story_title: "Developer UX Standard Compliance"
story_name: "developer-ux-standard"
prd_name: "cli-standards-compliance"
prd_file: "internal-docs/feature/20260607-cli-standards/prd-cli-standards-compliance.md"
phase: 1
parallel_id: 6
branch: "feature/current/cli-standards-compliance/story-01-006-developer-ux-standard"
status: "todo"
assignee: ""
reviewer: ""
dependencies: []
parallel_safe: true
modules: ["root", "docs"]
priority: "MUST"
risk_level: "low"
tags: ["feat", "devex"]
due: "2026-06-21"
created_at: "2026-06-07"
updated_at: "2026-06-07"
---

## Summary

Implement Developer UX standard compliance as specified in ADR #0: support direnv for environment variable management, devbox for development environment consistency, provide justfile for common development tasks, support nix for reproducible development environments, support nx for monorepo tooling (if applicable), support containers for development and testing, and follow unified logging standards.

## Sub-Tasks

- [ ] Create .envrc file for direnv with smartfo-specific environment variables
- [ ] Add direnv configuration for development environment setup
- [ ] Create devbox.json configuration for reproducible development environment
- [ ] Add required devbox packages (Rust toolchain, etc.)
- [ ] Create justfile with common development tasks (build, test, lint, etc.)
- [ ] Add just recipe for running all tests
- [ ] Add just recipe for running linting
- [ ] Add just recipe for running clippy
- [ ] Add just recipe for building release binary
- [ ] Create nix shell configuration for reproducible builds (if applicable)
- [ ] Add container support (Dockerfile) for development and testing
- [ ] Add container-compose configuration for local development
- [ ] Implement unified logging standards across all output
- [ ] Add documentation for Developer UX setup in README
- [ ] Add documentation for using direnv with smartfo
- [ ] Add documentation for using devbox with smartfo
- [ ] Add documentation for using justfile commands

## Relevant Files

- `.envrc` — Direnv configuration for environment variables
- `devbox.json` — Devbox configuration for development environment
- `justfile` — Just command runner for common tasks
- `shell.nix` — Nix shell configuration (if applicable)
- `Dockerfile` — Container configuration for development/testing
- `docker-compose.yml` — Container orchestration for local development
- `README.md` — Documentation for Developer UX setup
- `src/logging.rs` — Ensure unified logging standards

## Acceptance Criteria

- [ ] .envrc file exists with smartfo environment variables
- [ ] devbox.json provides reproducible development environment
- [ ] justfile provides common development tasks
- [ ] Nix shell configuration works (if implemented)
- [ ] Dockerfile supports development and testing
- [ ] docker-compose.yml enables local development
- [ ] Unified logging standards are followed
- [ ] Documentation covers Developer UX setup
- [ ] Documentation covers direnv usage
- [ ] Documentation covers devbox usage
- [ ] Documentation covers justfile commands

## Test Plan

- Manual: Test direnv environment variable loading
- Manual: Test devbox environment setup
- Manual: Test justfile commands
- Manual: Test container build and run
- Lint: `cargo clippy -- -D warnings`
- Types: `cargo check`

## Observability

- Log development environment setup (info level)
- Log which Developer UX tools are being used (debug level)

## Compliance

- Follows ADR #0: Developer UX Standard

## Risks & Mitigations

- Risk: Developer UX tools may not be available on all systems — Mitigation: Provide fallback instructions for systems without specific tools
- Risk: Container configuration may become outdated — Mitigation: Document update process and version compatibility

## Dependencies

None

## Notes

- Priority should be given to tools that are most commonly used in the Rust ecosystem
- Consider the trade-off between Developer UX complexity and ease of contribution
- Ensure all Developer UX tools are optional for end-users but recommended for contributors