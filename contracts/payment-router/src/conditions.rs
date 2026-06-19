// Condition validation for path-payment routing.
use soroban_sdk::Env;
use ai_engine_shared::{ContractError, PaymentCondition, PaymentRequest};

pub fn validate_condition(env: &Env, condition: &PaymentCondition) -> Result<(), ContractError> {
    if condition.min_received <= 0 {
        return Err(ContractError::InvalidCondition);
    }
    if condition.max_slippage_bps > 10_000 {
        return Err(ContractError::InvalidCondition);
    }
    let current_ledger = env.ledger().sequence();
    if condition.deadline_ledger <= current_ledger {
        return Err(ContractError::InvalidCondition);
    }
    Ok(())
}

pub fn validate_request(request: &PaymentRequest) -> Result<(), ContractError> {
    if request.amount <= 0 {
        return Err(ContractError::InvalidAmount);
    }
    Ok(())
}

pub fn is_condition_met(
    env: &Env,
    condition: &PaymentCondition,
    received_amount: i128,
) -> Result<(), ContractError> {
    if env.ledger().sequence() > condition.deadline_ledger {
        return Err(ContractError::PaymentTimeout);
    }
    if received_amount < condition.min_received {
        return Err(ContractError::ConditionNotMet);
    }
    Ok(())
}

pub fn compute_min_with_slippage(amount: i128, slippage_bps: u32) -> i128 {
    let slippage = (amount as i128) * (slippage_bps as i128) / 10_000;
    amount - slippage
}
