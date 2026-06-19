// Treasury contract entry point — budget allocation and stablecoin vault.
#![no_std]

mod contract;
mod storage;
mod allocation;
mod errors;

#[cfg(test)]
mod test;

pub use contract::TreasuryContract;
