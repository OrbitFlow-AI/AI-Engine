# Contract unit tests — run with `cargo test --workspace`.
# Treasury and payment-router tests are embedded in each contract crate.

## Running Tests

```bash
cargo test -p ai-engine-shared
cargo test -p ai-engine-treasury
cargo test -p ai-engine-payment-router
cargo test -p ai-engine-agent-sdk
```

## Test Categories

- **Unit tests**: allocation logic, condition validation, policy enforcement
- **Integration tests**: TypeScript E2E in `tests/integration/`
- **Security tests**: pause, unauthorized access, over-budget (future)

## Adding Tests

Add `#[cfg(test)]` modules in contract source files. Use `soroban-sdk` testutils feature for ledger simulation.
