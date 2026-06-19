// Structured logging stub for SDK observability.
import type { PaymentEvent } from '../types.js';

export type LogLevel = 'debug' | 'info' | 'warn' | 'error';

export class Logger {
  private readonly namespace: string;
  private level: LogLevel;

  constructor(namespace: string, level: LogLevel = 'info') {
    this.namespace = namespace;
    this.level = level;
  }

  setLevel(level: LogLevel): void {
    this.level = level;
  }

  info(message: string, data?: Record<string, unknown>): void {
    this.log('info', message, data);
  }

  warn(message: string, data?: Record<string, unknown>): void {
    this.log('warn', message, data);
  }

  error(message: string, data?: Record<string, unknown>): void {
    this.log('error', message, data);
  }

  debug(message: string, data?: Record<string, unknown>): void {
    this.log('debug', message, data);
  }

  /** Emit structured payment event for audit ingestion. */
  emitPaymentEvent(event: PaymentEvent): void {
    this.info(`payment:${event.type}`, {
      agentId: event.agentId,
      amount: event.amount.toString(),
      timestamp: event.timestamp,
      ...event.metadata,
    });
  }

  private log(level: LogLevel, message: string, data?: Record<string, unknown>): void {
    const levels: LogLevel[] = ['debug', 'info', 'warn', 'error'];
    if (levels.indexOf(level) < levels.indexOf(this.level)) {
      return;
    }
    const entry = {
      timestamp: new Date().toISOString(),
      level,
      namespace: this.namespace,
      message,
      ...data,
    };
    console.log(JSON.stringify(entry));
  }
}
