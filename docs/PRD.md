# AI Agent Treasury & Micropayment Router — Product Requirements Document

## 1. Problem Statement

Autonomous AI agents increasingly orchestrate multi-step workflows that require paid access to LLM APIs, data providers, and compute services. Today, each payment typically requires human wallet signing, breaking agent autonomy and making sub-cent micropayments economically impractical.

Teams building multi-agent LLM systems need a programmatic treasury layer on Stellar/Soroban that:

- Allocates stablecoin budgets per agent or agent cohort
- Routes micropayments via condition-based path payments
- Integrates with passkey-based smart wallets (Stellar Smart Account Kit) for programmatic authorization
- Enforces spend policies without manual intervention per transaction

Without this infrastructure, agent systems either pre-pay vendors off-chain (losing on-chain auditability) or centralize funds in a custodial service (increasing trust and compliance risk).

## 2. Target Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| **Agent Platform Operator** | Runs multi-agent LLM orchestration (LangGraph, CrewAI, custom) | Central treasury with per-agent budget allocation and audit trail |
| **Agent Developer** | Builds autonomous agents consuming paid APIs | Simple SDK to request micropayments within allocated budget |
| **DeFi / Web3 Engineer** | Integrates on-chain payments into agent workflows | Soroban contracts with clear interfaces and test coverage |
| **Enterprise FinOps** | Manages spend governance for AI workloads | Spending limits, role-based access, observability hooks |

### User Stories

- As a **platform operator**, I want to deposit stablecoins into a treasury and allocate budgets to agent smart accounts so each agent can spend autonomously within limits.
- As an **agent developer**, I want to call a single SDK method to pay for an API call without managing wallets or signing keys directly.
- As a **DeFi engineer**, I want condition-based path payments so agents only settle when the requested resource is delivered.
- As **FinOps**, I want spending metrics and audit logs to reconcile agent costs against provider invoices.

## 3. Goals and Non-Goals

### Goals

| ID | Goal | Success Metric |
|----|------|----------------|
| G1 | Enable autonomous agent micropayments on Stellar/Soroban | Agent completes paid API call without human wallet signing |
| G2 | Per-agent budget isolation with treasury oversight | Agent cannot spend beyond allocated budget |
| G3 | Condition-based path-payment routing | Payment settles only when delivery condition is met |
| G4 | Smart Account Kit integration for programmatic keys | Agent wallet created via passkey/smart account flow |
| G5 | TypeScript and Rust SDKs for agent systems | SDK covers allocate → pay → audit lifecycle |
| G6 | Production-ready contract security patterns | Access control, pause, and audit events implemented |

### Non-Goals

- **Full LLM orchestration framework** — AI-Engine is payment infrastructure, not an agent runtime.
- **Vendor-side payment acceptance** — we route payments; merchants integrate separately.
- **Fiat on/off-ramp** — treasury assumes stablecoins are already deposited.
- **Cross-chain bridging** — Stellar/Soroban only in v1.
- **Regulatory compliance automation** — KYC/AML hooks are out of scope; operators handle compliance.
- **Real-time price oracles** — path payments use Stellar DEX liquidity; custom oracle integration is future work.

## 4. Core Features

| Feature | Description | Acceptance Criterion |
|---------|-------------|---------------------|
| **Treasury Contract** | Holds stablecoin deposits and tracks total/allocated balances | Admin can deposit; `allocated + unallocated = total`; events emitted on deposit |
| **Agent Budget Allocation** | Assign spend caps to agent addresses or smart accounts | Allocation reduces unallocated balance; agent spend cannot exceed allocation |
| **Micropayment Router** | Route stablecoin payments to vendor addresses with memos | Router debits agent budget and transfers to destination atomically |
| **Condition-Based Path Payment** | Pay via Stellar path when delivery conditions are met | Payment aborts if min-received or destination asset condition fails |
| **Smart Account Kit Hooks** | Create/link passkey smart wallets for agents | SDK exposes `createAgentWallet` and `linkTreasury` with policy metadata |
| **Multi-Agent Budget Pools** | Shared pool with per-agent sub-limits | Pool total enforced; individual agents capped independently |
| **Spend Policies** | Rate limits, allowlists, max single payment | Policy violation returns structured error without state change |
| **Audit Events** | On-chain events for allocate, pay, revoke, pause | Indexer can reconstruct agent spend history from events |
| **SDK (TypeScript)** | Agent runtime integration for Node/Bun | `allocateBudget`, `requestPayment`, `getBalance` methods with typed errors |
| **SDK (Rust)** | Native agent or contract-side helpers | Mirrors TS API; compiles in workspace without network fetch |
| **Observability Stubs** | Metrics/logging interfaces for operators | SDK emits structured log events; metrics counters defined |
| **CI/CD Skeleton** | Build, lint, test pipelines | GitHub Actions runs contract build and SDK typecheck on PR |

## 5. Technical Constraints

| Constraint | Implication |
|------------|-------------|
| **Soroban smart contracts (Rust)** | All on-chain logic in `no_std` Rust; WASM size and compute budget limits apply |
| **Stellar path payments** | Routing uses Stellar liquidity paths; slippage and asset trustlines must be configured |
| **Smart Account Kit** | Programmatic signing requires passkey/smart-account integration; session keys scoped by policy |
| **Stablecoin micropayments** | Primary asset USDC (or testnet equivalent); amounts in stroops/minimum units |
| **No dependency install in scaffold** | Cargo/npm manifests present; consumers run install locally |
| **Render deployment** | HTTP services bind `0.0.0.0:$PORT`; ephemeral filesystem — state on-chain only |
| **Multi-agent concurrency** | Contract design must handle concurrent spend attempts without overdraft |

## 6. Open Questions

| ID | Question | Owner | Notes |
|----|----------|-------|-------|
| OQ1 | Which testnet stablecoin asset codes will be canonical for demos? | Platform | USDC on testnet vs custom soroban token |
| OQ2 | Should condition fulfillment be on-chain oracle attestation or off-chain callback? | Architecture | Affects router contract interface |
| OQ3 | Smart Account Kit session key TTL defaults for agent workloads? | Security | Balance autonomy vs revocation speed |
| OQ4 | Required vendor allowlist vs open payments? | Product | FinOps may require allowlist in enterprise |
| OQ5 | Formal audit timeline before mainnet? | Leadership | Out of scaffold scope but affects launch |
| OQ6 | Indexer choice for audit event ingestion (Mercury, custom)? | Infra | SDK stubs only in v1 scaffold |
