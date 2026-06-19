// Shared domain types for treasury and payment router contracts.
use soroban_sdk::{contracttype, Address, BytesN, String, Symbol, i128, u64};

/// Unique identifier for an agent smart account.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgentId(pub Address);

/// Budget allocation record for an agent.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BudgetAllocation {
    pub agent: AgentId,
    pub allocated: i128,
    pub spent: i128,
    pub expires_at: u64,
}

impl BudgetAllocation {
    pub fn remaining(&self) -> i128 {
        self.allocated.saturating_sub(self.spent)
    }

    pub fn can_spend(&self, amount: i128) -> bool {
        amount > 0 && self.remaining() >= amount
    }
}

/// Payment condition for path-payment routing.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PaymentCondition {
    pub min_received: i128,
    pub max_slippage_bps: u32,
    pub deadline_ledger: u32,
    pub memo: String,
}

/// Micropayment request from an agent.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PaymentRequest {
    pub agent: AgentId,
    pub vendor: Address,
    pub amount: i128,
    pub asset: Address,
    pub condition: PaymentCondition,
    pub reference: BytesN<32>,
}

/// Cohort budget pool for multi-agent management.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CohortBudget {
    pub cohort_id: Symbol,
    pub total_cap: i128,
    pub allocated: i128,
    pub agent_count: u32,
}
