// Agent wallet interface — programmatic signing via Smart Account Kit.
import type { AgentConfig, PaymentRequest, PaymentResult, SmartAccountConfig } from './types.js';
import { SmartAccountKitAdapter } from './smart-account-kit.js';
import { Logger } from './observability/logger.js';

export class AgentWallet {
  private readonly config: AgentConfig;
  private readonly smartAccount: SmartAccountKitAdapter;
  private readonly logger: Logger;
  private address: string | null = null;

  constructor(
    config: AgentConfig,
    smartAccountConfig: SmartAccountConfig,
    logger?: Logger,
  ) {
    this.config = config;
    this.smartAccount = new SmartAccountKitAdapter(smartAccountConfig);
    this.logger = logger ?? new Logger('AgentWallet');
  }

  /** Create or recover agent smart account via Smart Account Kit. */
  async initialize(): Promise<string> {
    this.address = await this.smartAccount.createAgentWallet(this.config.agentId);
    this.logger.info('Agent wallet initialized', { address: this.address });
    return this.address;
  }

  getAddress(): string {
    if (!this.address) {
      throw new Error('Agent wallet not initialized. Call initialize() first.');
    }
    return this.address;
  }

  /** Sign a payment transaction within delegated session key policy. */
  async signPayment(request: PaymentRequest): Promise<PaymentResult> {
    this.logger.info('Signing payment', {
      vendor: request.vendor,
      amount: request.amount.toString(),
    });

    const signed = await this.smartAccount.signTransaction({
      contractId: this.config.routerContractId,
      method: 'initiate_payment',
      args: {
        agent: this.getAddress(),
        vendor: request.vendor,
        amount: request.amount,
        asset: request.asset,
        condition: request.condition,
        reference: request.reference,
      },
    });

    return {
      paymentId: BigInt(signed.result ?? 0),
      status: 'initiated',
      txHash: signed.txHash,
    };
  }

  /** Rotate session key (e.g., on TTL expiry). */
  async rotateSessionKey(): Promise<void> {
    await this.smartAccount.rotateSessionKey(this.getAddress());
    this.logger.info('Session key rotated');
  }
}
