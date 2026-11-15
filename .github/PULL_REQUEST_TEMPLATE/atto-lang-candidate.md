# AttoLang candidate change

## Changed source and authority
- Owning repository:
- Immutable head commit SHA:
- Changed `.atto` source paths and blob SHAs:
- Owning JSON Schema / TypeSpec peer authorities:
- Scope: candidate / production (default: **candidate**)

## Contract and runtime evidence
- [ ] The exact `.atto` source was parsed and typechecked by the pinned Atto Rust/Wasm or native compiler.
- [ ] The compiler produced an **executable** artifact and that artifact was run (parse-only is insufficient).
- [ ] Receipt records source SHA, compiler SHA, target, command, exit code, tests, and artifact digest.
- [ ] Unsupported profiles, skipped jobs, stale receipts, or `E_UNSUPPORTED` count as **not admitted**.
- [ ] Boundary cases and negative mutants cover schema drift, forbidden credentials, and private/public surface leaks.
- [ ] The independently authored TypeSpec and JSON Schema authorities have passing compatibility evidence.
- [ ] The SDK is not added to publishing matrices or shipped before all gates pass.

## CI and security
- [ ] All required checks passed on this **exact** head SHA; approvals are current.
- [ ] If Actions minutes were unavailable, a temporary private test-carrier mirror was built from the identical source Git tree; every source/mirror blob SHA was checked.
- [ ] No source access token, private key, plaintext environment data, provider credentials or secrets were copied into the carrier.
- [ ] Existing review, branch protection, and domain-specific security checks remain in effect.

## Rollout
- Candidate/production status and blocked capabilities:
- Compiler and cross-backend discrepancies:
- Rollback / follow-up PRs:

> This checklist is a review aid, **not** a substitute for compiler evidence,
> passing required checks, an approved review, or the owning repository's policy.
