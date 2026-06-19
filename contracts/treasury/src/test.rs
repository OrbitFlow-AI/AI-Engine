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
}
