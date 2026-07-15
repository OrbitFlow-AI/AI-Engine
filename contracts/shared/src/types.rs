// Shared domain types for treasury and payment router contracts.
use soroban_sdk::{contracttype, Address, BytesN, String, Symbol, Vec, i128, u64};

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

/// Sliding-window rate limit policy applied per agent.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RateLimitWindow {
    pub window_seconds: u64,
    pub max_payments: u32,
    pub window_start: u64,
    pub payments_in_window: u32,
}

impl RateLimitWindow {
    /// Roll the window forward if it has elapsed, resetting the counter.
    pub fn rolled(&self, now: u64) -> RateLimitWindow {
        if now.saturating_sub(self.window_start) >= self.window_seconds {
            RateLimitWindow {
                window_seconds: self.window_seconds,
                max_payments: self.max_payments,
                window_start: now,
                payments_in_window: 0,
            }
        } else {
            self.clone()
        }
    }

    pub fn has_capacity(&self) -> bool {
        self.payments_in_window < self.max_payments
    }
}

/// Aggregate spend policy configuration for the payment router.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpendPolicy {
    pub max_single_payment: i128,
    pub daily_limit: i128,
    pub rate_limit_window_seconds: u64,
    pub rate_limit_max_payments: u32,
}

/// Per-admin allocation policy for the treasury contract.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AllocationPolicy {
    pub daily_allocation_cap: i128,
    pub min_allocation: i128,
    pub max_allocation: i128,
}

/// Lifecycle status of a governance proposal.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProposalStatus {
    Pending,
    Executed,
    Cancelled,
}

/// The action a governance proposal will perform once it reaches threshold approvals.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProposalAction {
    ChangeAdmin(Address),
    SetPause(bool),
    AddSigner(Address),
    RemoveSigner(Address),
    SetThreshold(u32),
}

/// A pending multisig governance proposal.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GovernanceProposal {
    pub id: u64,
    pub proposer: Address,
    pub action: ProposalAction,
    pub approvals: Vec<Address>,
    pub status: ProposalStatus,
    pub created_at: u64,
    pub expires_at: u64,
}
