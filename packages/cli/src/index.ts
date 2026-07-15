#!/usr/bin/env node
// AI-Engine CLI entry point — dispatches to treasury, router, and governance commands.
import { runTreasuryCommand } from './commands/treasury.js';
import { runRouterCommand } from './commands/router.js';
import { runGovernanceCommand } from './commands/governance.js';

async function main(): Promise<void> {
  const [command, ...rest] = process.argv.slice(2);

  switch (command) {
    case 'treasury':
      await runTreasuryCommand(rest);
      break;
    case 'router':
      await runRouterCommand(rest);
      break;
    case 'governance':
      await runGovernanceCommand(rest);
      break;
    case undefined:
    case '-h':
    case '--help':
      printUsage();
      break;
    default:
      console.error(`Unknown command: ${command}`);
      printUsage();
      process.exitCode = 1;
  }
}

function printUsage(): void {
  console.log(`AI-Engine CLI

Usage: ai-engine <command> <subcommand> [...args]

Commands:
  treasury    Deposit, allocate, and inspect treasury budgets
  router      Initiate, settle, and refund micropayments
  governance  Propose, approve, and execute multisig actions

Configuration is read from environment variables:
  AI_ENGINE_AGENT_ID, AI_ENGINE_NETWORK_PASSPHRASE, AI_ENGINE_RPC_URL,
  AI_ENGINE_TREASURY_CONTRACT_ID, AI_ENGINE_ROUTER_CONTRACT_ID`);
}

main().catch((err) => {
  console.error(err instanceof Error ? err.message : String(err));
  process.exitCode = 1;
});
