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
| **Treasury Contract** | On-chain stablecoin vault with admin controls | Admin can deposit/withdraw; balance queryable; emits deposit events |
| **Agent Budget Allocation** | Assign spending limits per agent address | Admin allocates budget; agent balance decrements on spend; rejects over-budget |
| **Payment Router** | Route micropayments to vendor destinations | Agent initiates payment; router validates budget + conditions; settles on-chain |
| **Condition-Based Path Payments** | Pay only when resource delivery condition met | Router holds escrow; releases on condition proof or timeout refund |
| **Smart Account Kit Hooks** | Programmatic agent wallet creation and signing | SDK exposes `createAgentWallet()` and `signPayment()` via Smart Account Kit |
| **Multi-Agent Budget Management** | Cohort budgets, sub-allocation, revocation | Admin can set cohort cap; revoke agent budget; reclaim unspent funds |
| **TypeScript Agent SDK** | High-level client for agent runtime integration | `TreasuryClient`, `RouterClient`, `BudgetManager` exported from `@ai-engine/sdk` |
| **Rust Agent SDK** | Native client for Rust-based agent systems | `AgentClient` trait with allocate, pay, balance query methods |
| **Spending Policies** | Rate limits, allowlists, max single payment | Router rejects payments exceeding per-tx or daily limits |
| **Observability Stubs** | Logging and metrics hooks for production | SDK emits structured payment events; metrics counters for spend/volume |
| **CI/CD Pipeline** | Automated contract build, test, deploy skeleton | GitHub Actions runs `cargo test` and SDK typecheck on PR |
| **Security Patterns** | Role-based access, pause, reentrancy guards | Only admin can allocate; emergency pause halts router; no reentrant spend |

## 5. Technical Constraints

| Constraint | Detail |
|------------|--------|
| **Blockchain** | Stellar testnet/mainnet; Soroban smart contracts (Rust `#![no_std]`) |
| **Token Standard** | Stellar classic assets (USDC) via Soroban token interface |
| **Wallet Integration** | Stellar Smart Account Kit for passkey-based programmatic signing |
| **Path Payments** | Stellar DEX path payment strict send/receive for stablecoin routing |
| **Contract Size** | Soroban WASM size limits; shared types extracted to `contracts/shared` crate |
| **Agent Runtime** | TypeScript SDK targets Node.js 18+; Rust SDK targets stable toolchain |
| **Deployment** | Render-compatible CI/CD skeleton; ephemeral filesystem (no local state) |
| **No Dependency Install** | Scaffold phase writes code/config only; CI validates on push |

## 6. Open Questions

| ID | Question | Impact | Proposed Default |
|----|----------|--------|------------------|
| OQ1 | Which stablecoin(s) to support at launch? | Token contract addresses | USDC on Stellar testnet |
| OQ2 | How are delivery conditions verified on-chain? | Path payment escrow release | Off-chain oracle attestation via admin/multisig in v1 |
| OQ3 | Smart Account Kit session key TTL? | Agent autonomy vs. security | 24h delegated session keys with spend cap |
| OQ4 | Cohort budget nesting depth? | Contract storage complexity | Flat: treasury → agent (no nested cohorts in v1) |
| OQ5 | Minimum micropayment amount? | Soroban fee economics | 0.01 USDC floor to cover network fees |
| OQ6 | Formal audit timeline? | Mainnet readiness | Post-scaffold; testnet only until audit complete |
