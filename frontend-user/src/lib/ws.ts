import { getToken } from '$lib/api';

type WsCallback = (data: unknown) => void;

export class WsClient {
  private ws: WebSocket | null = null;
  private url: string;
  private callbacks: Map<string, WsCallback[]> = new Map();
  private subscribedChannels: Set<string> = new Set();
  private reconnectTimer: ReturnType<typeof setTimeout> | null = null;
  private reconnectAttempts = 0;
  private maxReconnectAttempts = 20;
  private baseDelay = 1000;
  private onConnectCallbacks: (() => void)[] = [];
  private onDisconnectCallbacks: (() => void)[] = [];
  private destroyed = false;

  constructor() {
    const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
    this.url = `${protocol}//${window.location.host}/ws`;
  }

  connect() {
    if (this.destroyed) return;
    const t = getToken();
    if (!t) {
      setTimeout(() => this.connect(), 1000);
      return;
    }
    try {
      this.ws = new WebSocket(`${this.url}?token=${t}`);
    } catch {
      this.scheduleReconnect();
      return;
    }

    this.ws.onopen = () => {
      this.reconnectAttempts = 0;
      for (const ch of this.subscribedChannels) {
        this.send({ type: 'subscribe', channel: ch });
      }
      this.onConnectCallbacks.forEach(cb => cb());
    };

    this.ws.onmessage = (event) => {
      try {
        const msg = JSON.parse(event.data);
        if (msg.channel && this.callbacks.has(msg.channel)) {
          this.callbacks.get(msg.channel)!.forEach(cb => cb(msg.data));
        }
        if (msg.channel) {
          const cbs = this.callbacks.get(msg.channel);
          if (cbs) cbs.forEach(cb => cb(msg.data));
        }
      } catch {}
    };

    this.ws.onclose = () => {
      this.onDisconnectCallbacks.forEach(cb => cb());
      this.scheduleReconnect();
    };

    this.ws.onerror = () => {
      this.ws?.close();
    };
  }

  private scheduleReconnect() {
    if (this.destroyed || this.reconnectAttempts >= this.maxReconnectAttempts) return;
    const delay = Math.min(this.baseDelay * Math.pow(2, this.reconnectAttempts), 30000);
    this.reconnectAttempts++;
    this.reconnectTimer = setTimeout(() => this.connect(), delay);
  }

  private send(data: unknown) {
    if (this.ws?.readyState === WebSocket.OPEN) {
      this.ws.send(JSON.stringify(data));
    }
  }

  subscribe(channel: string, callback: WsCallback) {
    if (!this.callbacks.has(channel)) {
      this.callbacks.set(channel, []);
    }
    this.callbacks.get(channel)!.push(callback);
    this.subscribedChannels.add(channel);
    if (this.ws?.readyState === WebSocket.OPEN) {
      this.send({ type: 'subscribe', channel });
    }
  }

  unsubscribe(channel: string, callback?: WsCallback) {
    if (callback && this.callbacks.has(channel)) {
      const cbs = this.callbacks.get(channel)!;
      const idx = cbs.indexOf(callback);
      if (idx >= 0) cbs.splice(idx, 1);
      if (cbs.length === 0) this.callbacks.delete(channel);
    } else {
      this.callbacks.delete(channel);
    }
    this.subscribedChannels.delete(channel);
    if (this.ws?.readyState === WebSocket.OPEN) {
      this.send({ type: 'unsubscribe', channel });
    }
  }

  onConnect(cb: () => void) {
    this.onConnectCallbacks.push(cb);
  }

  onDisconnect(cb: () => void) {
    this.onDisconnectCallbacks.push(cb);
  }

  disconnect() {
    this.destroyed = true;
    if (this.reconnectTimer) clearTimeout(this.reconnectTimer);
    if (this.ws) {
      this.ws.onclose = null;
      this.ws.close();
      this.ws = null;
    }
    this.callbacks.clear();
    this.subscribedChannels.clear();
    this.onConnectCallbacks = [];
    this.onDisconnectCallbacks = [];
  }
}

let wsInstance: WsClient | null = null;

export function getWs(): WsClient {
  if (!wsInstance) {
    wsInstance = new WsClient();
  }
  return wsInstance;
}

export function destroyWs() {
  if (wsInstance) {
    wsInstance.disconnect();
    wsInstance = null;
  }
}
