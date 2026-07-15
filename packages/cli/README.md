# @ai-engine/cli

Command-line interface for the AI-Engine treasury, router, and governance Soroban contracts. Wraps the
TypeScript SDK (`@ai-engine/sdk`) so operators and CI pipelines can drive the core loop without writing
scripts.

## Install

```bash
cd packages/cli && npm install && npm run build
```

## Configuration

The CLI reads connection details from environment variables (see `.env.example` at the repo root):

| Variable | Purpose |
|----------|---------|
| `AI_ENGINE_AGENT_ID` | Identifier for the calling agent |
| `AI_ENGINE_NETWORK_PASSPHRASE` | Stellar network passphrase (testnet by default) |
| `AI_ENGINE_RPC_URL` | Soroban RPC endpoint |
| `AI_ENGINE_TREASURY_CONTRACT_ID` | Deployed treasury contract id |
| `AI_ENGINE_ROUTER_CONTRACT_ID` | Deployed payment-router contract id |

## Usage

```bash
# Treasury
ai-engine treasury deposit <admin> <amount>
ai-engine treasury allocate <admin> <agent> <amount> [expiresAt]
ai-engine treasury budget <agent>
ai-engine treasury set-policy <admin> <dailyCap> <min> <max>

# Router
ai-engine router pay <agent> <vendor> <amount> <asset> <minReceived> <deadlineLedger> [memo]
ai-engine router settle <paymentId> <receivedAmount>
ai-engine router refund <admin> <paymentId> [reason]

# Governance
ai-engine governance init <admin> <threshold> <signer...>
ai-engine governance propose <proposer> <change-admin|set-pause|add-signer|remove-signer|set-threshold> <value> [ttlSeconds]
ai-engine governance approve <approver> <proposalId>
ai-engine governance execute <executor> <proposalId>
ai-engine governance cancel <admin> <proposalId>
```

## Scope

The CLI is a thin wrapper around `@ai-engine/sdk` clients; it does not implement its own contract logic.
See [docs/API.md](../../docs/API.md) for the underlying method reference.
