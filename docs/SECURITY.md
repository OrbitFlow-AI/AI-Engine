# Security guidelines for AI-Engine treasury and payment router deployment.
# AI Agent Treasury & Micropayment Router — Security Guidelines

## Access Control

- **Admin-only operations**: treasury deposit/withdraw, budget allocation, router pause, vendor allowlist
- **Agent-scoped signing**: agents can only initiate payments from their own smart account address
- **Router authorization**: only the registered payment router can call `treasury.record_spend`

## Key Management

- Use **Stellar Smart Account Kit** passkeys for human-controlled admin accounts
- Agent session keys must have **TTL limits** and **spend caps** enforced by Smart Account Kit policy
- Never store private keys in environment variables or source code
- Rotate session keys on compromise or TTL expiry

## Contract Security

- Emergency **pause** on both treasury and router contracts
- **Reentrancy guards** on spend recording paths
- **Budget expiry** prevents stale allocations from being spent
- **Vendor allowlist** optional for enterprise deployments

## Operational Security

- Monitor `payment_initiated`, `payment_settled`, and `budget_revoked` events
- Set alerts on unusual spend velocity or single-payment limit breaches
- Run formal audit before mainnet deployment

## Threat Model (v1)

| Threat | Mitigation |
|--------|------------|
| Agent overspend | Per-agent budget + daily limits + max single payment |
| Compromised session key | TTL rotation + spend cap via Smart Account Kit |
| Malicious vendor | Optional vendor allowlist |
| Admin key compromise | Multisig admin (future); pause + revoke |
| Router exploit | Pause router; treasury holds funds until spend recorded |
