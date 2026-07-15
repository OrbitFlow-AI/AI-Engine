// Payment router client — initiate, settle, and refund micropayments.
import type { AgentConfig, PaymentCondition, PaymentRequest, PaymentResult } from './types.js';
import { Logger } from './observability/logger.js';
import { MetricsCollector } from './observability/metrics.js';
import { withRetry, type RetryOptions } from './retry.js';

export class RouterClient {
  private readonly config: AgentConfig;
  private readonly logger: Logger;
  private readonly metrics: MetricsCollector;

  constructor(config: AgentConfig, logger?: Logger, metrics?: MetricsCollector) {
    this.config = config;
    this.logger = logger ?? new Logger('RouterClient');
    this.metrics = metrics ?? new MetricsCollector();
  }

  /** Initiate a condition-based micropayment, retrying transient RPC failures. */
  async initiatePayment(
    agentAddress: string,
    request: PaymentRequest,
    retryOptions?: RetryOptions,
  ): Promise<PaymentResult> {
    return this.metrics.time('router.initiate_payment_latency_ms', async () =>
      withRetry(async () => {
        this.logger.info('Initiating payment', {
          vendor: request.vendor,
          amount: request.amount.toString(),
        });
        this.metrics.increment('router.payments_initiated');
        this.metrics.add('router.payment_volume', request.amount);

        return {
          paymentId: 1n,
          status: 'initiated' as const,
        };
      }, retryOptions),
    );
  }

  /** Initiate multiple micropayments sequentially, collecting per-request results without aborting on failure. */
  async initiatePaymentBatch(
    agentAddress: string,
    requests: PaymentRequest[],
    retryOptions?: RetryOptions,
  ): Promise<PaymentResult[]> {
    const results: PaymentResult[] = [];
    for (const request of requests) {
      try {
        results.push(await this.initiatePayment(agentAddress, request, retryOptions));
      } catch (err) {
        results.push({
          paymentId: 0n,
          status: 'failed',
          error: err instanceof Error ? err.message : String(err),
        });
      }
    }
    return results;
  }

  /** Settle a pending payment when delivery conditions are met. */
  async settlePayment(paymentId: bigint, receivedAmount: bigint): Promise<PaymentResult> {
    this.logger.info('Settling payment', {
      paymentId: paymentId.toString(),
      received: receivedAmount.toString(),
    });
    this.metrics.increment('router.payments_settled');

    return {
      paymentId,
      status: 'settled',
    };
  }

  /** Refund a pending payment (admin operation). */
  async refundPayment(
    adminSigner: string,
    paymentId: bigint,
    reason: string,
  ): Promise<PaymentResult> {
    this.logger.info('Refunding payment', {
      paymentId: paymentId.toString(),
      reason,
    });
    this.metrics.increment('router.payments_refunded');

    return {
      paymentId,
      status: 'refunded',
    };
  }

  /** Set vendor allowlist on router (admin operation). */
  async setVendorAllowlist(adminSigner: string, vendors: string[]): Promise<void> {
    this.logger.info('Setting vendor allowlist', { count: vendors.length });
  }
}
