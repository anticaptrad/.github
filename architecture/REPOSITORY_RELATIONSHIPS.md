# `anticaptrad` repository relationships

Generated from reviewed policy and the current **public** repository inventory.

- Public repositories declared: **11**
- Private repository names withheld: **0**
- Relationship edges: **41**

## Repository roles

| Repository | Role | Lifecycle |
|---|---|---|
| [`.github`](https://github.com/anticaptrad/.github) | `organization_governance` | `active` |
| [`act-interfaces`](https://github.com/anticaptrad/act-interfaces) | `interfaces` | `active` |
| [`act-clients`](https://github.com/anticaptrad/act-clients) | `client_sdk` | `active` |
| [`act-api-server.rs`](https://github.com/anticaptrad/act-api-server.rs) | `api_service` | `active` |
| [`act-ai-server.ts`](https://github.com/anticaptrad/act-ai-server.ts) | `domain_service` | `active` |
| [`act-sync`](https://github.com/anticaptrad/act-sync) | `sync_service` | `active` |
| [`act-mcp-server.rs`](https://github.com/anticaptrad/act-mcp-server.rs) | `mcp_server` | `active` |
| [`act-web-server.rs`](https://github.com/anticaptrad/act-web-server.rs) | `web_bff` | `active` |
| [`act-infra`](https://github.com/anticaptrad/act-infra) | `infrastructure` | `active` |
| [`act-e2e`](https://github.com/anticaptrad/act-e2e) | `end_to_end_tests` | `active` |
| [`act-monorepo`](https://github.com/anticaptrad/act-monorepo) | `composition_workspace` | `active` |

## Declared edges

| From | Relationship | To | Status/basis |
|---|---|---|---|
| `anticaptrad/.github` | `governs` | `anticaptrad/act-ai-server.ts` | `inferred` / `role-convention`: organization defaults, safety, and relationship declarations |
| `anticaptrad/.github` | `governs` | `anticaptrad/act-api-server.rs` | `inferred` / `role-convention`: organization defaults, safety, and relationship declarations |
| `anticaptrad/.github` | `governs` | `anticaptrad/act-clients` | `inferred` / `role-convention`: organization defaults, safety, and relationship declarations |
| `anticaptrad/.github` | `governs` | `anticaptrad/act-e2e` | `inferred` / `role-convention`: organization defaults, safety, and relationship declarations |
| `anticaptrad/.github` | `governs` | `anticaptrad/act-infra` | `inferred` / `role-convention`: organization defaults, safety, and relationship declarations |
| `anticaptrad/.github` | `governs` | `anticaptrad/act-interfaces` | `inferred` / `role-convention`: organization defaults, safety, and relationship declarations |
| `anticaptrad/.github` | `governs` | `anticaptrad/act-mcp-server.rs` | `inferred` / `role-convention`: organization defaults, safety, and relationship declarations |
| `anticaptrad/.github` | `governs` | `anticaptrad/act-monorepo` | `inferred` / `role-convention`: organization defaults, safety, and relationship declarations |
| `anticaptrad/.github` | `governs` | `anticaptrad/act-sync` | `inferred` / `role-convention`: organization defaults, safety, and relationship declarations |
| `anticaptrad/.github` | `governs` | `anticaptrad/act-web-server.rs` | `inferred` / `role-convention`: organization defaults, safety, and relationship declarations |
| `anticaptrad/act-ai-server.ts` | `implements_contracts_from` | `anticaptrad/act-interfaces` | `inferred` / `role-convention`: service boundary implements canonical contracts |
| `anticaptrad/act-api-server.rs` | `implements_contracts_from` | `anticaptrad/act-interfaces` | `inferred` / `role-convention`: service boundary implements canonical contracts |
| `anticaptrad/act-clients` | `generated_from` | `anticaptrad/act-interfaces` | `inferred` / `role-convention`: SDK bindings derive from canonical contracts |
| `anticaptrad/act-e2e` | `tests` | `anticaptrad/act-ai-server.ts` | `inferred` / `role-convention`: black-box compatibility verification |
| `anticaptrad/act-e2e` | `tests` | `anticaptrad/act-api-server.rs` | `inferred` / `role-convention`: black-box compatibility verification |
| `anticaptrad/act-e2e` | `tests` | `anticaptrad/act-mcp-server.rs` | `inferred` / `role-convention`: black-box compatibility verification |
| `anticaptrad/act-e2e` | `tests` | `anticaptrad/act-sync` | `inferred` / `role-convention`: black-box compatibility verification |
| `anticaptrad/act-e2e` | `tests` | `anticaptrad/act-web-server.rs` | `inferred` / `role-convention`: black-box compatibility verification |
| `anticaptrad/act-infra` | `deploys` | `anticaptrad/act-ai-server.ts` | `inferred` / `role-convention`: product infrastructure declares runtime resources |
| `anticaptrad/act-infra` | `deploys` | `anticaptrad/act-api-server.rs` | `inferred` / `role-convention`: product infrastructure declares runtime resources |
| `anticaptrad/act-infra` | `deploys` | `anticaptrad/act-mcp-server.rs` | `inferred` / `role-convention`: product infrastructure declares runtime resources |
| `anticaptrad/act-infra` | `deploys` | `anticaptrad/act-sync` | `inferred` / `role-convention`: product infrastructure declares runtime resources |
| `anticaptrad/act-infra` | `deploys` | `anticaptrad/act-web-server.rs` | `inferred` / `role-convention`: product infrastructure declares runtime resources |
| `anticaptrad/act-mcp-server.rs` | `calls` | `anticaptrad/act-api-server.rs` | `inferred` / `role-convention`: agent tools use the authenticated product API |
| `anticaptrad/act-mcp-server.rs` | `uses_sdk` | `anticaptrad/act-clients` | `inferred` / `role-convention`: agent adapter reuses the typed product SDK |
| `anticaptrad/act-monorepo` | `composes` | `anticaptrad/act-ai-server.ts` | `inferred` / `role-convention`: development workspace and release bill of materials |
| `anticaptrad/act-monorepo` | `composes` | `anticaptrad/act-api-server.rs` | `inferred` / `role-convention`: development workspace and release bill of materials |
| `anticaptrad/act-monorepo` | `composes` | `anticaptrad/act-clients` | `inferred` / `role-convention`: development workspace and release bill of materials |
| `anticaptrad/act-monorepo` | `composes` | `anticaptrad/act-e2e` | `inferred` / `role-convention`: development workspace and release bill of materials |
| `anticaptrad/act-monorepo` | `composes` | `anticaptrad/act-infra` | `inferred` / `role-convention`: development workspace and release bill of materials |
| `anticaptrad/act-monorepo` | `composes` | `anticaptrad/act-interfaces` | `inferred` / `role-convention`: development workspace and release bill of materials |
| `anticaptrad/act-monorepo` | `composes` | `anticaptrad/act-mcp-server.rs` | `inferred` / `role-convention`: development workspace and release bill of materials |
| `anticaptrad/act-monorepo` | `composes` | `anticaptrad/act-sync` | `inferred` / `role-convention`: development workspace and release bill of materials |
| `anticaptrad/act-monorepo` | `composes` | `anticaptrad/act-web-server.rs` | `inferred` / `role-convention`: development workspace and release bill of materials |
| `anticaptrad/act-sync` | `synchronizes_with` | `anticaptrad/act-api-server.rs` | `inferred` / `role-convention`: sync exchanges state through the product service boundary |
| `anticaptrad/act-sync` | `uses_contracts_from` | `anticaptrad/act-interfaces` | `inferred` / `role-convention`: sync payloads follow canonical schemas |
| `anticaptrad/act-web-server.rs` | `calls` | `anticaptrad/act-api-server.rs` | `inferred` / `role-convention`: client uses the product service boundary |
| `organization://anticaptrad` | `reconciles_via` | `platform://opto-sync` | `platform-default` / `platform-policy`: product sync wraps the generic reconciliation engine |
| `organization://anticaptrad` | `deployed_via` | `platform://ORESoftware/k8s-cluster` | `platform-default` / `platform-policy`: immutable artifacts are promoted by digest through GitOps |
| `organization://anticaptrad` | `uses_transport_library` | `platform://ORESoftware/mcp-rust-libs` | `platform-default` / `platform-policy`: shared MCP transport and protocol hardening |
| `organization://anticaptrad` | `packaged_via` | `platform://zed-pkg` | `platform-default` / `platform-policy`: Zed resolves artifacts while submodules compose editable source |

## Composition, service, and observability contract

Git submodules compose editable source; Zed packages resolve packages/artifacts; dual-managed commits must match. Production deploys immutable image digests, not runtime source builds. Cross-service access uses APIs/SDKs/events rather than another service database. MCP uses the product API/SDK. Services emit OpenTelemetry traces, bounded metrics, and correlated structured logs.

## Privacy boundary

This public registry deliberately omits private repository names and edges; the count above makes the boundary explicit.
