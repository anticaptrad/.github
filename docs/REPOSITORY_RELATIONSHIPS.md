<!-- ore-org-baseline:begin -->
# Repository relationships for `anticaptrad`

This file is rendered from `repository-relationships.json`. The JSON registry is authoritative.

- Audience: `public`
- Repositories represented: **11**
- Relationships represented: **17**
- Inventory digest: `sha256:936b4bac6e8bd731ca6e89845f888a31739c35b1ca965ef7ba10f840cfc472c9`

## Immutable routing identity

| Field | Value |
|---|---|
| Mapping ID | `context:anticaptrad` |
| GitHub owner ID | `308909485` |
| Linear project ID | `17fc7fee-7a7b-41ba-8e6e-919d7866406e` |
| Linear team ID | `eb8ab169-5afe-4b6f-9cab-3f2aa3e887dc` |

## Repositories

| Repository | Visibility | Roles | Archived |
|---|---|---|---|
| `anticaptrad/.github` | `public` | `community-health`, `governance`, `relationship-registry` | no |
| `anticaptrad/act-ai-server.ts` | `public` | `repository` | no |
| `anticaptrad/act-api-server.rs` | `public` | `api-server` | no |
| `anticaptrad/act-clients` | `public` | `clients` | no |
| `anticaptrad/act-e2e` | `public` | `end-to-end-tests` | no |
| `anticaptrad/act-infra` | `public` | `infrastructure` | no |
| `anticaptrad/act-interfaces` | `public` | `interfaces` | no |
| `anticaptrad/act-mcp-server.rs` | `public` | `mcp-server` | no |
| `anticaptrad/act-monorepo` | `public` | `monorepo` | no |
| `anticaptrad/act-sync` | `public` | `sync` | no |
| `anticaptrad/act-web-server.rs` | `public` | `web-server` | no |

## Relationships

| From | Type | To | Status | Required |
|---|---|---|---|---|
| `anticaptrad/.github` | `governs` | `anticaptrad/act-ai-server.ts` | `declared` | yes |
| `anticaptrad/.github` | `governs` | `anticaptrad/act-api-server.rs` | `declared` | yes |
| `anticaptrad/.github` | `governs` | `anticaptrad/act-clients` | `declared` | yes |
| `anticaptrad/.github` | `governs` | `anticaptrad/act-e2e` | `declared` | yes |
| `anticaptrad/.github` | `governs` | `anticaptrad/act-infra` | `declared` | yes |
| `anticaptrad/.github` | `governs` | `anticaptrad/act-interfaces` | `declared` | yes |
| `anticaptrad/.github` | `governs` | `anticaptrad/act-mcp-server.rs` | `declared` | yes |
| `anticaptrad/.github` | `governs` | `anticaptrad/act-monorepo` | `declared` | yes |
| `anticaptrad/.github` | `governs` | `anticaptrad/act-sync` | `declared` | yes |
| `anticaptrad/.github` | `governs` | `anticaptrad/act-web-server.rs` | `declared` | yes |
| `anticaptrad/act-api-server.rs` | `depends_on` | `anticaptrad/act-interfaces` | `inferred` | no |
| `anticaptrad/act-clients` | `depends_on` | `anticaptrad/act-interfaces` | `inferred` | no |
| `anticaptrad/act-e2e` | `tests` | `anticaptrad/act-monorepo` | `inferred` | no |
| `anticaptrad/act-infra` | `deploys` | `anticaptrad/act-monorepo` | `inferred` | no |
| `anticaptrad/act-mcp-server.rs` | `depends_on` | `anticaptrad/act-interfaces` | `inferred` | no |
| `anticaptrad/act-sync` | `depends_on` | `anticaptrad/act-interfaces` | `inferred` | no |
| `anticaptrad/act-web-server.rs` | `depends_on` | `anticaptrad/act-interfaces` | `inferred` | no |

## Editing relationships

Put reviewed public declarations in `repository-relationships.manual.json`; do not edit the generated registry directly.
Private repository names and private-only relationships belong in the private `approved-private-registry` mirror.
Inferred edges are advisory and must remain visibly labeled until reviewed.
<!-- ore-org-baseline:end -->
