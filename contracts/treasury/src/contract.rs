// Treasury contract implementation — deposit, allocate, and balance management.
use soroban_sdk::{contract, contractimpl, Address, Env, Symbol, Vec, i128};
use ai_engine_shared::{
    AgentId, AllocationPolicy, AllocationPolicyUpdatedEvent, DepositEvent, BudgetAllocation,
    ContractError, GovernanceProposal, ProposalAction,
};
use crate::{allocation, governance, policy, storage};

#[contract]
pub struct TreasuryContract;

#[contractimpl]
impl TreasuryContract {
    /// Initialize treasury with admin and token contract address.
    pub fn initialize(env: Env, admin: Address, token: Address) -> Result<(), ContractError> {
        if storage::is_initialized(&env) {
            return Err(ContractError::AlreadyInitialized);
        }
        admin.require_auth();
        storage::set_admin(&env, &admin);
        storage::set_token(&env, &token);
        storage::set_total_balance(&env, 0);
        storage::set_paused(&env, false);
        Ok(())
    }

    /// Record a deposit into the treasury (admin-only).
    pub fn deposit(env: Env, depositor: Address, amount: i128) -> Result<i128, ContractError> {
        storage::require_admin(&env, &depositor)?;
        if amount <= 0 {
            return Err(ContractError::InvalidAmount);
        }

        let token = storage::get_token(&env)?;
        // Token transfer would invoke token contract in production
        let _ = token;

        let new_balance = storage::get_total_balance(&env) + amount;
        storage::set_total_balance(&env, new_balance);

        env.events().publish(
            (Symbol::new(&env, "deposit"), depositor.clone()),
            DepositEvent {
                depositor,
                amount,
                total_balance: new_balance,
            },
        );

        Ok(new_balance)
    }

    /// Allocate budget to an agent smart account.
    pub fn allocate_budget(
        env: Env,
        admin: Address,
        agent: Address,
        amount: i128,
        expires_at: u64,
    ) -> Result<BudgetAllocation, ContractError> {
        allocation::allocate_budget(
            &env,
            &admin,
            &AgentId(agent),
            amount,
            expires_at,
        )
    }

    /// Query remaining budget for an agent.
    pub fn get_budget(env: Env, agent: Address) -> Result<i128, ContractError> {
        allocation::get_remaining_budget(&env, &AgentId(agent))
    }

    /// Record spend against agent budget (called by payment router).
    pub fn record_spend(
        env: Env,
        router: Address,
        agent: Address,
        amount: i128,
    ) -> Result<BudgetAllocation, ContractError> {
        let admin = storage::get_admin(&env)?;
        router.require_auth();
        // Router must be authorized — in production stored as authorized router address
        let _ = admin;
        allocation::record_spend(&env, &AgentId(agent), amount)
    }

    /// Revoke unspent agent budget (admin-only).
    pub fn revoke_budget(
        env: Env,
        admin: Address,
        agent: Address,
    ) -> Result<i128, ContractError> {
        allocation::revoke_budget(&env, &admin, &AgentId(agent))
    }

    /// Get total treasury balance.
    pub fn total_balance(env: Env) -> i128 {
        storage::get_total_balance(&env)
    }

    /// Emergency pause (admin-only).
    pub fn pause(env: Env, admin: Address) -> Result<(), ContractError> {
        storage::require_admin(&env, &admin)?;
        storage::set_paused(&env, true);
        Ok(())
    }

    /// Unpause treasury (admin-only).
    pub fn unpause(env: Env, admin: Address) -> Result<(), ContractError> {
        storage::require_admin(&env, &admin)?;
        storage::set_paused(&env, false);
        Ok(())
    }

    /// Set the allocation policy — daily allocation cap and per-agent bounds (admin-only).
    pub fn set_allocation_policy(
        env: Env,
        admin: Address,
        allocation_policy: AllocationPolicy,
    ) -> Result<(), ContractError> {
        storage::require_admin(&env, &admin)?;
        policy::validate_policy(&allocation_policy)?;
        storage::set_allocation_policy(&env, &allocation_policy);

        env.events().publish(
            (Symbol::new(&env, "allocation_policy_updated"), admin.clone()),
            AllocationPolicyUpdatedEvent {
                updated_by: admin,
                daily_allocation_cap: allocation_policy.daily_allocation_cap,
                min_allocation: allocation_policy.min_allocation,
                max_allocation: allocation_policy.max_allocation,
            },
        );
        Ok(())
    }

    /// Read the current allocation policy.
    pub fn get_allocation_policy(env: Env) -> AllocationPolicy {
        storage::get_allocation_policy(&env)
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
