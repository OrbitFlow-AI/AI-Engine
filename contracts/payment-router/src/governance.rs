// Multisig governance — propose, approve, and execute admin actions on the router.
use soroban_sdk::{Address, Env, Symbol, Vec};
use ai_engine_shared::{
    self as shared, ContractError, GovernanceProposal, ProposalAction, ProposalApprovedEvent,
    ProposalCancelledEvent, ProposalCreatedEvent, ProposalExecutedEvent, ProposalStatus,
};
use crate::storage;

pub fn init_governance(
    env: &Env,
    admin: &Address,
    signers: Vec<Address>,
    threshold: u32,
) -> Result<(), ContractError> {
    storage::require_admin(env, admin)?;
    shared::validate_threshold(signers.len(), threshold)?;
    storage::set_signers(env, &signers);
    storage::set_threshold(env, threshold);
    Ok(())
}

pub fn propose_action(
    env: &Env,
    proposer: &Address,
    action: ProposalAction,
    ttl_seconds: u64,
) -> Result<u64, ContractError> {
    proposer.require_auth();
    let signers = storage::get_signers(env);
    if !shared::is_signer(&signers, proposer) {
        return Err(ContractError::NotASigner);
    }

    let now = env.ledger().timestamp();
    let id = storage::next_proposal_id(env);
    let mut approvals = Vec::new(env);
    approvals.push_back(proposer.clone());

    let proposal = GovernanceProposal {
        id,
        proposer: proposer.clone(),
        action: action.clone(),
        approvals,
        status: ProposalStatus::Pending,
        created_at: now,
        expires_at: if ttl_seconds == 0 { 0 } else { now + ttl_seconds },
    };
    storage::set_proposal(env, &proposal);

    env.events().publish(
        (Symbol::new(env, "proposal_created"), proposer.clone()),
        ProposalCreatedEvent {
            proposal_id: id,
            proposer: proposer.clone(),
            action,
        },
    );

    Ok(id)
}

pub fn approve_proposal(env: &Env, approver: &Address, proposal_id: u64) -> Result<(), ContractError> {
    approver.require_auth();
    let signers = storage::get_signers(env);
    if !shared::is_signer(&signers, approver) {
        return Err(ContractError::NotASigner);
    }

    let mut proposal = storage::get_proposal(env, proposal_id)
        .ok_or(ContractError::ProposalNotFound)?;
    shared::require_open(&proposal, env.ledger().timestamp())?;

    if shared::has_approved(&proposal.approvals, approver) {
        return Err(ContractError::AlreadyApproved);
    }

    proposal.approvals.push_back(approver.clone());
    let approval_count = proposal.approvals.len();
    storage::set_proposal(env, &proposal);

    env.events().publish(
        (Symbol::new(env, "proposal_approved"), approver.clone()),
        ProposalApprovedEvent {
            proposal_id,
            approver: approver.clone(),
            approval_count,
        },
    );

    Ok(())
}

pub fn execute_proposal(
    env: &Env,
    executor: &Address,
    proposal_id: u64,
) -> Result<(), ContractError> {
    executor.require_auth();
    let mut proposal = storage::get_proposal(env, proposal_id)
        .ok_or(ContractError::ProposalNotFound)?;
    shared::require_open(&proposal, env.ledger().timestamp())?;

    let threshold = storage::get_threshold(env);
    if !shared::meets_threshold(&proposal, threshold) {
        return Err(ContractError::InsufficientApprovals);
    }

    apply_action(env, &proposal.action)?;

    proposal.status = ProposalStatus::Executed;
    storage::set_proposal(env, &proposal);

    env.events().publish(
        (Symbol::new(env, "proposal_executed"), executor.clone()),
        ProposalExecutedEvent {
            proposal_id,
            executed_by: executor.clone(),
        },
    );

    Ok(())
}

pub fn cancel_proposal(
    env: &Env,
    admin: &Address,
    proposal_id: u64,
) -> Result<(), ContractError> {
    storage::require_admin(env, admin)?;
    let mut proposal = storage::get_proposal(env, proposal_id)
        .ok_or(ContractError::ProposalNotFound)?;
    shared::require_open(&proposal, env.ledger().timestamp())?;

    proposal.status = ProposalStatus::Cancelled;
    storage::set_proposal(env, &proposal);

    env.events().publish(
        (Symbol::new(env, "proposal_cancelled"), admin.clone()),
        ProposalCancelledEvent {
            proposal_id,
            cancelled_by: admin.clone(),
        },
    );

    Ok(())
}

fn apply_action(env: &Env, action: &ProposalAction) -> Result<(), ContractError> {
    match action {
        ProposalAction::ChangeAdmin(new_admin) => {
            storage::set_admin(env, new_admin);
            Ok(())
        }
        ProposalAction::SetPause(paused) => {
            storage::set_paused(env, *paused);
            Ok(())
        }
        ProposalAction::AddSigner(signer) => {
            let mut signers = storage::get_signers(env);
            if !shared::is_signer(&signers, signer) {
                signers.push_back(signer.clone());
            }
            storage::set_signers(env, &signers);
            Ok(())
        }
        ProposalAction::RemoveSigner(signer) => {
            let signers = storage::get_signers(env);
            let mut remaining = Vec::new(env);
            for s in signers.iter() {
                if &s != signer {
                    remaining.push_back(s);
                }
            }
            let threshold = storage::get_threshold(env);
            shared::validate_threshold(remaining.len(), threshold)?;
            storage::set_signers(env, &remaining);
            Ok(())
        }
        ProposalAction::SetThreshold(new_threshold) => {
            let signers = storage::get_signers(env);
            shared::validate_threshold(signers.len(), *new_threshold)?;
            storage::set_threshold(env, *new_threshold);
            Ok(())
        }
    }
}
