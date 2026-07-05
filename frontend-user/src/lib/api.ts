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
    throw new Error(err.error || err.message || 'Request failed');
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
  stop_price: string | null;
  reduce_only: boolean;
  time_in_force: string;
  created_at: string;
}

export interface PlaceOrderRequest {
  side: string;
  order_type: string;
  base_currency: string;
  quote_currency: string;
  price?: number;
  quantity: number;
  stop_price?: number;
  reduce_only?: boolean;
  time_in_force?: string;
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

export interface FuturesPosition {
  id: string;
  user_id: string;
  pair: string;
  side: string;
  quantity: string;
  entry_price: string;
  mark_price: string;
  liquidation_price: string;
  margin: string;
  leverage: number;
  unrealized_pnl: string;
  realized_pnl: string;
  status: string;
  take_profit: string | null;
  stop_loss: string | null;
  created_at: string;
  updated_at: string;
}

export interface OpenPositionRequest {
  pair: string;
  side: string;
  quantity: number;
  leverage: number;
}

export interface SetTpSlRequest {
  take_profit?: number | null;
  stop_loss?: number | null;
}

export interface PartialCloseRequest {
  quantity: number;
}

export interface P2pOffer {
  id: string;
  user_id: string;
  type: string;
  currency: string;
  fiat_currency: string;
  price: string;
  available: string;
  min_amount: string;
  max_amount: string;
  payment_method: string;
  status: string;
  created_at: string;
}

export interface CreateOfferRequest {
  type: string;
  currency: string;
  price: number;
  available: number;
  min_amount: number;
  max_amount: number;
  payment_method: string;
}

export interface P2pOrder {
  id: string;
  offer_id: string;
  buyer_id: string;
  seller_id: string;
  amount: string;
  total: string;
  status: string;
  created_at: string;
  updated_at: string;
}

export interface AcceptOfferRequest {
  amount: number;
}

export interface P2pMessage {
  id: string;
  order_id: string;
  sender_id: string;
  message: string;
  created_at: string;
}

export interface SendMessageRequest {
  message: string;
}


