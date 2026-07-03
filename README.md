# AI-Engine

**AI Agent Treasury & Micropayment Router** — a Soroban/Stellar smart contract framework for autonomous multi-agent LLM micropayments.

## Overview

AI-Engine enables AI agent platforms to:

- Deposit stablecoins into an on-chain **treasury**
- Allocate **per-agent budgets** with spending isolation
- Route **micropayments** via condition-based path payments
- Integrate with **Stellar Smart Account Kit** for passkey-based programmatic wallets

## Monorepo Structure

```
AI-Engine/
├── contracts/
│   ├── shared/          # Shared types, events, errors
│   ├── treasury/        # Treasury & budget allocation contract
│   └── payment-router/  # Micropayment routing & path payments
├── packages/
│   └── sdk/             # TypeScript SDK for agent systems
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
```

## Documentation

- [Product Requirements](./docs/PRD.md)
- [API Reference](./docs/API.md)
- [Roadmap](./docs/ROADMAP.md)
- [Security Guidelines](./docs/SECURITY.md)
- [Deployment Guide](./docs/DEPLOYMENT.md)

## License

Apache-2.0
