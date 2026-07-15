// AgentClient trait — allocate, pay, and query budget lifecycle.
use std::time::{SystemTime, UNIX_EPOCH};
use ai_engine_shared::{BudgetAllocation, ContractError, PaymentRequest};
use crate::policy::RateLimiter;
use crate::types::{AgentConfig, PaymentResult, PaymentStatus, SpendPolicy};

pub trait AgentClient: Send + Sync {
    fn config(&self) -> &AgentConfig;

    fn allocate_budget(
        &self,
        agent_address: &str,
        amount: i128,
        expires_at: u64,
    ) -> Result<BudgetAllocation, ContractError>;

    fn get_remaining_budget(&self, agent_address: &str) -> Result<i128, ContractError>;

    fn initiate_payment(
        &self,
        agent_address: &str,
        request: &PaymentRequest,
    ) -> Result<PaymentResult, ContractError>;

    fn settle_payment(
        &self,
        payment_id: u64,
        received_amount: i128,
    ) -> Result<PaymentResult, ContractError>;

    fn revoke_budget(&self, agent_address: &str) -> Result<i128, ContractError>;
}

/// Mock agent client for unit tests — validates policy without on-chain calls.
pub struct MockAgentClient {
    config: AgentConfig,
    policy: SpendPolicy,
    remaining_budget: i128,
    rate_limiter: RateLimiter,
}

impl MockAgentClient {
    pub fn new(config: AgentConfig, policy: SpendPolicy) -> Self {
        let rate_limiter = RateLimiter::new(policy.clone());
        Self {
            config,
            remaining_budget: policy.daily_limit,
            policy,
            rate_limiter,
        }
    }

    pub fn set_remaining_budget(&mut self, amount: i128) {
        self.remaining_budget = amount;
    }

    /// Check whether the agent may send another payment within the configured rate-limit window,
    /// recording the attempt if allowed. Callers should invoke this before `initiate_payment`.
    pub fn check_rate_limit(&mut self, agent_address: &str) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        self.rate_limiter.check_and_record(agent_address, now)
    }
}

impl AgentClient for MockAgentClient {
    fn config(&self) -> &AgentConfig {
        &self.config
    }

    fn allocate_budget(
        &self,
        _agent_address: &str,
        amount: i128,
        _expires_at: u64,
    ) -> Result<BudgetAllocation, ContractError> {
        if amount <= 0 {
            return Err(ContractError::InvalidAmount);
        }
        // On-chain allocation requires Soroban client; mock returns NotInitialized
        Err(ContractError::NotInitialized)
    }

    fn get_remaining_budget(&self, _agent_address: &str) -> Result<i128, ContractError> {
        Ok(self.remaining_budget)
    }

    fn initiate_payment(
        &self,
        _agent_address: &str,
        request: &PaymentRequest,
    ) -> Result<PaymentResult, ContractError> {
        if request.amount <= 0 {
            return Err(ContractError::InvalidAmount);
        }
        if request.amount > self.policy.max_single_payment {
            return Err(ContractError::PaymentExceedsLimit);
        }
        if request.amount > self.remaining_budget {
            return Err(ContractError::InsufficientBudget);
        }
        Ok(PaymentResult {
            payment_id: 1,
            status: PaymentStatus::Initiated,
            tx_hash: None,
        })
    }

    fn settle_payment(
        &self,
        payment_id: u64,
        received_amount: i128,
    ) -> Result<PaymentResult, ContractError> {
        if received_amount <= 0 {
            return Err(ContractError::ConditionNotMet);
        }
        Ok(PaymentResult {
            payment_id,
            status: PaymentStatus::Settled,
            tx_hash: Some("mock_tx_hash".into()),
        })
    }

    fn revoke_budget(&self, _agent_address: &str) -> Result<i128, ContractError> {
        Ok(self.remaining_budget)
    }
}
