// Payment router persistent storage and policy accessors.
use soroban_sdk::{contracttype, Address, Env, Map, Symbol, Vec};
use ai_engine_shared::{AgentId, ContractError, PaymentRequest};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Treasury,
    Paused,
    MaxSinglePayment,
    DailyLimit(AgentId),
    VendorAllowlist,
    PendingPayment(u64),
    PaymentCounter,
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

pub fn get_treasury(env: &Env) -> Result<Address, ContractError> {
    env.storage()
        .instance()
        .get(&DataKey::Treasury)
        .ok_or(ContractError::NotInitialized)
}

pub fn set_treasury(env: &Env, treasury: &Address) {
    env.storage().instance().set(&DataKey::Treasury, treasury);
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

pub fn get_max_single_payment(env: &Env) -> i128 {
    env.storage()
        .instance()
        .get(&DataKey::MaxSinglePayment)
        .unwrap_or(i128::MAX)
}

pub fn set_max_single_payment(env: &Env, max: i128) {
    env.storage().instance().set(&DataKey::MaxSinglePayment, &max);
}

pub fn get_daily_spent(env: &Env, agent: &AgentId) -> i128 {
    env.storage()
        .persistent()
        .get(&DataKey::DailyLimit(agent.clone()))
        .unwrap_or(0)
}

pub fn add_daily_spent(env: &Env, agent: &AgentId, amount: i128) {
    let current = get_daily_spent(env, agent);
    env.storage()
        .persistent()
        .set(&DataKey::DailyLimit(agent.clone()), &(current + amount));
}

pub fn is_vendor_allowed(env: &Env, vendor: &Address) -> bool {
    let allowlist: Option<Vec<Address>> = env.storage()
        .instance()
        .get(&DataKey::VendorAllowlist);

    match allowlist {
        None => true,
        Some(list) if list.is_empty() => true,
        Some(list) => list.iter().any(|v| v == vendor),
    }
}

pub fn set_vendor_allowlist(env: &Env, vendors: Vec<Address>) {
    env.storage()
        .instance()
        .set(&DataKey::VendorAllowlist, &vendors);
}

pub fn next_payment_id(env: &Env) -> u64 {
    let counter: u64 = env.storage()
        .instance()
        .get(&DataKey::PaymentCounter)
        .unwrap_or(0);
    env.storage()
        .instance()
        .set(&DataKey::PaymentCounter, &(counter + 1));
    counter
}

pub fn store_pending(env: &Env, id: u64, request: &PaymentRequest) {
    env.storage()
        .persistent()
        .set(&DataKey::PendingPayment(id), request);
}

pub fn get_pending(env: &Env, id: u64) -> Option<PaymentRequest> {
    env.storage()
        .persistent()
        .get(&DataKey::PendingPayment(id))
}

pub fn require_not_paused(env: &Env) -> Result<(), ContractError> {
    if is_paused(env) {
        Err(ContractError::RouterPaused)
    } else {
        Ok(())
    }
}

pub fn require_admin(env: &Env, caller: &Address) -> Result<(), ContractError> {
    let admin = get_admin(env)?;
    admin.require_auth();
    if caller != &admin {
        return Err(ContractError::Unauthorized);
    }
    Ok(())
}
