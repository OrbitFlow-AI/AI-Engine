// Shared types for the Rust agent SDK.
use ai_engine_shared::{
    AgentId, AllocationPolicy, BudgetAllocation, GovernanceProposal, PaymentCondition,
    PaymentRequest, ProposalAction,
};

pub struct AgentConfig {
    pub agent_id: String,
    pub rpc_url: String,
    pub treasury_contract: String,
    pub router_contract: String,
    pub network_passphrase: String,
}

pub struct PaymentResult {
    pub payment_id: u64,
    pub status: PaymentStatus,
    pub tx_hash: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PaymentStatus {
    Initiated,
    Settled,
    Refunded,
    Failed,
}

#[derive(Clone)]
pub struct SpendPolicy {
    pub max_single_payment: i128,
    pub daily_limit: i128,
    pub rate_limit_window_seconds: u64,
    pub rate_limit_max_payments: u32,
}

pub type SharedBudgetAllocation = BudgetAllocation;
pub type SharedPaymentCondition = PaymentCondition;
pub type SharedPaymentRequest = PaymentRequest;
pub type SharedAgentId = AgentId;
pub type SharedAllocationPolicy = AllocationPolicy;
pub type SharedGovernanceProposal = GovernanceProposal;
pub type SharedProposalAction = ProposalAction;
