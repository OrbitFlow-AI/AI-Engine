// Shared multisig governance helpers used by treasury and router contracts.
use soroban_sdk::{Address, Vec};
use crate::errors::ContractError;
use crate::types::{GovernanceProposal, ProposalStatus};

/// Validate a proposed signer threshold against the current signer set size.
pub fn validate_threshold(signer_count: u32, threshold: u32) -> Result<(), ContractError> {
    if threshold == 0 || threshold > signer_count {
        return Err(ContractError::InvalidThreshold);
    }
    Ok(())
}

pub fn is_signer(signers: &Vec<Address>, addr: &Address) -> bool {
    signers.iter().any(|s| &s == addr)
}

pub fn has_approved(approvals: &Vec<Address>, addr: &Address) -> bool {
    approvals.iter().any(|a| &a == addr)
}

/// Check whether a proposal is still open (pending and not expired).
pub fn require_open(proposal: &GovernanceProposal, now: u64) -> Result<(), ContractError> {
    if proposal.status != ProposalStatus::Pending {
        return Err(ContractError::ProposalAlreadyExecuted);
    }
    if proposal.expires_at > 0 && now > proposal.expires_at {
        return Err(ContractError::ProposalExpired);
    }
    Ok(())
}

/// Whether a proposal has collected enough approvals to execute.
pub fn meets_threshold(proposal: &GovernanceProposal, threshold: u32) -> bool {
    proposal.approvals.len() >= threshold
}
