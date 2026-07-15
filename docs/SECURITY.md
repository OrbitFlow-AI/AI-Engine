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
- **Spend policy rate limiting**: sliding-window cap on payments per agent per time window, in
  addition to the existing max-single-payment and daily-limit checks
- **Allocation policy bounds**: treasury enforces a daily allocation cap and per-agent min/max
  bounds so a single admin action cannot drain the treasury in one call

## Governance

- Both treasury and router support an optional **multisig governance** mode: `init_governance`
  configures a signer set and approval threshold; `propose_action` / `approve_proposal` /
  `execute_proposal` gate admin-equivalent actions (`ChangeAdmin`, `SetPause`, `AddSigner`,
  `RemoveSigner`, `SetThreshold`) behind N-of-M approval
- Proposals carry an optional TTL (`expires_at`); expired or already-executed/cancelled proposals
  cannot be approved or executed
- Removing a signer or lowering the threshold is itself validated against
  `InvalidThreshold` so governance cannot be configured into a state that requires more
  approvals than there are signers
- Until `init_governance` is called, the single admin key remains the sole authority — governance
  is additive and does not remove the existing admin-only access control paths

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
| Admin key compromise | Optional multisig governance (N-of-M proposal approval); pause + revoke |
| Runaway budget allocation | Allocation policy: daily cap + per-agent min/max bounds |
| Payment flooding / spam | Router spend policy: sliding-window rate limit per agent |
| Router exploit | Pause router; treasury holds funds until spend recorded |
