// `ai-engine treasury <subcommand>` — deposit, allocate, and inspect treasury budgets.
import { TreasuryClient, type AgentConfig, type AllocationPolicy } from '@ai-engine/sdk';

function loadConfig(): AgentConfig {
  return {
    agentId: process.env.AI_ENGINE_AGENT_ID ?? 'cli-agent',
    networkPassphrase: process.env.AI_ENGINE_NETWORK_PASSPHRASE ?? 'Test SDF Network ; September 2015',
    rpcUrl: process.env.AI_ENGINE_RPC_URL ?? 'https://soroban-testnet.stellar.org',
    treasuryContractId: process.env.AI_ENGINE_TREASURY_CONTRACT_ID ?? '',
    routerContractId: process.env.AI_ENGINE_ROUTER_CONTRACT_ID ?? '',
  };
}

export async function runTreasuryCommand(args: string[]): Promise<void> {
  const [subcommand, ...rest] = args;
  const client = new TreasuryClient(loadConfig());

  switch (subcommand) {
    case 'deposit': {
      const [admin, amount] = rest;
      requireArgs({ admin, amount }, ['admin', 'amount']);
      const balance = await client.deposit(admin, BigInt(amount));
      console.log(`New treasury balance: ${balance}`);
      return;
    }
    case 'allocate': {
      const [admin, agent, amount, expiresAt] = rest;
      requireArgs({ admin, agent, amount }, ['admin', 'agent', 'amount']);
      const allocation = await client.allocateBudget(
        admin,
        agent,
        BigInt(amount),
        Number(expiresAt ?? 0),
      );
      console.log(JSON.stringify(allocation, bigintReplacer, 2));
      return;
    }
    case 'budget': {
      const [agent] = rest;
      requireArgs({ agent }, ['agent']);
      const remaining = await client.getBudget(agent);
      console.log(`Remaining budget: ${remaining}`);
      return;
    }
    case 'set-policy': {
      const [admin, dailyCap, min, max] = rest;
      requireArgs({ admin, dailyCap, min, max }, ['admin', 'dailyCap', 'min', 'max']);
      const policy: AllocationPolicy = {
        dailyAllocationCap: BigInt(dailyCap),
        minAllocation: BigInt(min),
        maxAllocation: BigInt(max),
      };
      await client.setAllocationPolicy(admin, policy);
      console.log('Allocation policy updated.');
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

function bigintReplacer(_key: string, value: unknown): unknown {
  return typeof value === 'bigint' ? value.toString() : value;
}

function printUsage(): void {
  console.log(`Usage: ai-engine treasury <deposit|allocate|budget|set-policy> [...args]`);
}
