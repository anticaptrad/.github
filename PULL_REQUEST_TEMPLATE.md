## Change summary

Describe the user-visible behavior, affected repositories or components, compatibility impact, failure behavior, and reversible rollout path. Mark non-applicable checks as `N/A` with a reason.

## Review gates

### Review path and dependencies

- [ ] All commits are on a non-default branch and this pull request is the only proposed path into the protected default branch.
- [ ] No generated tool, bot, migration runner, or deployment process writes directly to a protected default branch.
- [ ] The change is reviewable as one unit, or its staged rollout and follow-up pull requests are identified.
- [ ] Cross-repository dependencies are pinned by immutable commit, lockfile, or released Zed package.
- [ ] Public contracts are generated from canonical interface or schema sources and consumer compatibility was checked.
- [ ] Breaking changes include migration, rollback, backfill, and staged-rollout evidence.

### Scope and ownership boundaries

- [ ] The change is focused and does not silently cross repository ownership boundaries.
- [ ] No `*-infra` repository is introduced as a Git submodule under `*-monorepo/apps`.
- [ ] Shared functionality is imported from its owning repository rather than copied into a local implementation.
- [ ] Public contracts, compatibility, telemetry, rollback, and failure behavior are documented.

### SQL, persistence, and state

- [ ] No SQL changes are present, or every declaration has a registered `<organization>.<domain>` namespace, stable object prefix where required, and explicit owning repository.
- [ ] Domain SQL may remain with its owning organization, but identity, ordering, checksums, drift detection, and promotion are registered through `declarative-migrations`.
- [ ] JSON Schema, generated interfaces, ORM models, fixtures, and migration declarations were updated and checked deterministically together.
- [ ] Application startup validates schema compatibility and does not apply production DDL.
- [ ] Destructive changes, tenant isolation, RLS/authorization, idempotency, state-machine invariants, backfill, and rollback have evidence.

### Infrastructure, security, and end-to-end coverage

- [ ] Application manifests remain app-owned; cluster composition is delegated to `oresoftware/k8s-cluster` and shared components to `oresoftware/k8s-libs-and-shared-defs`.
- [ ] Workload identity, restricted Pod Security, default-deny networking, explicit egress, probes, non-root execution, bounded resources, secret handling, and immutable image/dependency references were considered.
- [ ] Authentication and authorization failures are fail-closed, sensitive operations are auditable, and tenant boundaries are preserved.
- [ ] Destructive and cross-runtime tests run in the corresponding `*-test` organization or an isolated E2E environment, with teardown evidence.
- [ ] Zed lifecycle hooks cover deterministic format, lint, build, contract, test, and publish checks without bypassing language-native validation.

### Verification, observability, and safety

- [ ] Unit, integration, adversarial, migration, contract, and end-to-end tests cover the changed behavior.
- [ ] ORES OTEL trace and correlation propagation is present where applicable, with secrets and user-content capture disabled by default.
- [ ] Secrets, credentials, personal data, private repository inventory, sensitive telemetry, and user content are excluded from source, logs, fixtures, and build artifacts.
- [ ] Conflicts were resolved semantically using both sides and relevant history; no destructive Git recovery, force push, or history rewrite was used.
- [ ] Test evidence, residual risks, follow-up work, and intentionally deferred repositories are listed below.

## Validation evidence and residual risk

Provide exact commands, checks, fixtures, test-organization run links, migration and drift results, teardown evidence, manual verification, known limitations, and follow-up owners.
