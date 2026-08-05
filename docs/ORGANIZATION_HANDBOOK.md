# anticaptrad organization handbook

> Shared operating defaults for repositories maintained under **anticaptrad**. Repository-local policy may strengthen these rules but should not silently weaken them.

## Mission

anticaptrad maintains the software, services, libraries, and documentation published under the anticaptrad project. This `.github` repository is the canonical home for shared policy, reusable templates, community health files, and planning links. Product-specific claims and requirements belong in the repository that owns them and should be backed by current implementation or approved planning records.

## Repository contract

Each active repository must document purpose, ownership boundary, maturity, supported environments, development and test commands, authoritative interfaces and data, release and rollback procedures, compatibility policy, and GitHub Project/Linear links. Assumptions, external integrations, generated artifacts, privacy boundaries, and operational limitations must be explicit.

## Change workflow

1. Anchor work in an issue, Linear item, or documented maintenance objective.
2. Keep branches and pull requests focused.
3. Explain motivation, scope, risk, validation, compatibility, migration, and rollback.
4. Test success, invalid input, permission, timeout, retry, partial failure, and recovery paths as relevant.
5. Resolve conflicts semantically by reconstructing both sides' intent.
6. Prefer squash merges for focused work unless commit structure materially improves auditability.

## Evidence, security, and documentation

Pull requests should include reproducible commands, approved fixtures, expected and observed results, negative-path coverage, documentation updates, and CI or local-equivalent evidence. Never commit credentials, production data, private keys, or sensitive logs. Follow `SECURITY.md` for private reporting. Keep examples executable, links current, assumptions explicit, and consequential architectural, compatibility, privacy, and operational decisions recorded.

## Planning ownership

GitHub owns code, reviews, checks, releases, and delivery evidence. Linear owns priority, dependencies, sequencing, and cross-project planning. The organization GitHub Project is the cross-repository execution view; see `PROJECTS.md` for routing details.

## Organization health

- [ ] Profiles, descriptions, topics, and READMEs accurately describe current scope.
- [ ] Community health files and reusable issue/PR guidance are present.
- [ ] Repository boundaries, interfaces, integrations, and operational limits are explicit.
- [ ] Required checks reflect current correctness, security, privacy, and supply-chain risk.
- [ ] Stale repositories are archived or clearly marked.
- [ ] GitHub Project and Linear links resolve and reflect completed work.
