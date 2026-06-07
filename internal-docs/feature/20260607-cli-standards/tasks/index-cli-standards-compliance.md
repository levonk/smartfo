# Task Index: CLI Standards Compliance

| Story ID | Story Title | Branch | Dependencies | Parallel-safe | Modules |
| -------- | ----------- | ------ | ------------ | ------------- | ------- |
| 01-001 | Standard Arguments Implementation | feature/current/cli-standards-compliance/story-01-001-standard-arguments | None | Parallel-safe: true | cli.rs, main.rs |
| 01-002 | Config Initialization & System Config | feature/current/cli-standards-compliance/story-01-002-config-initialization | None | Parallel-safe: true | config.rs |
| 01-003 | Install/Uninstall Enhancement | feature/current/cli-standards-compliance/story-01-003-install-uninstall | None | Parallel-safe: true | main.rs, install.rs (new) |
| 01-004 | Input & Globbing Support | feature/current/cli-standards-compliance/story-01-004-input-globbing | None | Parallel-safe: true | cli.rs |
| 01-005 | Output Discipline & JSON Mode | feature/current/cli-standards-compliance/story-01-005-output-discipline | None | Parallel-safe: true | logging.rs, cli.rs |
| 01-006 | Developer UX Standard Compliance | feature/current/cli-standards-compliance/story-01-006-developer-ux-standard | None | Parallel-safe: true | root, docs |
| 02-001 | Logging Modes Implementation | feature/current/cli-standards-compliance/story-02-001-logging-modes | 01-005 | Parallel-safe: true | logging.rs |
| 02-002 | Signals & Exit Codes | feature/current/cli-standards-compliance/story-02-002-signals-exit-codes | None | Parallel-safe: true | main.rs |
| 02-003 | Dry-Run Mode | feature/current/cli-standards-compliance/story-02-003-dry-run-mode | None | Parallel-safe: true | mv.rs, rm.rs |
| 02-004 | Confirmation Prompts | feature/current/cli-standards-compliance/story-02-004-confirmation-prompts | None | Parallel-safe: true | mv.rs, rm.rs |
| 02-005 | Progress Indicators | feature/current/cli-standards-compliance/story-02-005-progress-indicators | 02-001 | Parallel-safe: true | worker.rs, indicatif integration |
| 03-001 | Daemon Enhancements | feature/current/cli-standards-compliance/story-03-001-daemon-enhancements | 02-002 | Parallel-safe: true | daemon.rs, queue.rs |
| 03-002 | Error Message Formatting | feature/current/cli-standards-compliance/story-03-002-error-formatting | None | Parallel-safe: true | All modules (error handling) |
| 03-003 | File/URL Reference Formatting | feature/current/cli-standards-compliance/story-03-003-file-url-formatting | None | Parallel-safe: true | All modules (output formatting) |
| 03-004 | Shell Completion Generation | feature/current/cli-standards-compliance/story-03-004-shell-completion | 01-001 | Parallel-safe: true | completions.rs (new) |
| 03-005 | Man Pages Generation | feature/current/cli-standards-compliance/story-03-005-man-pages | 01-001 | Parallel-safe: true | man.rs (new), docs/ |
| 04-001 | Pager Integration | feature/current/cli-standards-compliance/story-04-001-pager-integration | None | Parallel-safe: true | cli.rs, output.rs |
| 04-002 | Subcommand Organization | feature/current/cli-standards-compliance/story-04-002-subcommand-organization | 01-001 | Parallel-safe: true | cli.rs |
| 04-003 | Configuration Validation | feature/current/cli-standards-compliance/story-04-003-config-validation | 01-002 | Parallel-safe: true | config.rs |
| 04-004 | Terminal Size Awareness | feature/current/cli-standards-compliance/story-04-004-terminal-size-awareness | None | Parallel-safe: true | output.rs |
| 04-005 | Environment Variable Naming | feature/current/cli-standards-compliance/story-04-005-env-var-naming | 01-002 | Parallel-safe: true | config.rs |
| 05-001 | Cross-Platform Path Handling | feature/current/cli-standards-compliance/story-05-001-cross-platform-paths | None | Parallel-safe: true | All modules (path operations) |
| 05-002 | Credential/Secret Handling | feature/current/cli-standards-compliance/story-05-002-credential-handling | None | Parallel-safe: true | config.rs, logging.rs |
| 05-003 | Resource Limits Implementation | feature/current/cli-standards-compliance/story-05-003-resource-limits | 02-002 | Parallel-safe: true | daemon.rs, worker.rs |
| 05-004 | Testing Infrastructure | feature/current/cli-standards-compliance/story-05-004-testing-infrastructure | None | Parallel-safe: false | tests/ (all test modules) |
| 06-001 | Collection/Processing Separation | feature/current/cli-standards-compliance/story-06-001-collection-processing-separation | 03-001 | Parallel-safe: true | daemon.rs, export.rs (new) |
| 06-002 | Config File Versioning | feature/current/cli-standards-compliance/story-06-002-config-versioning | 04-003 | Parallel-safe: true | config.rs |
| 06-003 | Structured Logging Auto-Detection | feature/current/cli-standards-compliance/story-06-003-logging-auto-detection | 02-001 | Parallel-safe: true | logging.rs |
| 06-004 | Signal-Based Config Reload | feature/current/cli-standards-compliance/story-06-004-sighup-config-reload | 02-002, 04-003 | Parallel-safe: true | main.rs, daemon.rs, config.rs |
| 07-001 | Health Check Implementation | feature/current/cli-standards-compliance/story-07-001-health-check | 03-001 | Parallel-safe: true | health.rs (new), daemon.rs |
| 07-002 | Privacy Mode Implementation | feature/current/cli-standards-compliance/story-07-002-privacy-mode | 06-002 | Parallel-safe: true | config.rs, audit.rs |
| 07-003 | Audit Logging Enhancements | feature/current/cli-standards-compliance/story-07-003-audit-logging-enhancements | 07-002 | Parallel-safe: true | audit.rs |
| 07-004 | TUI Mode Implementation | feature/current/cli-standards-compliance/story-07-004-tui-mode | 01-001, 04-004 | Parallel-safe: true | tui.rs (new), ratatui integration |
| 08-001 | Integration Testing | feature/current/cli-standards-compliance/story-08-001-integration-testing | 05-004, 07-004 | Parallel-safe: false | tests/integration/ |
| 08-002 | Documentation Completion | feature/current/cli-standards-compliance/story-08-002-documentation-completion | 03-005, 07-004 | Parallel-safe: true | docs/, README.md |
| 08-003 | Cross-Platform Testing | feature/current/cli-standards-compliance/story-08-003-cross-platform-testing | 05-001, 08-001 | Parallel-safe: true | tests/ |
| 08-004 | Release Preparation | feature/current/cli-standards-compliance/story-08-004-release-preparation | 08-001, 08-002, 08-003 | Parallel-safe: false | Cargo.toml, CHANGELOG.md |