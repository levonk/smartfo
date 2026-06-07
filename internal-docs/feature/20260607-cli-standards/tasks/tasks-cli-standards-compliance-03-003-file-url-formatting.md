---
story_id: "03-003"
story_title: "File/URL Reference Formatting"
story_name: "file-url-formatting"
prd_name: "cli-standards-compliance"
prd_file: "internal-docs/feature/20260607-cli-standards/prd-cli-standards-compliance.md"
phase: 3
parallel_id: 3
branch: "feature/current/cli-standards-compliance/story-03-003-file-url-formatting"
status: "todo"
assignee: ""
reviewer: ""
dependencies: []
parallel_safe: true
modules: ["All modules (output formatting)"]
priority: "SHOULD"
risk_level: "low"
tags: ["feat", "formatting"]
due: "2026-07-19"
created_at: "2026-06-07"
updated_at: "2026-06-07"
---

## Summary

Implement file reference formatting as specified in ADR #15 and URL formatting as specified in ADR #16. Ensure all file references with line numbers use VSCode-compatible format, support both `file:///absolute/path/to/file:line:column` and standard `file:line:column`, ensure modern terminals can auto-linkify these references, ensure all URLs are in standard HTTP/HTTPS format with proper encoding, support copying URLs to browser, and ensure smart terminal linking works for URLs.

## Sub-Tasks

- [ ] Define VSCode-compatible file reference format: `file:///absolute/path/to/file:line:column`
- [ ] Define standard file reference format: `file:line:column`
- [ ] Audit all file references across all modules
- [ ] Update file references in error messages to use VSCode-compatible format
- [ ] Update file references in help output to use VSCode-compatible format
- [ ] Update file references in debug output to use VSCode-compatible format
- [ ] Ensure both VSCode-compatible and standard formats are supported
- [ ] Test that modern terminals can auto-linkify file references
- [ ] Audit all URL references across all modules
- [ ] Ensure all URLs use standard HTTP/HTTPS format
- [ ] Ensure all URLs have proper encoding
- [ ] Add support for copying URLs to clipboard/browser
- [ ] Ensure smart terminal linking works for URLs
- [ ] Add unit tests for file reference formatting
- [ ] Add unit tests for URL formatting
- [ ] Add unit tests for terminal linkification
- [ ] Add integration tests for file and URL references

## Relevant Files

- `src/main.rs` — Update file and URL references
- `src/cli.rs` — Update file and URL references in help output
- `src/config.rs` — Update file references in config errors
- `src/vcs.rs` — Update file and URL references
- `src/logging.rs` — Update file and URL references in log output
- `tests/formatting_tests.rs` — Add tests for file/URL formatting

## Acceptance Criteria

- [ ] All file references with line numbers use VSCode-compatible format
- [ ] Both `file:///absolute/path/to/file:line:column` and `file:line:column` formats are supported
- [ ] Modern terminals can auto-linkify file references
- [ ] All URLs are in standard HTTP/HTTPS format
- [ ] All URLs have proper encoding
- [ ] URLs can be copied to clipboard/browser
- [ ] Smart terminal linking works for URLs
- [ ] All tests pass

## Test Plan

- Unit: `cargo test formatting_tests::vscode_file_references`
- Unit: `cargo test formatting_tests::standard_file_references`
- Unit: `cargo test formatting_tests::url_formatting`
- Unit: `cargo test formatting_tests::url_encoding`
- Unit: `cargo test formatting_tests::terminal_linkification`
- Integration: `cargo test formatting_tests::help_output_references`
- Integration: `cargo test formatting_tests::error_output_references`
- Lint: `cargo clippy -- -D warnings`
- Types: `cargo check`

## Observability

- Log file reference format usage (debug level)
- Log URL format usage (debug level)

## Compliance

- Follows ADR #15: File Reference Formatting
- Follows ADR #16: URL Formatting

## Risks & Mitigations

- Risk: Terminal linkification may not work on all terminals — Mitigation: Provide fallback to standard format
- Risk: URL encoding may cause issues with special characters — Mitigation: Test with various URL formats

## Dependencies

None

## Notes

- Use terminal link codes (OSC 8) for modern terminal linking
- Consider adding --no-links flag to disable terminal linking
- Ensure file references work with relative paths when appropriate