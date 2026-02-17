export interface HeartbeatOptions {
  interval?: number;
  timeout?: number;
  onPing?: () => void;
  onPong?: () => void;
  onTimeout?: () => void;
}

export class HeartbeatManager {
  private interval: number;
  private timeout: number;
  private pingTimer: NodeJS.Timeout | null = null;
  private pongTimer: NodeJS.Timeout | null = null;
  private lastPongTime: number = 0;
  private onPing?: () => void;
  private onPong?: () => void;
  private onTimeout?: () => void;
  private isRunning = false;

  constructor(options: HeartbeatOptions = {}) {
    this.interval = options.interval ?? 30000;
    this.timeout = options.timeout ?? 10000;
    this.onPing = options.onPing;
    this.onPong = options.onPong;
    this.onTimeout = options.onTimeout;
  }

  start(): void {
    if (this.isRunning) return;
    this.isRunning = true;
    this.schedulePing();
  }

  stop(): void {
    this.isRunning = false;
    this.clearTimers();
  }

  pongReceived(): void {
    this.lastPongTime = Date.now();
    this.clearPongTimer();
    this.onPong?.();
  }

  isAlive(): boolean {
    if (!this.isRunning) return false;
    const timeSinceLastPong = Date.now() - this.lastPongTime;
    return timeSinceLastPong < this.interval + this.timeout;
  }

  private schedulePing(): void {
    this.clearPingTimer();
    this.pingTimer = setInterval(() => {
      this.sendPing();
    }, this.interval);
  }

  private sendPing(): void {
    this.onPing?.();
    this.startPongTimeout();
  }

  private startPongTimeout(): void {
    this.clearPongTimer();
    this.pongTimer = setTimeout(() => {
      this.onTimeout?.();
    }, this.timeout);
  }

  private clearTimers(): void {
    this.clearPingTimer();
    this.clearPongTimer();
  }

  private clearPingTimer(): void {
    if (this.pingTimer) {
      clearInterval(this.pingTimer);
      this.pingTimer = null;
    }
  }

  private clearPongTimer(): void {
    if (this.pongTimer) {
      clearTimeout(this.pongTimer);
      this.pongTimer = null;
    }
  }
}

export function createHeartbeatManager(options?: HeartbeatOptions): HeartbeatManager {
  return new HeartbeatManager(options);
}
