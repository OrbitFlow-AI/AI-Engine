#[cfg(test)]
mod router_tests {
    use soroban_sdk::{testutils::Address as _, Address, Env, BytesN, String};
    use ai_engine_payment_router::PaymentRouterContract;
    use ai_engine_payment_router::PaymentRouterContractClient;
    use ai_engine_shared::PaymentCondition;

    #[test]
    fn test_initialize_router() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let treasury = Address::generate(&env);
        let contract_id = env.register(PaymentRouterContract, ());
        let client = PaymentRouterContractClient::new(&env, &contract_id);

        client.initialize(&admin, &treasury, &100_000i128);
    }

    #[test]
    fn test_rejects_zero_amount_payment() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let treasury = Address::generate(&env);
        let agent = Address::generate(&env);
        let vendor = Address::generate(&env);
        let asset = Address::generate(&env);

        let contract_id = env.register(PaymentRouterContract, ());
        let client = PaymentRouterContractClient::new(&env, &contract_id);
        client.initialize(&admin, &treasury, &100_000i128);

        let condition = PaymentCondition {
            min_received: 1000,
            max_slippage_bps: 50,
            deadline_ledger: env.ledger().sequence() + 100,
            memo: String::from_str(&env, "test"),
        };

        let reference = BytesN::from_array(&env, &[0u8; 32]);
        let result = client.try_initiate_payment(
            &agent, &vendor, &0i128, &asset, &condition, &reference,
        );
        assert!(result.is_err());
    }
}
