// Shared library root — re-exports types, events, and errors for AI-Engine contracts.
#![no_std]

pub mod types;
pub mod events;
pub mod errors;

pub use types::*;
pub use events::*;
pub use errors::*;
