# AI-Engine API Reference
## Treasury Contract

| Method | Params | Returns | Auth |
|--------|--------|---------|------|
| `initialize` | admin, token | () | admin |
| `deposit` | depositor, amount | i128 (new balance) | admin |
| `allocate_budget` | admin, agent, amount, expires_at | BudgetAllocation | admin |
| `get_budget` | agent | i128 (remaining) | any |
| `record_spend` | router, agent, amount | BudgetAllocation | router |
| `revoke_budget` | admin, agent | i128 (reclaimed) | admin |
| `total_balance` | — | i128 | any |
| `pause` / `unpause` | admin | () | admin |
| `set_allocation_policy` | admin, allocation_policy | () | admin |
| `get_allocation_policy` | — | AllocationPolicy | any |
| `init_governance` | admin, signers, threshold | () | admin |
| `propose_action` | proposer, action, ttl_seconds | u64 (proposal_id) | signer |
| `approve_proposal` | approver, proposal_id | () | signer |
| `execute_proposal` | executor, proposal_id | () | any |
| `cancel_proposal` | admin, proposal_id | () | admin |
| `get_proposal` | proposal_id | Option\<GovernanceProposal\> | any |

## Payment Router Contract

| Method | Params | Returns | Auth |
|--------|--------|---------|------|
| `initialize` | admin, treasury, max_single_payment | () | admin |
| `initiate_payment` | agent, vendor, amount, asset, condition, reference | u64 (payment_id) | agent |
| `settle_payment` | payment_id, received_amount | () | any |
| `refund_payment` | admin, payment_id, reason | () | admin |
| `set_vendor_allowlist` | admin, vendors | () | admin |
| `set_max_payment` | admin, max | () | admin |
| `pause` / `unpause` | admin | () | admin |
| `set_spend_policy` | admin, spend_policy | () | admin |
| `get_spend_policy` | — | SpendPolicy | any |
| `init_governance` | admin, signers, threshold | () | admin |
| `propose_action` | proposer, action, ttl_seconds | u64 (proposal_id) | signer |
| `approve_proposal` | approver, proposal_id | () | signer |
| `execute_proposal` | executor, proposal_id | () | any |
| `cancel_proposal` | admin, proposal_id | () | admin |
| `get_proposal` | proposal_id | Option\<GovernanceProposal\> | any |

`SpendPolicy` bundles `max_single_payment`, `daily_limit`, and a sliding-window rate limit
(`rate_limit_window_seconds`, `rate_limit_max_payments`) enforced per agent on every
`initiate_payment` call. `ProposalAction` is one of `ChangeAdmin`, `SetPause`, `AddSigner`,
`RemoveSigner`, or `SetThreshold` — the same enum is shared by the treasury and router governance
modules so a single multisig signer set can manage both contracts identically.

## TypeScript SDK

```typescript
import {
  AgentWallet,
  TreasuryClient,
  RouterClient,
  BudgetManager,
  ConditionBuilder,
  PolicyManager,
  GovernanceClient,
  Logger,
  MetricsCollector,
  WebhookNotifier,
  withRetry,
} from '@ai-engine/sdk';
```

| Module | Purpose |
|--------|---------|
| `PolicyManager` | Client-side spend-policy validation and rate-limit pre-flight checks, mirroring on-chain enforcement |
| `GovernanceClient` | Propose, approve, execute, and cancel multisig governance actions |
| `WebhookNotifier` | Forwards `PaymentEvent`s to an operator webhook with retry/backoff |
| `withRetry` | Exponential-backoff wrapper for flaky RPC calls, used internally by `RouterClient.initiatePayment` |
| `RouterClient.initiatePaymentBatch` | Sequentially submits multiple payment requests, collecting per-request results |

## CLI

`@ai-engine/cli` wraps the TypeScript SDK for shell and CI use — see
[packages/cli/README.md](../packages/cli/README.md) for the full command reference
(`treasury`, `router`, `governance` subcommands).

## Rust SDK

`ai-engine-agent-sdk` mirrors the TypeScript SDK's spend-policy and retry primitives:
`RateLimiter` (sliding-window rate limiting) and `with_retry` (synchronous exponential backoff).

See [DEPLOYMENT.md](./DEPLOYMENT.md) for initialization sequence.
