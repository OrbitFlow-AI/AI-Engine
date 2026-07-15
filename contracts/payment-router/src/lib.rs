// Payment router contract entry point — micropayment routing and path payments.
#![no_std]

mod contract;
mod storage;
mod routing;
mod conditions;
mod errors;
mod policy;

#[cfg(test)]
mod test;

pub use contract::PaymentRouterContract;
