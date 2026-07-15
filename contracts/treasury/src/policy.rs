// Allocation policy enforcement — daily allocation cap and per-agent bounds.
use soroban_sdk::Env;
use ai_engine_shared::{AllocationPolicy, ContractError};
use crate::storage;

const SECONDS_PER_DAY: u64 = 86_400;

/// Validate an allocation policy before it is persisted.
pub fn validate_policy(policy: &AllocationPolicy) -> Result<(), ContractError> {
    if policy.daily_allocation_cap <= 0 {
        return Err(ContractError::InvalidPolicy);
    }
    if policy.min_allocation <= 0 || policy.max_allocation <= 0 {
        return Err(ContractError::InvalidPolicy);
    }
    if policy.min_allocation > policy.max_allocation {
        return Err(ContractError::InvalidPolicy);
    }
    if policy.max_allocation > policy.daily_allocation_cap {
        return Err(ContractError::InvalidPolicy);
    }
    Ok(())
}

/// Validate a single allocation amount against the configured policy bounds.
pub fn validate_amount(env: &Env, amount: i128) -> Result<(), ContractError> {
    let policy = storage::get_allocation_policy(env);
    if amount < policy.min_allocation || amount > policy.max_allocation {
        return Err(ContractError::PolicyViolation);
    }
    Ok(())
}

/// Check the admin's rolling daily allocation cap has capacity and record this allocation.
pub fn check_and_record_daily_cap(env: &Env, amount: i128) -> Result<(), ContractError> {
    let policy = storage::get_allocation_policy(env);
    let day = env.ledger().timestamp() / SECONDS_PER_DAY;

    let already_allocated = storage::get_daily_allocated(env, day);
    if already_allocated + amount > policy.daily_allocation_cap {
        return Err(ContractError::PolicyViolation);
    }

    storage::set_daily_allocated(env, day, already_allocated + amount);
    Ok(())
}
