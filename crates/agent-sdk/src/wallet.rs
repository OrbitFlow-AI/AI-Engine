// Agent wallet wrapper for programmatic signing in Rust agent systems.
use crate::types::AgentConfig;

pub struct AgentWallet {
    config: AgentConfig,
    address: Option<String>,
    session_key_expiry: u64,
}

impl AgentWallet {
    pub fn new(config: AgentConfig) -> Self {
        Self {
            config,
            address: None,
            session_key_expiry: 0,
        }
    }

    pub fn initialize(&mut self, address: String, session_ttl_seconds: u64) {
        self.address = Some(address);
        self.session_key_expiry = session_ttl_seconds;
    }

    pub fn address(&self) -> Option<&str> {
        self.address.as_deref()
    }

    pub fn is_session_valid(&self, current_timestamp: u64) -> bool {
        current_timestamp < self.session_key_expiry
    }

    pub fn config(&self) -> &AgentConfig {
        &self.config
    }

    pub fn rotate_session(&mut self, new_ttl_seconds: u64) {
        self.session_key_expiry = new_ttl_seconds;
    }
}
