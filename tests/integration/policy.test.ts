// Integration test scaffolding — client-side spend policy and rate-limit enforcement.
import { describe, it } from 'node:test';
import assert from 'node:assert/strict';
import { PolicyManager } from '@ai-engine/sdk';
import type { AgentConfig, SpendPolicy } from '@ai-engine/sdk';

const testConfig: AgentConfig = {
  agentId: 'test-agent-001',
  networkPassphrase: 'Test SDF Network ; September 2015',
  rpcUrl: 'https://soroban-testnet.stellar.org',
  treasuryContractId: 'TREASURY_TEST_ID',
  routerContractId: 'ROUTER_TEST_ID',
};

const policy: SpendPolicy = {
  maxSinglePayment: 100_000n,
  dailyLimit: 1_000_000n,
  rateLimitWindowSeconds: 60,
  rateLimitMaxPayments: 2,
};

describe('PolicyManager rate limiting', () => {
  it('allows payments within the rate-limit window capacity', () => {
    const manager = new PolicyManager(testConfig, policy);
    const now = Date.now();
    assert.equal(manager.checkAndRecordRateLimit('agent-a', now), true);
    assert.equal(manager.checkAndRecordRateLimit('agent-a', now), true);
  });

  it('rejects payments once the window capacity is exhausted', () => {
    const manager = new PolicyManager(testConfig, policy);
    const now = Date.now();
    manager.checkAndRecordRateLimit('agent-b', now);
    manager.checkAndRecordRateLimit('agent-b', now);
    assert.equal(manager.checkAndRecordRateLimit('agent-b', now), false);
  });

  it('resets capacity once the window rolls over', () => {
    const manager = new PolicyManager(testConfig, policy);
    const start = Date.now();
    manager.checkAndRecordRateLimit('agent-c', start);
    manager.checkAndRecordRateLimit('agent-c', start);
    assert.equal(manager.checkAndRecordRateLimit('agent-c', start), false);

    const afterWindow = start + policy.rateLimitWindowSeconds! * 1000 + 1;
    assert.equal(manager.checkAndRecordRateLimit('agent-c', afterWindow), true);
  });

  it('tracks agents independently', () => {
    const manager = new PolicyManager(testConfig, policy);
    const now = Date.now();
    manager.checkAndRecordRateLimit('agent-d', now);
    manager.checkAndRecordRateLimit('agent-d', now);
    assert.equal(manager.checkAndRecordRateLimit('agent-e', now), true);
  });
});
