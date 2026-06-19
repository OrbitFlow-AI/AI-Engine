# Deployment guide for AI-Engine on Stellar testnet and Render.
# AI Agent Treasury & Micropayment Router — Deployment Guide

## Prerequisites

- Rust stable with `wasm32-unknown-unknown` target
- Soroban CLI (for contract deployment)
- Node.js 18+ (for TypeScript SDK)
- Stellar testnet account with XLM for fees

## Contract Deployment

```bash
# Build WASM artifacts
cargo build --target wasm32-unknown-unknown --release -p ai-engine-treasury
cargo build --target wasm32-unknown-unknown --release -p ai-engine-payment-router

# Deploy via Soroban CLI (after install)
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/ai_engine_treasury.wasm \
  --network testnet

soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/ai_engine_payment_router.wasm \
  --network testnet
```

## Initialization Sequence

1. Deploy treasury contract → call `initialize(admin, token)`
2. Deploy payment router → call `initialize(admin, treasury, max_single_payment)`
3. Deposit USDC → `treasury.deposit(admin, amount)`
4. Create agent wallets via Smart Account Kit
5. Allocate budgets → `treasury.allocate_budget(admin, agent, amount, expires_at)`

## Environment Variables

Copy `.env.example` to `.env` and set:

- `TREASURY_CONTRACT_ID` — deployed treasury contract address
- `PAYMENT_ROUTER_CONTRACT_ID` — deployed router contract address
- `STELLAR_RPC_URL` — Soroban RPC endpoint

## Render Deployment

The `render.yaml` blueprint defines an optional indexer service:

- Binds to `0.0.0.0:$PORT` per Render requirements
- Ephemeral filesystem — all state on-chain
- Free tier spins down after 15 minutes inactivity

## CI/CD

- **CI** (`.github/workflows/ci.yml`): runs on every push/PR
- **Deploy** (`.github/workflows/deploy.yml`): manual workflow dispatch for testnet

## Post-Deploy Verification

```bash
# Query treasury balance
soroban contract invoke --id $TREASURY_CONTRACT_ID -- total_balance

# Query agent budget
soroban contract invoke --id $TREASURY_CONTRACT_ID -- get_budget --agent $AGENT_ADDRESS
```
