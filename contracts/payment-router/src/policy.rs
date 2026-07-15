// Spend policy and sliding-window rate-limit enforcement for the router.
use soroban_sdk::Env;
use ai_engine_shared::{AgentId, ContractError, RateLimitWindow, SpendPolicy};
use crate::storage;

/// Validate a spend policy before it is persisted.
pub fn validate_policy(policy: &SpendPolicy) -> Result<(), ContractError> {
    if policy.max_single_payment <= 0 || policy.daily_limit <= 0 {
        return Err(ContractError::InvalidPolicy);
    }
    if policy.max_single_payment > policy.daily_limit {
        return Err(ContractError::InvalidPolicy);
    }
    if policy.rate_limit_window_seconds == 0 || policy.rate_limit_max_payments == 0 {
        return Err(ContractError::InvalidPolicy);
    }
    Ok(())
}

/// Check the agent's rate-limit window has capacity and record this payment against it.
/// Rolls the window forward first if it has elapsed.
pub fn check_and_record(env: &Env, agent: &AgentId) -> Result<(), ContractError> {
    let policy = storage::get_spend_policy(env);
    let now = env.ledger().timestamp();

    let window = storage::get_rate_limit(env, agent).unwrap_or(RateLimitWindow {
        window_seconds: policy.rate_limit_window_seconds,
        max_payments: policy.rate_limit_max_payments,
        window_start: now,
        payments_in_window: 0,
    });

    let mut rolled = window.rolled(now);
    // Policy may have changed since the window was created — always apply current caps.
    rolled.window_seconds = policy.rate_limit_window_seconds;
    rolled.max_payments = policy.rate_limit_max_payments;

    if !rolled.has_capacity() {
        storage::set_rate_limit(env, agent, &rolled);
        return Err(ContractError::RateLimitExceeded);
    }

    rolled.payments_in_window += 1;
    storage::set_rate_limit(env, agent, &rolled);
    Ok(())
}
