// Integration test scaffolding — multisig governance client lifecycle.
import { describe, it } from 'node:test';
import assert from 'node:assert/strict';
import { GovernanceClient } from '@ai-engine/sdk';
import type { AgentConfig, ProposalAction } from '@ai-engine/sdk';

const testConfig: AgentConfig = {
  agentId: 'test-agent-001',
  networkPassphrase: 'Test SDF Network ; September 2015',
  rpcUrl: 'https://soroban-testnet.stellar.org',
  treasuryContractId: 'TREASURY_TEST_ID',
  routerContractId: 'ROUTER_TEST_ID',
};

describe('GovernanceClient', () => {
  it('rejects a threshold greater than the signer count', async () => {
    const client = new GovernanceClient(testConfig);
    await assert.rejects(
      () => client.initGovernance('ADMIN_ADDRESS', ['SIGNER_A'], 2),
      /threshold/,
    );
  });

  it('accepts a valid signer set and threshold', async () => {
    const client = new GovernanceClient(testConfig);
    await assert.doesNotReject(() =>
      client.initGovernance('ADMIN_ADDRESS', ['SIGNER_A', 'SIGNER_B'], 2),
    );
  });

  it('proposes a change-admin action', async () => {
    const client = new GovernanceClient(testConfig);
    const action: ProposalAction = { type: 'change_admin', newAdmin: 'NEW_ADMIN_ADDRESS' };
    const proposalId = await client.proposeAction('SIGNER_A', action);
    assert.equal(typeof proposalId, 'bigint');
  });

  it('approves and executes a proposal', async () => {
    const client = new GovernanceClient(testConfig);
    const proposalId = await client.proposeAction('SIGNER_A', { type: 'set_pause', paused: true });
    await assert.doesNotReject(() => client.approveProposal('SIGNER_B', proposalId));
    await assert.doesNotReject(() => client.executeProposal('SIGNER_A', proposalId));
  });
});
