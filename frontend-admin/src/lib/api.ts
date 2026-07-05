const BASE = '/api/v1';

let token: string | null = null;

export function setToken(t: string | null) {
 token = t;
 if (t) localStorage.setItem('admin_token', t);
 else localStorage.removeItem('admin_token');
}

export function getToken(): string | null {
 if (!token) token = localStorage.getItem('admin_token');
 return token;
}

async function request<T>(method: string, path: string, body?: unknown): Promise<T> {
 const headers: Record<string, string> = { 'Content-Type': 'application/json' };
 const t = getToken();
 if (t) headers['Authorization'] = `Bearer ${t}`;

 const res = await fetch(`${BASE}${path}`, {
   method,
   headers,
   body: body ? JSON.stringify(body) : undefined,
 });

 if (!res.ok) {
   const err = await res.json().catch(() => ({ error: 'Request failed' }));
   throw new Error(err.error || 'Request failed');
 }

 return res.json();
}

export const api = {
 get: <T>(path: string) => request<T>('GET', path),
 post: <T>(path: string, body?: unknown) => request<T>('POST', path, body),
};

export interface User {
 id: string;
 email: string;
 role: string;
}

export interface Wallet {
 id: string;
 user_id: string;
 currency: string;
 balance: string;
 locked: string;
}

export interface QueueItem {
 tx_id: string;
 user_id: string;
 user_email: string;
 amount: string;
 currency: string;
 created_at: string;
}

export interface PendingTransactions {
 deposits: QueueItem[];
 withdrawals: QueueItem[];
}

export interface LiquidityData {
 pairs: Array<{
   pair: string;
   bid_depth: string;
   ask_depth: string;
   bid_count: number;
   ask_count: number;
   best_bid: string | null;
   best_ask: string | null;
 }>;
 circuit_breaker: boolean;
}
