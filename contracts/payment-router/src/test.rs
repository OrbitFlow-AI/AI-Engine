// Payment router unit tests — initialization and payment validation.
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

    #[test]
    fn test_set_and_get_spend_policy() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let treasury = Address::generate(&env);
        let contract_id = env.register(PaymentRouterContract, ());
        let client = PaymentRouterContractClient::new(&env, &contract_id);
        client.initialize(&admin, &treasury, &100_000i128);

        let new_policy = ai_engine_shared::SpendPolicy {
            max_single_payment: 50_000,
            daily_limit: 200_000,
            rate_limit_window_seconds: 60,
            rate_limit_max_payments: 5,
        };
        client.set_spend_policy(&admin, &new_policy);

        let stored = client.get_spend_policy();
        assert_eq!(stored.max_single_payment, 50_000);
        assert_eq!(stored.rate_limit_max_payments, 5);
    }

    #[test]
    fn test_rejects_invalid_spend_policy() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let treasury = Address::generate(&env);
        let contract_id = env.register(PaymentRouterContract, ());
        let client = PaymentRouterContractClient::new(&env, &contract_id);
        client.initialize(&admin, &treasury, &100_000i128);

        let bad_policy = ai_engine_shared::SpendPolicy {
            max_single_payment: 0,
            daily_limit: 200_000,
            rate_limit_window_seconds: 60,
            rate_limit_max_payments: 5,
        };
        let result = client.try_set_spend_policy(&admin, &bad_policy);
        assert!(result.is_err());
    }

    #[test]
    fn test_rate_limit_blocks_excess_payments() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let treasury = Address::generate(&env);
        let agent = Address::generate(&env);
        let vendor = Address::generate(&env);
        let asset = Address::generate(&env);

        let contract_id = env.register(PaymentRouterContract, ());
        let client = PaymentRouterContractClient::new(&env, &contract_id);
        client.initialize(&admin, &treasury, &1_000_000i128);

        let tight_policy = ai_engine_shared::SpendPolicy {
            max_single_payment: 1_000_000,
            daily_limit: 10_000_000,
            rate_limit_window_seconds: 3600,
            rate_limit_max_payments: 1,
        };
        client.set_spend_policy(&admin, &tight_policy);

        let condition = PaymentCondition {
            min_received: 1000,
            max_slippage_bps: 50,
            deadline_ledger: env.ledger().sequence() + 100,
            memo: String::from_str(&env, "test"),
        };
        let reference = BytesN::from_array(&env, &[0u8; 32]);

        let first = client.try_initiate_payment(
            &agent, &vendor, &1000i128, &asset, &condition, &reference,
        );
        assert!(first.is_ok());

        let second = client.try_initiate_payment(
            &agent, &vendor, &1000i128, &asset, &condition, &reference,
        );
        assert!(second.is_err());
    }
}
