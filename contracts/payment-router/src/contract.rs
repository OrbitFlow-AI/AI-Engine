// Payment router contract implementation — micropayment routing entry points.
use soroban_sdk::{contract, contractimpl, Address, BytesN, Env, String, Symbol, Vec, i128};
use ai_engine_shared::{
    AgentId, ContractError, GovernanceProposal, PaymentCondition, PaymentRequest, ProposalAction,
    SpendPolicy, SpendPolicyUpdatedEvent,
};
use crate::{governance, policy, routing, storage};

#[contract]
pub struct PaymentRouterContract;

#[contractimpl]
impl PaymentRouterContract {
    /// Initialize router with admin and treasury contract reference.
    pub fn initialize(
        env: Env,
        admin: Address,
        treasury: Address,
        max_single_payment: i128,
    ) -> Result<(), ContractError> {
        if storage::is_initialized(&env) {
            return Err(ContractError::AlreadyInitialized);
        }
        admin.require_auth();
        storage::set_admin(&env, &admin);
        storage::set_treasury(&env, &treasury);
        storage::set_max_single_payment(&env, max_single_payment);
        storage::set_paused(&env, false);
        Ok(())
    }

    /// Agent initiates a condition-based micropayment.
    pub fn initiate_payment(
        env: Env,
        agent: Address,
        vendor: Address,
        amount: i128,
        asset: Address,
        condition: PaymentCondition,
        reference: BytesN<32>,
    ) -> Result<u64, ContractError> {
        let request = PaymentRequest {
            agent: AgentId(agent.clone()),
            vendor,
            amount,
            asset,
            condition,
            reference,
        };
        routing::initiate_payment(&env, &agent, &request)
    }

    /// Settle a pending payment when conditions are met.
    pub fn settle_payment(
        env: Env,
        payment_id: u64,
        received_amount: i128,
    ) -> Result<(), ContractError> {
        let treasury = storage::get_treasury(&env)?;
        routing::settle_payment(&env, &treasury, payment_id, received_amount)
    }

    /// Refund a pending payment (admin-only).
    pub fn refund_payment(
        env: Env,
        admin: Address,
        payment_id: u64,
        reason: Symbol,
    ) -> Result<(), ContractError> {
        routing::refund_payment(&env, &admin, payment_id, reason)
    }

    /// Set vendor allowlist (admin-only). Empty list allows all vendors.
    pub fn set_vendor_allowlist(
        env: Env,
        admin: Address,
        vendors: Vec<Address>,
    ) -> Result<(), ContractError> {
        storage::require_admin(&env, &admin)?;
        storage::set_vendor_allowlist(&env, vendors);
        Ok(())
    }

    /// Emergency pause (admin-only).
    pub fn pause(env: Env, admin: Address) -> Result<(), ContractError> {
        storage::require_admin(&env, &admin)?;
        storage::set_paused(&env, true);
        Ok(())
    }

    /// Unpause router (admin-only).
    pub fn unpause(env: Env, admin: Address) -> Result<(), ContractError> {
        storage::require_admin(&env, &admin)?;
        storage::set_paused(&env, false);
        Ok(())
    }

    /// Update max single payment limit (admin-only).
    pub fn set_max_payment(
        env: Env,
        admin: Address,
        max: i128,
    ) -> Result<(), ContractError> {
        storage::require_admin(&env, &admin)?;
        if max <= 0 {
            return Err(ContractError::InvalidAmount);
        }
        storage::set_max_single_payment(&env, max);
        Ok(())
    }

    /// Set the full spend policy — max single payment, daily limit, and rate limit (admin-only).
    pub fn set_spend_policy(
        env: Env,
        admin: Address,
        spend_policy: SpendPolicy,
    ) -> Result<(), ContractError> {
        storage::require_admin(&env, &admin)?;
        policy::validate_policy(&spend_policy)?;
        storage::set_spend_policy(&env, &spend_policy);

        env.events().publish(
            (Symbol::new(&env, "policy_updated"), admin.clone()),
            SpendPolicyUpdatedEvent {
                updated_by: admin,
                max_single_payment: spend_policy.max_single_payment,
                daily_limit: spend_policy.daily_limit,
                rate_limit_max_payments: spend_policy.rate_limit_max_payments,
            },
        );
        Ok(())
    }

    /// Read the current spend policy.
    pub fn get_spend_policy(env: Env) -> SpendPolicy {
        storage::get_spend_policy(&env)
    }

    /// Initialize the multisig signer set and approval threshold (admin-only).
    pub fn init_governance(
        env: Env,
        admin: Address,
        signers: Vec<Address>,
        threshold: u32,
    ) -> Result<(), ContractError> {
        governance::init_governance(&env, &admin, signers, threshold)
    }

    /// Propose a governance action. The proposer's approval is recorded automatically.
    pub fn propose_action(
        env: Env,
        proposer: Address,
        action: ProposalAction,
        ttl_seconds: u64,
    ) -> Result<u64, ContractError> {
        governance::propose_action(&env, &proposer, action, ttl_seconds)
    }

    /// Approve a pending governance proposal as one of the configured signers.
    pub fn approve_proposal(
        env: Env,
        approver: Address,
        proposal_id: u64,
    ) -> Result<(), ContractError> {
        governance::approve_proposal(&env, &approver, proposal_id)
    }

    /// Execute a proposal once it has reached the approval threshold.
    pub fn execute_proposal(
        env: Env,
        executor: Address,
        proposal_id: u64,
    ) -> Result<(), ContractError> {
        governance::execute_proposal(&env, &executor, proposal_id)
    }

    /// Cancel a pending proposal (admin-only).
    pub fn cancel_proposal(
        env: Env,
        admin: Address,
        proposal_id: u64,
    ) -> Result<(), ContractError> {
        governance::cancel_proposal(&env, &admin, proposal_id)
    }

    /// Read a governance proposal by id.
    pub fn get_proposal(env: Env, proposal_id: u64) -> Option<GovernanceProposal> {
        storage::get_proposal(&env, proposal_id)
    }
}
