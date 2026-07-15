# AI-Engine

**AI Agent Treasury & Micropayment Router** — a Soroban/Stellar smart contract framework for autonomous multi-agent LLM micropayments.

## Overview

AI-Engine enables AI agent platforms to:

- Deposit stablecoins into an on-chain **treasury**
- Allocate **per-agent budgets** with spending isolation
- Route **micropayments** via condition-based path payments
- Integrate with **Stellar Smart Account Kit** for passkey-based programmatic wallets
- Enforce **spend policies** — max single payment, daily limit, and per-agent sliding-window rate
  limits — on every payment
- Bound treasury allocations with a **daily allocation cap** and per-agent min/max policy
- Gate admin actions behind optional **N-of-M multisig governance** on both the treasury and
  payment-router contracts
- Drive the full core loop from the command line via **`@ai-engine/cli`**

## Monorepo Structure

```
AI-Engine/
├── contracts/
│   ├── shared/          # Shared types, events, errors, governance helpers
│   ├── treasury/        # Treasury, budget allocation, allocation policy, and governance
│   └── payment-router/  # Micropayment routing, spend policy, and governance
├── packages/
│   ├── sdk/             # TypeScript SDK for agent systems
│   └── cli/             # Command-line interface wrapping the TypeScript SDK
├── crates/
│   └── agent-sdk/       # Rust SDK for agent systems
├── docs/                # PRD, ROADMAP, guides
├── tests/               # Integration test scaffolding
└── .github/workflows/   # CI/CD pipelines
```

## Tech Stack

- **Stellar / Soroban** — smart contracts in Rust (`#![no_std]`)
- **TypeScript SDK** — agent runtime integration (Node.js 18+)
- **Smart Account Kit** — passkey-based programmatic signing
- **USDC** — primary stablecoin for micropayments

## Quick Start

```bash
# Build Soroban contracts (requires soroban-cli)
cargo build --target wasm32-unknown-unknown --release

# TypeScript SDK
cd packages/sdk && npm install && npm run build

# CLI
cd packages/cli && npm install && npm run build
npx ai-engine treasury budget <agent>
```

## Documentation

- [Product Requirements](./docs/PRD.md)
- [Roadmap](./docs/ROADMAP.md)
- [Wave Contributor Onboarding](./docs/CONTRIBUTING-WAVE.md)
- [Security Guidelines](./docs/SECURITY.md)
- [Deployment Guide](./docs/DEPLOYMENT.md)

## License

Apache-2.0
