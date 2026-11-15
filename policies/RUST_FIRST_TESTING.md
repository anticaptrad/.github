# Rust-first testing and scripting policy

**Owner:** anticaptrad organization maintainers  
**Scope:** new and materially rewritten first-party scripts, tests, CI helpers, builds, releases, codegen, migration utilities, and developer tooling  
**Status:** proposed in PR #20. Sibling repositories must adopt or synchronize it explicitly.

## Mandatory standard

- **Write new test tools and automation in Rust, not Python.** Prefer Rust unit/integration tests, cargo test --locked, cargo xtask or rustc --test for a small independent harness.
- Do not introduce Python .py scripts, inline python -c or heredoc scripts, pytest, unittest or Python CI wrappers for new first-party testing and maintenance automation.
- Keep YAML declarative and shell wrappers minimal; use Rust for substantive validation and business logic, and pin external Actions to exact SHAs.
- Prefer Rust standard library unless structured parsing or another capability justifies a version-pinned Cargo dependency.
- Write snake_case helper and test names. Cover successful cases, edge cases, negative cases and malicious inputs when applicable.
- Preserve exact-head check and build evidence. A source-string scan does not prove AttoLang compiler execution.

## Existing Python migrations: do not delete blindly

1. Inventory existing Python scripts and all callers, including GitHub Actions.
2. Record arguments, input/output formats, exit codes, platform assumptions, failure behavior and security invariants.
3. Create an equivalent Rust implementation and Rust tests for positive and negative fixtures.
4. Run both implementations over the same fixtures and compare outputs and matching failures, documenting intentional differences.
5. Only after an exact-head successful CI run, migrate workflows and consumers and retire Python dependencies in a reviewed change.
6. Real Python product application code may be excepted with an explicit owner, justification and review, but ordinary scripts may not.

## AttoLang candidate admission

- Use Rust to validate independent TypeSpec/JSON Schema and .atto candidate manifests, but never count a source string check as executable compiler evidence.
- Require a pinned compiler SHA, source blob SHA, typecheck, emitted artifact, actual execution and negative conformance evidence before SDK promotion.
- Keep unverified .atto files out of production client packages.

## Actions budget fallback

When necessary, mirror a verified exact source Git tree into an approved private CI carrier that has available GitHub Actions minutes, verify every blob SHA, and record source and mirror refs and the run URL. Do not transfer secrets or assume cross-org private checkouts work with GITHUB_TOKEN.

This organization .github repository is not automatically inherited as a workflow policy by sibling repositories. Adopt the rules and tests explicitly.
