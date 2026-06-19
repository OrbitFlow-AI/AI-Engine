// Integration test scaffolding — treasury allocation to micropayment routing E2E.
import { describe, it } from 'node:test';
import assert from 'node:assert/strict';
import {
  AgentWallet,
  TreasuryClient,
  RouterClient,
  BudgetManager,
  ConditionBuilder,
  Logger,
  MetricsCollector,
} from '@ai-engine/sdk';
import type { AgentConfig, SmartAccountConfig, SpendPolicy } from '@ai-engine/sdk';

const testConfig: AgentConfig = {
  agentId: 'test-agent-001',
  networkPassphrase: 'Test SDF Network ; September 2015',
  rpcUrl: 'https://soroban-testnet.stellar.org',
  treasuryContractId: 'TREASURY_TEST_ID',
  routerContractId: 'ROUTER_TEST_ID',
};

const smartAccountConfig: SmartAccountConfig = {
  kitUrl: 'https://smart-account-kit.stellar.org',
  sessionKeyTtlSeconds: 86400,
};

const spendPolicy: SpendPolicy = {
  maxSinglePayment: 100_000n,
  dailyLimit: 1_000_000n,
};

describe('AI-Engine Core Loop', () => {
  it('initializes agent wallet via Smart Account Kit', async () => {
    const wallet = new AgentWallet(testConfig, smartAccountConfig);
    const address = await wallet.initialize();
    assert.ok(address.startsWith('G'));
  });

  it('allocates budget through treasury client', async () => {
    const treasury = new TreasuryClient(testConfig);
    const allocation = await treasury.allocateBudget(
      'ADMIN_ADDRESS',
      'AGENT_ADDRESS',
      500_000n,
      Date.now() + 86400000,
    );
    assert.equal(allocation.remaining, 500_000n);
  });

  it('builds path payment condition', () => {
    const condition = ConditionBuilder.create()
      .withMinReceived(99_000n)
      .withMaxSlippage(50)
      .withDeadline(1_000_000)
      .withMemo('llm-inference-payment')
      .build();

    assert.equal(condition.maxSlippageBps, 50);
    assert.equal(condition.memo, 'llm-inference-payment');
  });

  it('initiates micropayment via router client', async () => {
    const router = new RouterClient(testConfig);
    const condition = ConditionBuilder.create()
      .withMinReceived(10_000n)
      .withMaxSlippage(50)
      .withDeadline(1_000_000)
      .build();

    const result = await router.initiatePayment('AGENT_ADDRESS', {
      vendor: 'VENDOR_ADDRESS',
      amount: 10_000n,
      asset: 'USDC_ADDRESS',
      condition,
      reference: new Uint8Array(32),
    });

    assert.equal(result.status, 'initiated');
  });

  it('manages multi-agent cohort budgets', async () => {
    const manager = new BudgetManager(testConfig, spendPolicy);
    const cohort = manager.createCohort('research-agents', 5_000_000n);
    assert.equal(cohort.totalCap, 5_000_000n);
    assert.ok(manager.isVendorAllowed('any-vendor'));
  });

  it('collects observability metrics', () => {
    const metrics = new MetricsCollector();
    metrics.increment('router.payments_initiated');
    metrics.add('router.payment_volume', 10_000n);
    assert.equal(metrics.getCounter('router.payments_initiated'), 1);
    assert.ok(metrics.toPrometheus().includes('router.payments_initiated'));
  });
});
