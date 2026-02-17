/**
 * WebSocket reconnection logic with exponential backoff
 */

export interface ReconnectOptions {
  maxAttempts: number;
  initialDelay: number;
  maxDelay: number;
  backoffMultiplier: number;
}

export const defaultReconnectOptions: ReconnectOptions = {
  maxAttempts: 10,
  initialDelay: 1000,
  maxDelay: 30000,
  backoffMultiplier: 2,
};

export class ReconnectManager {
  private attempts = 0;
  private delay: number;
  private timeoutId: NodeJS.Timeout | null = null;

  constructor(private options: ReconnectOptions = defaultReconnectOptions) {
    this.delay = options.initialDelay;
  }

  shouldReconnect(): boolean {
    return this.attempts < this.options.maxAttempts;
  }

  getNextDelay(): number {
    this.attempts++;
    const nextDelay = Math.min(
      this.delay * this.options.backoffMultiplier,
      this.options.maxDelay
    );
    this.delay = nextDelay;
    return this.delay;
  }

  reset(): void {
    this.attempts = 0;
    this.delay = this.options.initialDelay;
    if (this.timeoutId) {
      clearTimeout(this.timeoutId);
      this.timeoutId = null;
    }
  }

  scheduleReconnect(callback: () => void): void {
    const delay = this.getNextDelay();
    this.timeoutId = setTimeout(callback, delay);
  }

  getAttempts(): number {
    return this.attempts;
  }
}
