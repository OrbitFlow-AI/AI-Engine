// Treasury contract unit tests — initialization and deposit flows.
#[cfg(test)]
mod treasury_tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Address, Env};
    use ai_engine_treasury::TreasuryContract;
    use ai_engine_treasury::TreasuryContractClient;

    #[test]
    fn test_initialize_treasury() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let token = Address::generate(&env);
        let contract_id = env.register(TreasuryContract, ());
        let client = TreasuryContractClient::new(&env, &contract_id);

        client.initialize(&admin, &token);
        assert_eq!(client.total_balance(), 0);
    }

    #[test]
    fn test_deposit_increases_balance() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let token = Address::generate(&env);
        let contract_id = env.register(TreasuryContract, ());
        let client = TreasuryContractClient::new(&env, &contract_id);

        client.initialize(&admin, &token);
        let new_balance = client.deposit(&admin, &1_000_000i128);
        assert_eq!(new_balance, 1_000_000);
    }

    #[test]
    fn test_set_and_get_allocation_policy() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let token = Address::generate(&env);
        let contract_id = env.register(TreasuryContract, ());
        let client = TreasuryContractClient::new(&env, &contract_id);
        client.initialize(&admin, &token);

        let new_policy = ai_engine_shared::AllocationPolicy {
            daily_allocation_cap: 500_000,
            min_allocation: 100,
            max_allocation: 200_000,
        };
        client.set_allocation_policy(&admin, &new_policy);

        let stored = client.get_allocation_policy();
        assert_eq!(stored.daily_allocation_cap, 500_000);
        assert_eq!(stored.max_allocation, 200_000);
    }

    #[test]
    fn test_allocation_rejected_below_policy_minimum() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let token = Address::generate(&env);
        let agent = Address::generate(&env);
        let contract_id = env.register(TreasuryContract, ());
        let client = TreasuryContractClient::new(&env, &contract_id);
        client.initialize(&admin, &token);
        client.deposit(&admin, &1_000_000i128);

        let new_policy = ai_engine_shared::AllocationPolicy {
            daily_allocation_cap: 500_000,
            min_allocation: 1_000,
            max_allocation: 200_000,
        };
        client.set_allocation_policy(&admin, &new_policy);

        let result = client.try_allocate_budget(&admin, &agent, &500i128, &0u64);
        assert!(result.is_err());
    }

    #[test]
    fn test_allocation_rejected_over_daily_cap() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let token = Address::generate(&env);
        let agent_a = Address::generate(&env);
        let agent_b = Address::generate(&env);
        let contract_id = env.register(TreasuryContract, ());
        let client = TreasuryContractClient::new(&env, &contract_id);
        client.initialize(&admin, &token);
        client.deposit(&admin, &1_000_000i128);

        let new_policy = ai_engine_shared::AllocationPolicy {
            daily_allocation_cap: 1_500,
            min_allocation: 1,
            max_allocation: 1_000,
        };
        client.set_allocation_policy(&admin, &new_policy);

        client.allocate_budget(&admin, &agent_a, &1_000i128, &0u64);
        let result = client.try_allocate_budget(&admin, &agent_b, &1_000i128, &0u64);
        assert!(result.is_err());
    }

    #[test]
    fn test_governance_proposal_executes_after_threshold() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let token = Address::generate(&env);
        let signer_b = Address::generate(&env);
        let signer_c = Address::generate(&env);
        let new_admin = Address::generate(&env);

        let contract_id = env.register(TreasuryContract, ());
        let client = TreasuryContractClient::new(&env, &contract_id);
        client.initialize(&admin, &token);

        let mut signers = soroban_sdk::Vec::new(&env);
        signers.push_back(admin.clone());
        signers.push_back(signer_b.clone());
        signers.push_back(signer_c.clone());
        client.init_governance(&admin, &signers, &2u32);

        let action = ai_engine_shared::ProposalAction::ChangeAdmin(new_admin.clone());
        let proposal_id = client.propose_action(&admin, &action, &0u64);

        // Only one approval (the proposer's) so far — should not yet execute.
        let exec_early = client.try_execute_proposal(&admin, &proposal_id);
        assert!(exec_early.is_err());

        client.approve_proposal(&signer_b, &proposal_id);
        client.execute_proposal(&admin, &proposal_id);

        let proposal = client.get_proposal(&proposal_id).unwrap();
        assert_eq!(proposal.status, ai_engine_shared::ProposalStatus::Executed);
    }

    #[test]
    fn test_governance_rejects_non_signer_proposal() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let token = Address::generate(&env);
        let outsider = Address::generate(&env);

        let contract_id = env.register(TreasuryContract, ());
        let client = TreasuryContractClient::new(&env, &contract_id);
        client.initialize(&admin, &token);

        let mut signers = soroban_sdk::Vec::new(&env);
        signers.push_back(admin.clone());
        client.init_governance(&admin, &signers, &1u32);

        let action = ai_engine_shared::ProposalAction::SetPause(true);
        let result = client.try_propose_action(&outsider, &action, &0u64);
        assert!(result.is_err());
    }
}
