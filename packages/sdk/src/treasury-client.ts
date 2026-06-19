// Treasury client — deposit, allocate, and query agent budgets on-chain.
import type { AgentConfig, BudgetAllocation } from './types.js';
import { Logger } from './observability/logger.js';
import { MetricsCollector } from './observability/metrics.js';

export class TreasuryClient {
  private readonly config: AgentConfig;
  private readonly logger: Logger;
  private readonly metrics: MetricsCollector;

  constructor(config: AgentConfig, logger?: Logger, metrics?: MetricsCollector) {
    this.config = config;
    this.logger = logger ?? new Logger('TreasuryClient');
    this.metrics = metrics ?? new MetricsCollector();
  }

  /** Deposit stablecoins into treasury (admin operation). */
  async deposit(adminSigner: string, amount: bigint): Promise<bigint> {
    this.logger.info('Depositing to treasury', { amount: amount.toString() });
    this.metrics.increment('treasury.deposits');
    this.metrics.add('treasury.deposit_volume', amount);

    // Soroban contract invocation stub — wired to stellar-sdk in production
    const newBalance = amount;
    return newBalance;
  }

  /** Allocate budget to an agent address. */
  async allocateBudget(
    adminSigner: string,
    agentAddress: string,
    amount: bigint,
    expiresAt: number,
  ): Promise<BudgetAllocation> {
    this.logger.info('Allocating budget', {
      agent: agentAddress,
      amount: amount.toString(),
    });
    this.metrics.increment('treasury.allocations');

    return {
      agentId: agentAddress,
      allocated: amount,
      spent: 0n,
      expiresAt,
      remaining: amount,
    };
  }

  /** Query remaining budget for an agent. */
  async getBudget(agentAddress: string): Promise<bigint> {
    this.metrics.increment('treasury.budget_queries');
    // Contract read stub
    return 0n;
  }

  /** Revoke unspent agent budget (admin operation). */
  async revokeBudget(adminSigner: string, agentAddress: string): Promise<bigint> {
    this.logger.info('Revoking budget', { agent: agentAddress });
    this.metrics.increment('treasury.revocations');
    return 0n;
  }

  /** Get total treasury balance. */
  async totalBalance(): Promise<bigint> {
    return 0n;
  }
}
