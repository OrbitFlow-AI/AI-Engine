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

## TypeScript SDK

```typescript
import {
  AgentWallet,
  TreasuryClient,
  RouterClient,
  BudgetManager,
  ConditionBuilder,
} from '@ai-engine/sdk';
```

See [DEPLOYMENT.md](./DEPLOYMENT.md) for initialization sequence.
