<!--
=============================================================================
SYNC IMPACT REPORT
=============================================================================
Version change: 0.0.0 → 1.0.0
Bump rationale: Initial constitution adoption (MAJOR - new governance)

Modified principles: N/A (initial version)

Added sections:
- Core Principles (5 principles)
  - I. Explicit Over Implicit
  - II. Readable Code First
  - III. CLI Interface Standards
  - IV. Test Coverage
  - V. Simplicity
- Documentation Standards
- Development Workflow
- Governance

Removed sections: N/A (initial version)

Templates requiring updates:
- .specify/templates/plan-template.md: ✅ Compatible (no changes needed)
- .specify/templates/spec-template.md: ✅ Compatible (no changes needed)
- .specify/templates/tasks-template.md: ✅ Compatible (no changes needed)

Follow-up TODOs: None
=============================================================================
-->

# checkPortCli Constitution

## Core Principles

### I. Explicit Over Implicit

All code, configuration, and documentation MUST prioritize explicit declarations over implicit behavior.

**Requirements**:
- Variable and function names MUST clearly describe their purpose without requiring context
- Magic numbers and strings MUST be replaced with named constants
- Default values MUST be documented where they are defined
- Side effects MUST be explicitly documented in function signatures or comments
- Configuration options MUST have explicit defaults with documented behavior

**Rationale**: Explicit code reduces cognitive load and eliminates guesswork during
maintenance and debugging. When behavior is visible in the code, developers spend
less time tracing execution paths.

### II. Readable Code First

Code MUST be written for human comprehension as the primary goal, with machine
execution as secondary.

**Requirements**:
- Function and variable names MUST use complete words (no abbreviations except
  widely-accepted ones like `id`, `url`, `cli`)
- Complex logic MUST be broken into named helper functions that describe intent
- Nested conditionals beyond 2 levels MUST be refactored for clarity
- Each function MUST have a single, clear responsibility
- Comments MUST explain "why", not "what" (code explains "what")

**Rationale**: Code is read far more often than it is written. Investing in
readability reduces long-term maintenance costs and onboarding time.

### III. CLI Interface Standards

All user-facing CLI interactions MUST follow consistent, predictable patterns.

**Requirements**:
- All commands MUST provide `--help` output describing usage and options
- Error messages MUST include actionable guidance (what went wrong, how to fix)
- Exit codes MUST follow POSIX conventions (0 = success, non-zero = error)
- Output MUST be parseable: human-readable by default, JSON via `--json` flag
- Long-running operations MUST provide progress indication

**Rationale**: Consistent CLI behavior reduces user friction and enables
integration with other tools and scripts.

### IV. Test Coverage

All features MUST include tests that verify correct behavior and prevent regressions.

**Requirements**:
- New features MUST include unit tests covering primary use cases
- Edge cases and error conditions MUST have explicit test coverage
- Test names MUST describe the scenario being tested in plain language
- Tests MUST be independent and not rely on execution order
- Integration tests MUST cover CLI command invocation and output parsing

**Rationale**: Tests serve as executable documentation and provide confidence
during refactoring. Well-named tests describe expected behavior.

### V. Simplicity

Solutions MUST use the simplest approach that meets requirements.

**Requirements**:
- YAGNI (You Aren't Gonna Need It): Features MUST NOT be added speculatively
- Abstractions MUST be justified by concrete, current use cases
- Dependencies MUST be evaluated for necessity before adoption
- Configuration options MUST be limited to genuinely useful variations
- Code MUST NOT be optimized without measured performance data

**Rationale**: Complexity grows organically if not actively resisted. Every
abstraction and feature has maintenance cost that must be justified.

## Documentation Standards

All documentation MUST enable independent understanding without requiring
external context or tribal knowledge.

**Requirements**:
- README MUST include: project purpose, installation, quick start, and examples
- API documentation MUST include parameter types, return values, and error cases
- Code comments MUST explain non-obvious design decisions
- Change history MUST be maintained via meaningful commit messages
- Configuration files MUST include inline comments explaining each option

## Development Workflow

Development activities MUST follow consistent, documented processes.

**Requirements**:
- All changes MUST be made on feature branches, not directly on main
- Commits MUST have descriptive messages explaining the change purpose
- Pull requests MUST include description of changes and testing performed
- Code review MUST verify adherence to constitution principles
- Breaking changes MUST be clearly marked and documented

## Governance

This constitution establishes the authoritative standards for the checkPortCli project.

**Amendment Process**:
1. Proposed changes MUST be documented with rationale
2. Changes MUST be reviewed for impact on existing code and workflows
3. Version MUST be incremented according to semantic versioning:
   - MAJOR: Principle removal or incompatible redefinition
   - MINOR: New principle or significant expansion
   - PATCH: Clarifications and non-semantic refinements

**Compliance**:
- All code reviews MUST verify adherence to these principles
- Violations MUST be justified in writing with documented rationale
- The constitution supersedes informal conventions and preferences

**Version**: 1.0.0 | **Ratified**: 2026-01-02 | **Last Amended**: 2026-01-02
