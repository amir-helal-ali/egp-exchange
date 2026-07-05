import { writable, derived } from 'svelte/store';
import type { User, Wallet, OrderbookSnapshot, FuturesPosition, P2pOffer, P2pOrder } from '$lib/api';

export const currentUser = writable<User | null>(null);
export const wallets = writable<Wallet[]>([]);
export const orderbook = writable<OrderbookSnapshot | null>(null);
export const lastPrice = writable<string | null>(null);
export const positions = writable<FuturesPosition[]>([]);
export const p2pOffers = writable<P2pOffer[]>([]);
export const p2pOrders = writable<P2pOrder[]>([]);
export const wsConnected = writable<boolean>(false);

export const isAuthenticated = derived(currentUser, ($u) => $u !== null);

export function logout() {
  currentUser.set(null);
  wallets.set([]);
  orderbook.set(null);
  lastPrice.set(null);
  positions.set([]);
  p2pOffers.set([]);
  p2pOrders.set([]);
  localStorage.removeItem('token');
}
