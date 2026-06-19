// Contract event definitions for treasury and router audit trail.
use soroban_sdk::{contracttype, Address, BytesN, String, Symbol, i128};
use crate::types::{AgentId, PaymentCondition};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DepositEvent {
    pub depositor: Address,
    pub amount: i128,
    pub total_balance: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AllocationEvent {
    pub agent: AgentId,
    pub amount: i128,
    pub expires_at: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PaymentInitiatedEvent {
    pub agent: AgentId,
    pub vendor: Address,
    pub amount: i128,
    pub reference: BytesN<32>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PaymentSettledEvent {
    pub agent: AgentId,
    pub vendor: Address,
    pub amount: i128,
    pub reference: BytesN<32>,
    pub memo: String,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PaymentRefundedEvent {
    pub agent: AgentId,
    pub amount: i128,
    pub reference: BytesN<32>,
    pub reason: Symbol,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BudgetRevokedEvent {
    pub agent: AgentId,
    pub reclaimed: i128,
    pub revoked_by: Address,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PathPaymentEvent {
    pub agent: AgentId,
    pub condition: PaymentCondition,
    pub path_length: u32,
}
