---
story_id: "03-005"
story_title: "Man Pages Generation"
story_name: "man-pages"
prd_name: "cli-standards-compliance"
prd_file: "internal-docs/feature/20260607-cli-standards/prd-cli-standards-compliance.md"
phase: 3
parallel_id: 5
branch: "feature/current/cli-standards-compliance/story-03-005-man-pages"
status: "todo"
assignee: ""
reviewer: ""
dependencies: ["01-001"]
parallel_safe: true
modules: ["man.rs (new)", "docs/"]
priority: "MUST"
risk_level: "low"
tags: ["feat", "documentation"]
due: "2026-07-19"
created_at: "2026-06-07"
updated_at: "2026-06-07"
---

## Summary

Implement man pages generation as specified in ADR #18. Generate traditional Unix man pages for documentation. Provide man pages for smartfo, mv, and rm modes. Make accessible via `man smartfo`, `man smartfo-mv`, `man smartfo-rm`. Add `--man` flag to display man page content. Install man pages to system man directory via `--install`. Use a man page generation library or manual roff source.

## Sub-Tasks

- [ ] Create new src/man.rs module
- [ ] Create man page source files in docs/ directory
- [ ] Write man page for smartfo (main binary)
- [ ] Write man page for mv mode (smartfo-mv)
- [ ] Write man page for rm mode (smartfo-rm)
- [ ] Include all flags and options in man pages
- [ ] Include examples in man pages
- [ ] Include configuration file documentation in man pages
- [ ] Include environment variable documentation in man pages
- [ ] Include exit codes in man pages
- [ ] Implement --man flag to display man page content
- [ ] Implement man page installation logic in --install flag
- [ ] Add man page uninstall logic in --uninstall flag
- [ ] Ensure man pages are installed to correct system directory (e.g., /usr/local/share/man/man1/)
- [ ] Add unit tests for man page generation
- [ ] Add integration tests for man page installation
- [ ] Verify man pages are accessible via man command

## Relevant Files

- `src/man.rs` (new) — Implement man page generation and installation logic
- `src/cli.rs` — Add --man flag
- `src/main.rs` — Handle --man flag display
- `src/install.rs` — Integrate man page installation
- `docs/smartfo.1` — Man page source for smartfo
- `docs/smartfo-mv.1` — Man page source for mv mode
- `docs/smartfo-rm.1` — Man page source for rm mode
- `tests/man_tests.rs` — Add tests for man pages

## Acceptance Criteria

- [ ] Man pages are generated for smartfo, mv, and rm modes
- [ ] Man pages are accessible via `man smartfo`, `man smartfo-mv`, `man smartfo-rm`
- [ ] --man flag displays man page content
- [ ] Man pages are installed via --install flag
- [ ] Man pages are uninstalled via --uninstall flag
- [ ] Man pages include all flags and options
- [ ] Man pages include examples
- [ ] Man pages include configuration documentation
- [ ] Man pages include environment variable documentation
- [ ] Man pages include exit codes
- [ ] All tests pass

## Test Plan

- Unit: `cargo test man_tests::man_page_generation`
- Unit: `cargo test man_tests::man_page_content`
- Integration: `cargo test man_tests::install_man_pages`
- Integration: `cargo test man_tests::uninstall_man_pages`
- Manual: Verify `man smartfo` works after installation
- Manual: Verify `man smartfo-mv` works after installation
- Manual: Verify `man smartfo-rm` works after installation
- Lint: `cargo clippy -- -D warnings`
- Types: `cargo check`

## Observability

- Log man page installation events (info level)
- Log man page uninstallation events (info level)

## Compliance

- Follows ADR #18: Man Pages

## Risks & Mitigations

- Risk: Man pages may become outdated as features change — Mitigation: Document process for updating man pages
- Risk: Man page installation may require sudo privileges — Mitigation: Clear error message if installation fails due to permissions

## Dependencies

- 01-001 (Standard Arguments Implementation) — ensures command structure is stable

## Notes

- Use roff format for man page sources
- Consider using pandoc or help2man for generation
- Ensure man pages follow Unix man page conventions (NAME, SYNOPSIS, DESCRIPTION, OPTIONS, EXAMPLES, etc.)
- Test man page rendering with different man implementations