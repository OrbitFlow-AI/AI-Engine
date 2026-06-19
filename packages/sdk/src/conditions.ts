// Fluent builder for path-payment conditions.
import type { PaymentCondition } from './types.js';

export class ConditionBuilder {
  private minReceived: bigint = 0n;
  private maxSlippageBps: number = 50;
  private deadlineLedger: number = 0;
  private memo: string = '';

  static create(): ConditionBuilder {
    return new ConditionBuilder();
  }

  withMinReceived(amount: bigint): this {
    this.minReceived = amount;
    return this;
  }

  withMaxSlippage(bps: number): this {
    if (bps < 0 || bps > 10_000) {
      throw new Error('Slippage must be between 0 and 10000 bps');
    }
    this.maxSlippageBps = bps;
    return this;
  }

  withDeadline(ledgerSequence: number): this {
    this.deadlineLedger = ledgerSequence;
    return this;
  }

  withMemo(memo: string): this {
    this.memo = memo;
    return this;
  }

  build(): PaymentCondition {
    if (this.minReceived <= 0n) {
      throw new Error('minReceived must be positive');
    }
    if (this.deadlineLedger <= 0) {
      throw new Error('deadlineLedger must be set');
    }
    return {
      minReceived: this.minReceived,
      maxSlippageBps: this.maxSlippageBps,
      deadlineLedger: this.deadlineLedger,
      memo: this.memo,
    };
  }
}
