// Client-side spend policy validation and rate-limit tracking for Rust agent systems.
use std::collections::HashMap;
use ai_engine_shared::RateLimitWindow;
use crate::types::SpendPolicy;

pub struct RateLimiter {
    policy: SpendPolicy,
    windows: HashMap<String, RateLimitWindow>,
}

impl RateLimiter {
    pub fn new(policy: SpendPolicy) -> Self {
        Self {
            policy,
            windows: HashMap::new(),
        }
    }

    pub fn update_policy(&mut self, policy: SpendPolicy) {
        self.policy = policy;
    }

    /// Validate an amount against the max single payment cap, independent of rate limiting.
    pub fn validate_amount(&self, amount: i128) -> bool {
        amount > 0 && amount <= self.policy.max_single_payment
    }

    /// Check the agent's sliding-window rate limit and record this payment if allowed.
    pub fn check_and_record(&mut self, agent_address: &str, now: u64) -> bool {
        let window = self
            .windows
            .get(agent_address)
            .cloned()
            .unwrap_or(RateLimitWindow {
                window_seconds: self.policy.rate_limit_window_seconds,
                max_payments: self.policy.rate_limit_max_payments,
                window_start: now,
                payments_in_window: 0,
            });

        let mut rolled = window.rolled(now);
        rolled.window_seconds = self.policy.rate_limit_window_seconds;
        rolled.max_payments = self.policy.rate_limit_max_payments;

        if !rolled.has_capacity() {
            self.windows.insert(agent_address.to_string(), rolled);
            return false;
        }

        rolled.payments_in_window += 1;
        self.windows.insert(agent_address.to_string(), rolled);
        true
    }

    pub fn reset(&mut self, agent_address: &str) {
        self.windows.remove(agent_address);
    }
}
