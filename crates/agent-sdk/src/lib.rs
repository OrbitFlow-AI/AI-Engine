// Rust agent SDK root — client traits and wallet interfaces for agent systems.
pub mod client;
pub mod wallet;
pub mod types;

pub use client::AgentClient;
pub use wallet::AgentWallet;
pub use types::*;
