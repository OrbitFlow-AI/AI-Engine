// `ai-engine router <subcommand>` — initiate, settle, and refund micropayments.
import { RouterClient, ConditionBuilder, type AgentConfig } from '@ai-engine/sdk';

function loadConfig(): AgentConfig {
  return {
    agentId: process.env.AI_ENGINE_AGENT_ID ?? 'cli-agent',
    networkPassphrase: process.env.AI_ENGINE_NETWORK_PASSPHRASE ?? 'Test SDF Network ; September 2015',
    rpcUrl: process.env.AI_ENGINE_RPC_URL ?? 'https://soroban-testnet.stellar.org',
    treasuryContractId: process.env.AI_ENGINE_TREASURY_CONTRACT_ID ?? '',
    routerContractId: process.env.AI_ENGINE_ROUTER_CONTRACT_ID ?? '',
  };
}

export async function runRouterCommand(args: string[]): Promise<void> {
  const [subcommand, ...rest] = args;
  const client = new RouterClient(loadConfig());

  switch (subcommand) {
    case 'pay': {
      const [agent, vendor, amount, asset, minReceived, deadlineLedger, memo] = rest;
      requireArgs({ agent, vendor, amount, asset, minReceived, deadlineLedger }, [
        'agent',
        'vendor',
        'amount',
        'asset',
        'minReceived',
        'deadlineLedger',
      ]);
      const condition = ConditionBuilder.create()
        .withMinReceived(BigInt(minReceived))
        .withDeadline(Number(deadlineLedger))
        .withMemo(memo ?? '')
        .build();

      const result = await client.initiatePayment(agent, {
        vendor,
        amount: BigInt(amount),
        asset,
        condition,
        reference: new Uint8Array(32),
      });
      console.log(JSON.stringify(result, bigintReplacer, 2));
      return;
    }
    case 'settle': {
      const [paymentId, receivedAmount] = rest;
      requireArgs({ paymentId, receivedAmount }, ['paymentId', 'receivedAmount']);
      const result = await client.settlePayment(BigInt(paymentId), BigInt(receivedAmount));
      console.log(JSON.stringify(result, bigintReplacer, 2));
      return;
    }
    case 'refund': {
      const [admin, paymentId, reason] = rest;
      requireArgs({ admin, paymentId }, ['admin', 'paymentId']);
      const result = await client.refundPayment(admin, BigInt(paymentId), reason ?? 'refund');
      console.log(JSON.stringify(result, bigintReplacer, 2));
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
  console.log(`Usage: ai-engine router <pay|settle|refund> [...args]`);
}
