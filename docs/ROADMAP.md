# AI Agent Treasury & Micropayment Router — Roadmap

Derived from [PRD.md](./PRD.md). Phases are sequential; exit criteria must pass before advancing.

## Phase 0: Scaffold

| Field | Detail |
|-------|--------|
| **Objective** | Establish monorepo layout, base configs, CI skeleton, and SDK/contract stubs |
| **Deliverables** | Cargo workspace, Soroban contract crates, TS/Rust SDK packages, README, `.gitignore`, GitHub Actions, env templates |
| **Exit Criteria** | Repo builds structurally (`cargo check --workspace` manifest-valid); CI workflow present; docs readable |
| **Complexity** | Low |

## Phase 1: Core Loop

| Field | Detail |
|-------|--------|
| **Objective** | Minimal end-to-end flow: treasury deposit → agent allocation → micropayment route |
| **Deliverables** | Treasury + payment-router Soroban contracts, basic agent SDK interface, Smart Account Kit hook stubs |
| **Exit Criteria** | SDK can call allocate + pay against contract interfaces; integration tests scaffold passes locally |
| **Complexity** | Medium |

## Phase 2: Feature Complete

| Field | Detail |
|-------|--------|
| **Objective** | Implement remaining PRD core features with edge cases and multi-agent pools |
| **Deliverables** | Condition-based path payments, spend policies, multi-agent budget pools, structured errors, audit events |
| **Exit Criteria** | All PRD core feature acceptance criteria met in code; negative paths covered in tests |
| **Complexity** | High |

## Phase 3: Production Hardening

| Field | Detail |
|-------|--------|
| **Objective** | Security, observability, performance, and deployment readiness |
| **Deliverables** | Auth patterns, pause/admin guards, metrics/logging stubs, Render-compatible deploy config, test scaffolding |
| **Exit Criteria** | CI runs build+test jobs; deployment pipeline documented; security checklist in docs |
| **Complexity** | Medium |

## Summary Timeline

| Phase | Focus | Complexity | Depends On |
|-------|-------|------------|------------|
| 0 | Scaffold | Low | — |
| 1 | Core Loop | Medium | Phase 0 |
| 2 | Feature Complete | High | Phase 1 |
| 3 | Production Hardening | Medium | Phase 2 |

**Estimated sequence:** Phase 0 → 1 → 2 → 3. Parallel work possible on SDK vs contracts within each phase after interfaces are defined.
