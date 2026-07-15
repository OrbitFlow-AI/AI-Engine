// Webhook notifier — forwards structured payment/governance events to an operator-configured endpoint.
import type { PaymentEvent } from '../types.js';
import { Logger } from './logger.js';

export interface WebhookConfig {
  url: string;
  headers?: Record<string, string>;
  maxRetries?: number;
}

export class WebhookNotifier {
  private readonly config: WebhookConfig;
  private readonly logger: Logger;

  constructor(config: WebhookConfig, logger?: Logger) {
    this.config = config;
    this.logger = logger ?? new Logger('WebhookNotifier');
  }

  /** Deliver a payment event to the configured webhook, retrying transient failures. */
  async notify(event: PaymentEvent): Promise<boolean> {
    const maxRetries = this.config.maxRetries ?? 3;

    for (let attempt = 0; attempt <= maxRetries; attempt++) {
      try {
        const response = await fetch(this.config.url, {
          method: 'POST',
          headers: {
            'content-type': 'application/json',
            ...this.config.headers,
          },
          body: JSON.stringify({
            ...event,
            amount: event.amount.toString(),
          }),
        });

        if (response.ok) {
          return true;
        }
        this.logger.warn('Webhook delivery rejected', {
          status: response.status,
          attempt,
        });
      } catch (err) {
        this.logger.warn('Webhook delivery failed', {
          attempt,
          error: err instanceof Error ? err.message : String(err),
        });
      }

      if (attempt < maxRetries) {
        await new Promise((resolve) => setTimeout(resolve, 2 ** attempt * 100));
      }
    }

    this.logger.error('Webhook delivery exhausted retries', { url: this.config.url });
    return false;
  }
}
