// Treasury contract persistent storage keys and accessors.
use soroban_sdk::{contracttype, Address, Env, Map, Symbol};
use ai_engine_shared::{AgentId, BudgetAllocation, CohortBudget, ContractError};

const ADMIN: Symbol = Symbol::short("ADMIN");
const TOKEN: Symbol = Symbol::short("TOKEN");
const PAUSED: Symbol = Symbol::short("PAUSED");
const TOTAL_BALANCE: Symbol = Symbol::short("TBAL");
const ALLOCATIONS: Symbol = Symbol::short("ALLOC");
const COHORTS: Symbol = Symbol::short("COHORT");

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Token,
    Paused,
    TotalBalance,
    Allocation(AgentId),
    Cohort(Symbol),
}

pub fn is_initialized(env: &Env) -> bool {
    env.storage().instance().has(&DataKey::Admin)
}

pub fn get_admin(env: &Env) -> Result<Address, ContractError> {
    env.storage()
        .instance()
        .get(&DataKey::Admin)
        .ok_or(ContractError::NotInitialized)
}

pub fn set_admin(env: &Env, admin: &Address) {
    env.storage().instance().set(&DataKey::Admin, admin);
}

pub fn get_token(env: &Env) -> Result<Address, ContractError> {
    env.storage()
        .instance()
        .get(&DataKey::Token)
        .ok_or(ContractError::NotInitialized)
}

pub fn set_token(env: &Env, token: &Address) {
    env.storage().instance().set(&DataKey::Token, token);
}

pub fn is_paused(env: &Env) -> bool {
    env.storage()
        .instance()
        .get(&DataKey::Paused)
        .unwrap_or(false)
}

pub fn set_paused(env: &Env, paused: bool) {
    env.storage().instance().set(&DataKey::Paused, &paused);
}

pub fn get_total_balance(env: &Env) -> i128 {
    env.storage()
        .instance()
        .get(&DataKey::TotalBalance)
        .unwrap_or(0)
}

pub fn set_total_balance(env: &Env, balance: i128) {
    env.storage().instance().set(&DataKey::TotalBalance, &balance);
}

pub fn get_allocation(env: &Env, agent: &AgentId) -> Option<BudgetAllocation> {
    env.storage()
        .persistent()
        .get(&DataKey::Allocation(agent.clone()))
}

pub fn set_allocation(env: &Env, allocation: &BudgetAllocation) {
    env.storage()
        .persistent()
        .set(&DataKey::Allocation(allocation.agent.clone()), allocation);
}

pub fn get_cohort(env: &Env, cohort_id: &Symbol) -> Option<CohortBudget> {
    env.storage()
        .persistent()
        .get(&DataKey::Cohort(cohort_id.clone()))
}

pub fn set_cohort(env: &Env, cohort: &CohortBudget) {
    env.storage()
        .persistent()
        .set(&DataKey::Cohort(cohort.cohort_id.clone()), cohort);
}

pub fn require_admin(env: &Env, caller: &Address) -> Result<(), ContractError> {
    let admin = get_admin(env)?;
    admin.require_auth();
    if caller != &admin {
        return Err(ContractError::Unauthorized);
    }
    Ok(())
}

pub fn require_not_paused(env: &Env) -> Result<(), ContractError> {
    if is_paused(env) {
        Err(ContractError::RouterPaused)
    } else {
        Ok(())
    }
}
