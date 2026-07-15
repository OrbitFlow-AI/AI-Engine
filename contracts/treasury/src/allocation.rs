// Budget allocation logic for agent treasuries.
use soroban_sdk::{Address, Env, Symbol};
use ai_engine_shared::{
    AgentId, AllocationEvent, BudgetAllocation, ContractError,
};
use crate::{policy, storage};

pub fn allocate_budget(
    env: &Env,
    admin: &Address,
    agent: &AgentId,
    amount: i128,
    expires_at: u64,
) -> Result<BudgetAllocation, ContractError> {
    storage::require_admin(env, admin)?;
    storage::require_not_paused(env)?;

    if amount <= 0 {
        return Err(ContractError::InvalidAmount);
    }

    policy::validate_amount(env, amount)?;
    policy::check_and_record_daily_cap(env, amount)?;

    let total = storage::get_total_balance(env);
    let existing = storage::get_allocation(env, agent);
    let already_allocated = existing.as_ref().map(|a| a.allocated).unwrap_or(0);
    let new_total_allocated = already_allocated + amount;

    if new_total_allocated > total {
        return Err(ContractError::InsufficientTreasuryBalance);
    }

    let allocation = BudgetAllocation {
        agent: agent.clone(),
        allocated: new_total_allocated,
        spent: existing.as_ref().map(|a| a.spent).unwrap_or(0),
        expires_at,
    };

    storage::set_allocation(env, &allocation);

    env.events().publish(
        (Symbol::new(env, "budget_allocated"), agent.clone()),
        AllocationEvent {
            agent: agent.clone(),
            amount,
            expires_at,
        },
    );

    Ok(allocation)
}

pub fn get_remaining_budget(env: &Env, agent: &AgentId) -> Result<i128, ContractError> {
    let allocation = storage::get_allocation(env, agent)
        .ok_or(ContractError::AgentNotFound)?;

    let now = env.ledger().timestamp();
    if allocation.expires_at > 0 && now > allocation.expires_at {
        return Err(ContractError::BudgetExpired);
    }

    Ok(allocation.remaining())
}

pub fn record_spend(
    env: &Env,
    agent: &AgentId,
    amount: i128,
) -> Result<BudgetAllocation, ContractError> {
    storage::require_not_paused(env)?;

    let mut allocation = storage::get_allocation(env, agent)
        .ok_or(ContractError::AgentNotFound)?;

    if !allocation.can_spend(amount) {
        return Err(ContractError::InsufficientBudget);
    }

    allocation.spent += amount;
    storage::set_allocation(env, &allocation);
    Ok(allocation)
}

pub fn revoke_budget(
    env: &Env,
    admin: &Address,
    agent: &AgentId,
) -> Result<i128, ContractError> {
    storage::require_admin(env, admin)?;

    let allocation = storage::get_allocation(env, agent)
        .ok_or(ContractError::AgentNotFound)?;

    let reclaimed = allocation.remaining();
    storage::set_allocation(env, &BudgetAllocation {
        agent: agent.clone(),
        allocated: allocation.spent,
        spent: allocation.spent,
        expires_at: allocation.expires_at,
    });

    env.events().publish(
        (Symbol::new(env, "budget_revoked"), agent.clone()),
        ai_engine_shared::BudgetRevokedEvent {
            agent: agent.clone(),
            reclaimed,
            revoked_by: admin.clone(),
        },
    );

    Ok(reclaimed)
}
