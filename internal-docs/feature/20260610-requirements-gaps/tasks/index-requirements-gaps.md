# Smartfo Requirements Gaps - Task Index (Test-First Approach)

## Overview

This task index addresses the compliance gaps identified in the smartfo requirements audit (78% overall compliance). The tasks are organized into sequential phases with parallel stories using a test-first approach to ensure quality from the start.

## Parallel Development Sets

### Phase 01: Test Infrastructure
| Story ID | Story Title | Status | Branch | Dependencies | Parallel-safe | Modules |
| -------- | ----------- | ------ | ------ | ------------ | ------------- | ------- |
| 01-001 | Test framework enhancements | [x] Done | feature/current/requirements-gaps/story-01-001-test-framework-enhancements | None | Parallel-safe: true | tests, framework |
| 01-002 | Cross-platform test harness setup | [x] Done | feature/current/requirements-gaps/story-01-002-cross-platform-test-harness | None | Parallel-safe: true | tests, platform |
| 01-003 | Property test extensions | [x] Done | feature/current/requirements-gaps/story-01-003-property-test-extensions | None | Parallel-safe: true | tests, property |

### Phase 02: Foundation Features + Tests
| Story ID | Story Title | Branch | Dependencies | Parallel-safe | Modules |
| -------- | ----------- | ------ | ------------ | ------------- | ------- |
| 02-001 | --init-config flag + tests | [x] Done | feature/current/requirements-gaps/story-02-001-init-config-flag-tests | 01-001 | Parallel-safe: true | install, cli, tests |
| 02-002 | Health check mechanism + tests | feature/current/requirements-gaps/story-02-002-health-check-mechanism-tests | 01-001 | Parallel-safe: true | daemon, health, tests | [x] Done |
| 02-003 | Terminal size awareness + tests | [x] Done | feature/current/requirements-gaps/story-02-003-terminal-size-awareness-tests | 01-002 | Parallel-safe: true | tui, terminal, tests |

### Phase 03: Core Features + Tests
| Story ID | Story Title | Branch | Dependencies | Parallel-safe | Modules |
| -------- | ----------- | ------ | ------------ | ------------- | ------- |
| 03-001 | TUI mode framework + tests | feature/current/requirements-gaps/story-03-001-tui-mode-framework-tests | 02-003 | Parallel-safe: true | tui, cli, tests | [~] In-Progress |
| 03-002 | Daemon process support + tests | feature/current/requirements-gaps/story-03-002-daemon-process-support-tests | 02-002 | Parallel-safe: true | daemon, cli, tests |
| 03-003 | Resource limits + tests | feature/current/requirements-gaps/story-03-003-resource-limits-tests | 02-002 | Parallel-safe: true | daemon, config, tests |

### Phase 04: Privacy + Tests
| Story ID | Story Title | Branch | Dependencies | Parallel-safe | Modules |
| -------- | ----------- | ------ | ------------ | ------------- | ------- |
| 04-001 | Privacy mode + tests | feature/current/requirements-gaps/story-04-001-privacy-mode-tests | 03-003 | Parallel-safe: true | privacy, config, tests |
| 04-002 | Audit log sanitization + tests | feature/current/requirements-gaps/story-04-002-audit-log-sanitization-tests | 04-001 | Parallel-safe: true | audit, privacy, tests |
| 04-003 | Secret handling + tests | feature/current/requirements-gaps/story-04-003-secret-handling-tests | 04-001 | Parallel-safe: true | logging, security, tests |

### Phase 05: AXI + Tests
| Story ID | Story Title | Branch | Dependencies | Parallel-safe | Modules |
| -------- | ----------- | ------ | ------------ | ------------- | ------- |
| 05-001 | Content-first behavior + tests | feature/current/requirements-gaps/story-05-001-content-first-behavior-tests | 03-001 | Parallel-safe: true | cli, output, tests |
| 05-002 | Session hooks + tests | feature/current/requirements-gaps/story-05-002-session-hooks-tests | 05-001 | Parallel-safe: true | hooks, session, tests |
| 05-003 | Contextual disclosure + tests | feature/current/requirements-gaps/story-05-003-contextual-disclosure-tests | 05-001 | Parallel-safe: true | output, suggestions, tests |

### Phase 06: Integration & CI
| Story ID | Story Title | Branch | Dependencies | Parallel-safe | Modules |
| -------- | ----------- | ------ | ------------ | ------------- | ------- |
| 06-001 | Collection/processing separation + tests | feature/current/requirements-gaps/story-06-001-collection-processing-tests | 03-002 | Parallel-safe: true | daemon, export, tests |
| 06-002 | CI skill generation integration | feature/current/requirements-gaps/story-06-002-ci-skill-generation | 05-003 | Parallel-safe: true | ci, skill |
| 06-003 | Cross-platform validation | feature/current/requirements-gaps/story-06-003-cross-platform-validation | 01-002, 04-003 | Parallel-safe: true | tests, platform |

### Phase 07: Config Reload
| Story ID | Story Title | Branch | Dependencies | Parallel-safe | Modules |
| -------- | ----------- | ------ | ------------ | ------------- | ------- |
| 07-001 | SIGHUP config reload + tests | feature/current/requirements-gaps/story-07-001-sighup-config-reload-tests | 02-001, 06-001 | Parallel-safe: false | config, signal, tests |

## Phase Summary

- **Phase 01**: Test infrastructure (3 parallel stories) - Test framework, cross-platform harness, property tests
- **Phase 02**: Foundation features (3 parallel stories) - --init-config, health checks, terminal awareness
- **Phase 03**: Core features (3 parallel stories) - TUI mode, daemon support, resource limits
- **Phase 04**: Privacy & security (3 parallel stories) - Privacy mode, audit sanitization, secret handling
- **Phase 05**: AXI enhancements (3 parallel stories) - Content-first behavior, session hooks, contextual disclosure
- **Phase 06**: Integration & CI (3 parallel stories) - Job export, CI integration, cross-platform validation
- **Phase 07**: Config reload (1 story) - SIGHUP config reload

Total: 19 stories across 7 sequential phases
