// Treasury contract entry point — budget allocation and stablecoin vault.
#![no_std]

mod contract;
mod storage;
mod allocation;
mod errors;

pub use contract::TreasuryContract;
