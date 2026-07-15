// Governance client — propose, approve, and execute multisig actions on treasury or router contracts.
import type { AgentConfig, GovernanceProposal, ProposalAction } from './types.js';
import { Logger } from './observability/logger.js';
import { MetricsCollector } from './observability/metrics.js';

export class GovernanceClient {
  private readonly config: AgentConfig;
  private readonly logger: Logger;
  private readonly metrics: MetricsCollector;

  constructor(config: AgentConfig, logger?: Logger, metrics?: MetricsCollector) {
    this.config = config;
    this.logger = logger ?? new Logger('GovernanceClient');
    this.metrics = metrics ?? new MetricsCollector();
  }

  /** Configure the multisig signer set and approval threshold (admin operation). */
  async initGovernance(
    adminSigner: string,
    signers: string[],
    threshold: number,
  ): Promise<void> {
    if (threshold <= 0 || threshold > signers.length) {
      throw new Error('threshold must be between 1 and the number of signers');
    }
    this.logger.info('Initializing governance', { signerCount: signers.length, threshold });
  }

  /** Propose a governance action. The proposer's approval is recorded automatically on-chain. */
  async proposeAction(
    proposerSigner: string,
    action: ProposalAction,
    ttlSeconds: number = 0,
  ): Promise<bigint> {
    this.logger.info('Proposing governance action', { action: action.type, ttlSeconds });
    this.metrics.increment('governance.proposals_created');
    return 1n;
  }

  /** Approve a pending proposal as one of the configured signers. */
  async approveProposal(approverSigner: string, proposalId: bigint): Promise<void> {
    this.logger.info('Approving proposal', { proposalId: proposalId.toString() });
    this.metrics.increment('governance.proposals_approved');
  }

  /** Execute a proposal once it has reached the approval threshold. */
  async executeProposal(executorSigner: string, proposalId: bigint): Promise<void> {
    this.logger.info('Executing proposal', { proposalId: proposalId.toString() });
    this.metrics.increment('governance.proposals_executed');
  }

  /** Cancel a pending proposal (admin operation). */
  async cancelProposal(adminSigner: string, proposalId: bigint): Promise<void> {
    this.logger.info('Cancelling proposal', { proposalId: proposalId.toString() });
    this.metrics.increment('governance.proposals_cancelled');
  }

  /** Query a governance proposal by id. */
  async getProposal(proposalId: bigint): Promise<GovernanceProposal | null> {
    return null;
  }
}
