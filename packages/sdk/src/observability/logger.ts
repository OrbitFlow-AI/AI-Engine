// Structured logging stub for SDK observability.
import type { PaymentEvent } from '../types.js';

export type LogLevel = 'debug' | 'info' | 'warn' | 'error';

export interface LogEntry {
  timestamp: string;
  level: LogLevel;
  namespace: string;
  message: string;
  [key: string]: unknown;
}

export type LogTransport = (entry: LogEntry) => void;

/** Default transport — writes structured JSON to the console. */
export const consoleTransport: LogTransport = (entry) => {
  console.log(JSON.stringify(entry));
};

export class Logger {
  private readonly namespace: string;
  private level: LogLevel;
  private transports: LogTransport[];

  constructor(namespace: string, level: LogLevel = 'info', transports: LogTransport[] = [consoleTransport]) {
    this.namespace = namespace;
    this.level = level;
    this.transports = transports;
  }

  setLevel(level: LogLevel): void {
    this.level = level;
  }

  /** Register an additional transport (e.g. a webhook or file sink) without replacing existing ones. */
  addTransport(transport: LogTransport): void {
    this.transports.push(transport);
  }

  /** Replace all transports outright. */
  setTransports(transports: LogTransport[]): void {
    this.transports = transports;
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
    const entry: LogEntry = {
      timestamp: new Date().toISOString(),
      level,
      namespace: this.namespace,
      message,
      ...data,
    };
    for (const transport of this.transports) {
      transport(entry);
    }
  }
}
