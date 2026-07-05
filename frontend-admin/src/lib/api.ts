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
  const headers: Record<string, string> = {};
  const t = getToken();
  if (t) headers['Authorization'] = `Bearer ${t}`;
  if (body !== undefined) headers['Content-Type'] = 'application/json';

  const res = await fetch(`${BASE}${path}`, {
    method,
    headers,
    body: body !== undefined ? JSON.stringify(body) : undefined,
  });

  if (!res.ok) {
    const err = await res.json().catch(() => ({ error: 'Request failed' }));
    throw new Error(err.error || err.message || 'Request failed');
  }

  if (res.status === 204) return undefined as T;
  return res.json();
}

export const api = {
  get: <T>(path: string) => request<T>('GET', path),
  post: <T>(path: string, body?: unknown) => request<T>('POST', path, body),
  put: <T>(path: string, body?: unknown) => request<T>('PUT', path, body),
  del: <T>(path: string) => request<T>('DELETE', path),
};

export interface User {
  id: string;
  email: string;
  role: string;
  created_at?: string;
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

export interface Currency {
  code: string;
  name: string;
  name_ar: string;
  type: string;
  decimals: number;
  withdrawal_fee: string;
  min_withdrawal: string;
  deposit_enabled: boolean;
  withdrawal_enabled: boolean;
}

export interface TradingPair {
  id: number;
  base_currency: string;
  quote_currency: string;
  maker_fee: string;
  taker_fee: string;
  max_leverage: number;
  futures_enabled: boolean;
  spot_enabled: boolean;
  is_active: boolean;
}

export interface FuturesPosition {
  id: string;
  user_id: string;
  user_email: string;
  pair: string;
  side: string;
  quantity: string;
  entry_price: string;
  mark_price: string;
  liquidation_price: string;
  leverage: number;
  margin: string;
  pnl: string;
  status: string;
}

export interface P2pOrder {
  id: string;
  offer_id: string;
  buyer_email: string;
  seller_email: string;
  amount: string;
  total: string;
  status: string;
  created_at: string;
}

export interface SystemSetting {
  key: string;
  value: string;
  description?: string;
}

export interface LoginResponse {
  token: string;
  user: User;
}
