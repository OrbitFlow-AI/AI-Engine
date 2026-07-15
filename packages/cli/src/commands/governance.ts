// `ai-engine governance <subcommand>` — propose, approve, and execute multisig actions.
import { GovernanceClient, type AgentConfig, type ProposalAction } from '@ai-engine/sdk';

function loadConfig(): AgentConfig {
  return {
    agentId: process.env.AI_ENGINE_AGENT_ID ?? 'cli-agent',
    networkPassphrase: process.env.AI_ENGINE_NETWORK_PASSPHRASE ?? 'Test SDF Network ; September 2015',
    rpcUrl: process.env.AI_ENGINE_RPC_URL ?? 'https://soroban-testnet.stellar.org',
    treasuryContractId: process.env.AI_ENGINE_TREASURY_CONTRACT_ID ?? '',
    routerContractId: process.env.AI_ENGINE_ROUTER_CONTRACT_ID ?? '',
  };
}

function parseAction(kind: string, value: string): ProposalAction {
  switch (kind) {
    case 'change-admin':
      return { type: 'change_admin', newAdmin: value };
    case 'set-pause':
      return { type: 'set_pause', paused: value === 'true' };
    case 'add-signer':
      return { type: 'add_signer', signer: value };
    case 'remove-signer':
      return { type: 'remove_signer', signer: value };
    case 'set-threshold':
      return { type: 'set_threshold', threshold: Number(value) };
    default:
      throw new Error(`Unknown proposal action kind: ${kind}`);
  }
}

export async function runGovernanceCommand(args: string[]): Promise<void> {
  const [subcommand, ...rest] = args;
  const client = new GovernanceClient(loadConfig());

  switch (subcommand) {
    case 'init': {
      const [admin, thresholdStr, ...signers] = rest;
      requireArgs({ admin, thresholdStr }, ['admin', 'thresholdStr']);
      await client.initGovernance(admin, signers, Number(thresholdStr));
      console.log('Governance initialized.');
      return;
    }
    case 'propose': {
      const [proposer, kind, value, ttlSeconds] = rest;
      requireArgs({ proposer, kind }, ['proposer', 'kind']);
      const action = parseAction(kind, value ?? '');
      const proposalId = await client.proposeAction(proposer, action, Number(ttlSeconds ?? 0));
      console.log(`Proposal created: ${proposalId}`);
      return;
    }
    case 'approve': {
      const [approver, proposalId] = rest;
      requireArgs({ approver, proposalId }, ['approver', 'proposalId']);
      await client.approveProposal(approver, BigInt(proposalId));
      console.log('Proposal approved.');
      return;
    }
    case 'execute': {
      const [executor, proposalId] = rest;
      requireArgs({ executor, proposalId }, ['executor', 'proposalId']);
      await client.executeProposal(executor, BigInt(proposalId));
      console.log('Proposal executed.');
      return;
    }
    case 'cancel': {
      const [admin, proposalId] = rest;
      requireArgs({ admin, proposalId }, ['admin', 'proposalId']);
      await client.cancelProposal(admin, BigInt(proposalId));
      console.log('Proposal cancelled.');
      return;
    }
    default:
      printUsage();
  }
}

function requireArgs(values: Record<string, string | undefined>, names: string[]): void {
  for (const name of names) {
    if (!values[name]) {
      throw new Error(`Missing required argument: ${name}`);
    }
  }
}

function printUsage(): void {
  console.log(`Usage: ai-engine governance <init|propose|approve|execute|cancel> [...args]`);
}
