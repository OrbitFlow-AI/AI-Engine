// Metrics collection stub for SDK observability.
export interface HistogramStats {
  count: number;
  sum: number;
  min: number;
  max: number;
  avg: number;
}

export class MetricsCollector {
  private readonly counters: Map<string, number> = new Map();
  private readonly gauges: Map<string, bigint> = new Map();
  private readonly histograms: Map<string, number[]> = new Map();

  increment(name: string, delta: number = 1): void {
    const current = this.counters.get(name) ?? 0;
    this.counters.set(name, current + delta);
  }

  /** Record a sample for a latency/size distribution (e.g. request duration in ms). */
  observe(name: string, value: number): void {
    const samples = this.histograms.get(name) ?? [];
    samples.push(value);
    this.histograms.set(name, samples);
  }

  /** Time an async operation and record its duration in the named histogram. */
  async time<T>(name: string, fn: () => Promise<T>): Promise<T> {
    const start = Date.now();
    try {
      return await fn();
    } finally {
      this.observe(name, Date.now() - start);
    }
  }

  getHistogramStats(name: string): HistogramStats | undefined {
    const samples = this.histograms.get(name);
    if (!samples || samples.length === 0) {
      return undefined;
    }
    const sum = samples.reduce((a, b) => a + b, 0);
    return {
      count: samples.length,
      sum,
      min: Math.min(...samples),
      max: Math.max(...samples),
      avg: sum / samples.length,
    };
  }

  add(name: string, value: bigint): void {
    const current = this.gauges.get(name) ?? 0n;
    this.gauges.set(name, current + value);
  }

  set(name: string, value: bigint): void {
    this.gauges.set(name, value);
  }

  getCounter(name: string): number {
    return this.counters.get(name) ?? 0;
  }

  getGauge(name: string): bigint {
    return this.gauges.get(name) ?? 0n;
  }

  /** Export metrics in Prometheus-compatible text format. */
  toPrometheus(): string {
    const lines: string[] = [];
    for (const [name, value] of this.counters) {
      lines.push(`# TYPE ${name} counter`);
      lines.push(`${name} ${value}`);
    }
    for (const [name, value] of this.gauges) {
      lines.push(`# TYPE ${name} gauge`);
      lines.push(`${name} ${value}`);
    }
    for (const [name] of this.histograms) {
      const stats = this.getHistogramStats(name);
      if (!stats) continue;
      lines.push(`# TYPE ${name} summary`);
      lines.push(`${name}_count ${stats.count}`);
      lines.push(`${name}_sum ${stats.sum}`);
    }
    return lines.join('\n');
  }

  reset(): void {
    this.counters.clear();
    this.gauges.clear();
    this.histograms.clear();
  }
}
