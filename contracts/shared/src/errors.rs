// Shared error types for AI-Engine Soroban contracts.
use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    NotInitialized = 1,
    AlreadyInitialized = 2,
    Unauthorized = 3,
    InsufficientBudget = 4,
    InsufficientTreasuryBalance = 5,
    PaymentExceedsLimit = 6,
    AgentNotFound = 7,
    BudgetExpired = 8,
    RouterPaused = 9,
    InvalidAmount = 10,
    InvalidCondition = 11,
    VendorNotAllowed = 12,
    CohortCapExceeded = 13,
    ReentrantCall = 14,
    ConditionNotMet = 15,
    PaymentTimeout = 16,
}
