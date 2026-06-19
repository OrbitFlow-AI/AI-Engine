// Multi-agent budget manager — cohort pools and sub-allocation tracking.
import type { AgentConfig, BudgetAllocation, SpendPolicy } from './types.js';
import { TreasuryClient } from './treasury-client.js';
import { Logger } from './observability/logger.js';

export interface CohortBudget {
  cohortId: string;
  totalCap: bigint;
  allocated: bigint;
  agentCount: number;
}

export class BudgetManager {
  private readonly treasury: TreasuryClient;
  private readonly logger: Logger;
  private readonly allocations: Map<string, BudgetAllocation> = new Map();
  private readonly cohorts: Map<string, CohortBudget> = new Map();
  private readonly policy: SpendPolicy;

  constructor(
    config: AgentConfig,
    policy: SpendPolicy,
    logger?: Logger,
  ) {
    this.treasury = new TreasuryClient(config, logger);
    this.logger = logger ?? new Logger('BudgetManager');
    this.policy = policy;
  }

  /** Create a cohort budget pool with a total spending cap. */
  createCohort(cohortId: string, totalCap: bigint): CohortBudget {
    const cohort: CohortBudget = {
      cohortId,
      totalCap,
      allocated: 0n,
      agentCount: 0,
    };
    this.cohorts.set(cohortId, cohort);
    this.logger.info('Cohort created', { cohortId, totalCap: totalCap.toString() });
    return cohort;
  }

  /** Allocate budget to an agent within a cohort cap. */
  async allocateToAgent(
    adminSigner: string,
    cohortId: string,
    agentAddress: string,
    amount: bigint,
    expiresAt: number,
  ): Promise<BudgetAllocation> {
    const cohort = this.cohorts.get(cohortId);
    if (!cohort) {
      throw new Error(`Cohort not found: ${cohortId}`);
    }
    if (cohort.allocated + amount > cohort.totalCap) {
      throw new Error('Cohort cap exceeded');
    }
    if (amount > this.policy.maxSinglePayment) {
      throw new Error('Amount exceeds max single payment policy');
    }

    const allocation = await this.treasury.allocateBudget(
      adminSigner,
      agentAddress,
      amount,
      expiresAt,
    );

    cohort.allocated += amount;
    cohort.agentCount += 1;
    this.allocations.set(agentAddress, allocation);

    return allocation;
  }

  /** Get allocation status for an agent. */
  getAgentBudget(agentAddress: string): BudgetAllocation | undefined {
    return this.allocations.get(agentAddress);
  }

  /** Revoke agent budget and reclaim to cohort pool. */
  async revokeAgent(
    adminSigner: string,
    cohortId: string,
    agentAddress: string,
  ): Promise<bigint> {
    const reclaimed = await this.treasury.revokeBudget(adminSigner, agentAddress);
    const cohort = this.cohorts.get(cohortId);
    if (cohort) {
      cohort.allocated -= reclaimed;
      cohort.agentCount = Math.max(0, cohort.agentCount - 1);
    }
    this.allocations.delete(agentAddress);
    return reclaimed;
  }

  /** Check if vendor is allowed by spend policy. */
  isVendorAllowed(vendor: string): boolean {
    if (!this.policy.vendorAllowlist || this.policy.vendorAllowlist.length === 0) {
      return true;
    }
    return this.policy.vendorAllowlist.includes(vendor);
  }
}
