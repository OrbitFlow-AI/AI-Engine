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

## 2. Target Users

| Persona | Needs | Success Signal |
|---------|-------|----------------|
| **Agent Platform Engineer** | SDK to allocate budgets and trigger payments from agent runtime | Integrates treasury in <1 day without Soroban expertise |
| **Treasury / Ops Admin** | Policy controls, audit trail, budget caps | Can revoke agent spend and inspect ledger in dashboard or CLI |
| **Vendor / Service Provider** | Reliable stablecoin settlement for API usage | Receives path-routed USDC payments with clear memo/reference |
| **Smart Wallet Integrator** | Smart Account Kit hooks for passkey auth | Agents sign via delegated session keys within policy bounds |

Primary deployment context: multi-agent LLM orchestration frameworks (LangGraph, custom agent fleets) running on server-side infrastructure with programmatic key material.

## 3. Goals and Non-Goals

### Goals

- Provide Soroban smart contracts for agent treasury allocation and micropayment routing
- Expose TypeScript and Rust SDKs for agent runtimes to request spend within policy
- Support condition-based path payments (destination asset, max slippage, min received)
- Integrate with Stellar Smart Account Kit for passkey-based programmatic wallets
- Enable multi-agent budget pools with per-agent sub-allocations and spend tracking
- Ship observable, testable scaffold suitable for testnet deployment

### Non-Goals

- Building a full LLM orchestration framework (integrate with existing agent systems)
- Fiat on/off-ramp or KYC/compliance tooling in v1
- Custodial key storage; agents must use Smart Account Kit or bring-your-own signer
- Mainnet production deployment and formal audit in initial scaffold phases
- Real-time off-chain price oracles beyond Stellar path payment mechanics
