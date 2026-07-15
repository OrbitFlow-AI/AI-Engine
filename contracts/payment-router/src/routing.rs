// Payment routing logic — validate, escrow, and settle micropayments.
use soroban_sdk::{Address, Env, Symbol, Vec};
use ai_engine_shared::{
    AgentId, ContractError, PathPaymentEvent, PaymentInitiatedEvent,
    PaymentRefundedEvent, PaymentRequest, PaymentSettledEvent,
};
use crate::{conditions, policy, storage};

const DEFAULT_DAILY_LIMIT: i128 = 10_000_000; // 1 USDC in stroops (7 decimals)

pub fn initiate_payment(
    env: &Env,
    agent_caller: &Address,
    request: &PaymentRequest,
) -> Result<u64, ContractError> {
    storage::require_not_paused(env)?;
    conditions::validate_request(request)?;
    conditions::validate_condition(env, &request.condition)?;

    agent_caller.require_auth();
    if agent_caller != &request.agent.0 {
        return Err(ContractError::Unauthorized);
    }

    if !storage::is_vendor_allowed(env, &request.vendor) {
        return Err(ContractError::VendorNotAllowed);
    }

    let max_single = storage::get_max_single_payment(env);
    if request.amount > max_single {
        return Err(ContractError::PaymentExceedsLimit);
    }

    let daily_spent = storage::get_daily_spent(env, &request.agent);
    if daily_spent + request.amount > DEFAULT_DAILY_LIMIT {
        return Err(ContractError::PaymentExceedsLimit);
    }

    policy::check_and_record(env, &request.agent)?;

    let payment_id = storage::next_payment_id(env);
    storage::store_pending(env, payment_id, request);

    env.events().publish(
        (Symbol::new(env, "payment_initiated"), request.agent.clone()),
        PaymentInitiatedEvent {
            agent: request.agent.clone(),
            vendor: request.vendor.clone(),
            amount: request.amount,
            reference: request.reference.clone(),
        },
    );

    env.events().publish(
        (Symbol::new(env, "path_payment"), request.agent.clone()),
        PathPaymentEvent {
            agent: request.agent.clone(),
            condition: request.condition.clone(),
            path_length: 1,
        },
    );

    Ok(payment_id)
}

pub fn settle_payment(
    env: &Env,
    treasury: &Address,
    payment_id: u64,
    received_amount: i128,
) -> Result<(), ContractError> {
    storage::require_not_paused(env)?;

    let request = storage::get_pending(env, payment_id)
        .ok_or(ContractError::AgentNotFound)?;

    conditions::is_condition_met(env, &request.condition, received_amount)?;

    // Invoke treasury record_spend in production via cross-contract call
    let _ = treasury;

    storage::add_daily_spent(env, &request.agent, request.amount);

    env.events().publish(
        (Symbol::new(env, "payment_settled"), request.agent.clone()),
        PaymentSettledEvent {
            agent: request.agent.clone(),
            vendor: request.vendor.clone(),
            amount: request.amount,
            reference: request.reference.clone(),
            memo: request.condition.memo.clone(),
        },
    );

    Ok(())
}

pub fn refund_payment(
    env: &Env,
    admin: &Address,
    payment_id: u64,
    reason: Symbol,
) -> Result<(), ContractError> {
    storage::require_admin(env, admin)?;

    let request = storage::get_pending(env, payment_id)
        .ok_or(ContractError::AgentNotFound)?;

    env.events().publish(
        (Symbol::new(env, "payment_refunded"), request.agent.clone()),
        PaymentRefundedEvent {
            agent: request.agent.clone(),
            amount: request.amount,
            reference: request.reference.clone(),
            reason,
        },
    );

    Ok(())
}
