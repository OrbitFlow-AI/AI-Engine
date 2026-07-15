// Shared TypeScript types for AI-Engine SDK consumers.
export interface AgentConfig {
  agentId: string;
  networkPassphrase: string;
  rpcUrl: string;
  treasuryContractId: string;
  routerContractId: string;
}

export interface BudgetAllocation {
  agentId: string;
  allocated: bigint;
  spent: bigint;
  expiresAt: number;
  remaining: bigint;
}

export interface PaymentCondition {
  minReceived: bigint;
  maxSlippageBps: number;
  deadlineLedger: number;
  memo: string;
}

export interface PaymentRequest {
  vendor: string;
  amount: bigint;
  asset: string;
  condition: PaymentCondition;
  reference: Uint8Array;
}

export interface PaymentResult {
  paymentId: bigint;
  status: 'initiated' | 'settled' | 'refunded' | 'failed';
  txHash?: string;
  error?: string;
}

export interface SmartAccountConfig {
  kitUrl: string;
  passkeyCredential?: PublicKeyCredential;
  sessionKeyTtlSeconds: number;
}

export interface SpendPolicy {
  maxSinglePayment: bigint;
  dailyLimit: bigint;
  vendorAllowlist?: string[];
  rateLimitWindowSeconds?: number;
  rateLimitMaxPayments?: number;
}

export interface AllocationPolicy {
  dailyAllocationCap: bigint;
  minAllocation: bigint;
  maxAllocation: bigint;
}

export type ProposalStatus = 'pending' | 'executed' | 'cancelled';

export type ProposalAction =
  | { type: 'change_admin'; newAdmin: string }
  | { type: 'set_pause'; paused: boolean }
  | { type: 'add_signer'; signer: string }
  | { type: 'remove_signer'; signer: string }
  | { type: 'set_threshold'; threshold: number };

export interface GovernanceProposal {
  id: bigint;
  proposer: string;
  action: ProposalAction;
  approvals: string[];
  status: ProposalStatus;
  createdAt: number;
  expiresAt: number;
}

export interface PaymentEvent {
  type: 'deposit' | 'allocation' | 'payment_initiated' | 'payment_settled' | 'payment_refunded';
  agentId: string;
  amount: bigint;
  timestamp: number;
  metadata?: Record<string, string>;
}
