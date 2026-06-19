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
}

export interface PaymentEvent {
  type: 'deposit' | 'allocation' | 'payment_initiated' | 'payment_settled' | 'payment_refunded';
  agentId: string;
  amount: bigint;
  timestamp: number;
  metadata?: Record<string, string>;
}
