// Client-side spend policy management — validation and rate-limit tracking mirrors on-chain enforcement.
import type { AgentConfig, SpendPolicy } from './types.js';
import { Logger } from './observability/logger.js';
import { MetricsCollector } from './observability/metrics.js';

interface RateLimitWindow {
  windowStart: number;
  paymentsInWindow: number;
}

export class PolicyManager {
  private readonly config: AgentConfig;
  private readonly logger: Logger;
  private readonly metrics: MetricsCollector;
  private policy: SpendPolicy;
  private readonly windows: Map<string, RateLimitWindow> = new Map();

  constructor(
    config: AgentConfig,
    policy: SpendPolicy,
    logger?: Logger,
    metrics?: MetricsCollector,
  ) {
    this.config = config;
    this.policy = policy;
    this.logger = logger ?? new Logger('PolicyManager');
    this.metrics = metrics ?? new MetricsCollector();
  }

  getPolicy(): SpendPolicy {
    return this.policy;
  }

  updatePolicy(policy: SpendPolicy): void {
    this.logger.info('Spend policy updated', {
      maxSinglePayment: policy.maxSinglePayment.toString(),
      dailyLimit: policy.dailyLimit.toString(),
    });
    this.policy = policy;
  }

  /** Validate a payment amount and vendor against the current policy, without recording it. */
  validate(vendor: string, amount: bigint): { ok: true } | { ok: false; reason: string } {
    if (amount <= 0n) {
      return { ok: false, reason: 'amount must be positive' };
    }
    if (amount > this.policy.maxSinglePayment) {
      return { ok: false, reason: 'amount exceeds max single payment' };
    }
    if (
      this.policy.vendorAllowlist &&
      this.policy.vendorAllowlist.length > 0 &&
      !this.policy.vendorAllowlist.includes(vendor)
    ) {
      return { ok: false, reason: 'vendor not allowed' };
    }
    return { ok: true };
  }

  /**
   * Check the sliding-window rate limit for an agent and record this payment if allowed.
   * Mirrors the on-chain router's window semantics for pre-flight validation.
   */
  checkAndRecordRateLimit(agentId: string, now: number = Date.now()): boolean {
    const windowSeconds = this.policy.rateLimitWindowSeconds ?? 60;
    const maxPayments = this.policy.rateLimitMaxPayments ?? Number.MAX_SAFE_INTEGER;

    const existing = this.windows.get(agentId);
    const elapsedSeconds = existing ? (now - existing.windowStart) / 1000 : Infinity;

    const window: RateLimitWindow =
      !existing || elapsedSeconds >= windowSeconds
        ? { windowStart: now, paymentsInWindow: 0 }
        : existing;

    if (window.paymentsInWindow >= maxPayments) {
      this.windows.set(agentId, window);
      this.metrics.increment('policy.rate_limit_rejections');
      this.logger.warn('Rate limit exceeded', { agentId, maxPayments, windowSeconds });
      return false;
    }

    window.paymentsInWindow += 1;
    this.windows.set(agentId, window);
    return true;
  }

  resetRateLimit(agentId: string): void {
    this.windows.delete(agentId);
  }
}
