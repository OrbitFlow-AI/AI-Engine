// Metrics collection stub for SDK observability.
export class MetricsCollector {
  private readonly counters: Map<string, number> = new Map();
  private readonly gauges: Map<string, bigint> = new Map();

  increment(name: string, delta: number = 1): void {
    const current = this.counters.get(name) ?? 0;
    this.counters.set(name, current + delta);
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
    return lines.join('\n');
  }

  reset(): void {
    this.counters.clear();
    this.gauges.clear();
  }
}
