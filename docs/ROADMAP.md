# AI Agent Treasury & Micropayment Router — Roadmap

> Phased delivery plan derived from [PRD.md](./PRD.md).

## Timeline Summary

| Phase | Name | Objective | Complexity | Depends On |
|-------|------|-----------|------------|------------|
| 0 | Scaffold | Monorepo structure, configs, CI skeleton | Low | — |
| 1 | Core Loop | Treasury allocation → micropayment routing E2E | Medium | Phase 0 |
| 2 | Feature Complete | Full PRD features, edge cases, multi-agent budgets | High | Phase 1 |
| 3 | Production Hardening | Security, observability, deployment, tests | Medium | Phase 2 |

---

## Phase 0: Scaffold

| Field | Detail |
|-------|--------|
| **Objective** | Establish monorepo folder structure, base configs, CI skeleton, and SDK/contract stubs |
| **Deliverables** | Root `Cargo.toml` workspace, `package.json` workspaces, `.gitignore`, `README.md`, `.env.example`, GitHub Actions CI, Soroban contract crates (treasury, payment-router, shared), TypeScript SDK package, Rust agent-sdk crate |
| **Exit Criteria** | Repo builds structurally; all crates/packages have entry points; CI workflow validates on push; README documents project layout |
| **Complexity** | Low |

---

## Phase 1: Core Loop

| Field | Detail |
|-------|--------|
| **Objective** | Minimal end-to-end flow: treasury budget allocation → agent micropayment routing |
| **Deliverables** | Treasury contract (deposit, allocate, balance query), Payment Router contract (initiate payment, validate budget), TypeScript `TreasuryClient` + `AgentWallet`, Smart Account Kit integration hooks, basic unit tests |
| **Exit Criteria** | Agent can be allocated budget and initiate a payment request via SDK; contracts enforce budget limits; Smart Account Kit hook interface defined |
| **Complexity** | Medium |

---

## Phase 2: Feature Complete

| Field | Detail |
|-------|--------|
| **Objective** | Implement remaining PRD core features with edge case handling |
| **Deliverables** | Condition-based path-payment logic, multi-agent budget management (cohorts, revocation), spending policies (rate limits, allowlists), error types, Rust SDK client, integration test stubs |
| **Exit Criteria** | All PRD core feature acceptance criteria met in code; edge cases (over-budget, paused router, expired conditions) handled; multi-agent budget isolation verified in tests |
| **Complexity** | High |

---

## Phase 3: Production Hardening

| Field | Detail |
|-------|--------|
| **Objective** | Prepare for testnet deployment with security, observability, and CI/CD |
| **Deliverables** | Auth/access control patterns, admin role management, emergency pause, SDK logging/metrics stubs, performance notes, Render-compatible deploy config, security/deployment docs, test scaffolding |
| **Exit Criteria** | Security patterns documented and implemented; observability hooks emit structured events; CI/CD pipeline runs build+test; deployment guide complete |
| **Complexity** | Medium |
