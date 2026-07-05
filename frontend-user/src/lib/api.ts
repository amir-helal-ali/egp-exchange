const BASE = '/api/v1';

let token: string | null = null;

export function setToken(t: string | null) {
 token = t;
 if (t) localStorage.setItem('token', t);
 else localStorage.removeItem('token');
}

export function getToken(): string | null {
 if (!token) token = localStorage.getItem('token');
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
 delete: <T>(path: string) => request<T>('DELETE', path),
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

export interface Order {
 id: string;
 user_id: string;
 side: string;
 order_type: string;
 base_currency: string;
 quote_currency: string;
 price: string | null;
 quantity: string;
 filled: string;
 status: string;
 created_at: string;
}

export interface Trade {
 id: string;
 price: string;
 quantity: string;
 total: string;
 taker_side: string;
 created_at: string;
}

export interface OrderbookLevel {
 price: string;
 quantity: string;
}

export interface OrderbookSnapshot {
 bids: OrderbookLevel[];
 asks: OrderbookLevel[];
 last_price: string | null;
 timestamp: string;
}

export interface ManualTransaction {
 id: string;
 user_id: string;
 tx_type: string;
 currency: string;
 amount: string;
 status: string;
 created_at: string;
 notes: string | null;
}
