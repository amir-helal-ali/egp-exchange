import { writable, derived } from 'svelte/store';
import type { User, Wallet, OrderbookSnapshot } from '../api';

export const currentUser = writable<User | null>(null);
export const wallets = writable<Wallet[]>([]);
export const orderbook = writable<OrderbookSnapshot | null>(null);
export const isAuthenticated = derived(currentUser, ($u) => $u !== null);

export function logout() {
 currentUser.set(null);
 wallets.set([]);
 localStorage.removeItem('token');
}
