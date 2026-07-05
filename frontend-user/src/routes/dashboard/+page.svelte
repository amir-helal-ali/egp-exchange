<script lang="ts">
 import { onMount } from 'svelte';
 import { wallets } from '$lib/stores/exchange';
 import { api } from '$lib/api';
 import { goto } from '$app/navigation';
 import type { Order, Wallet } from '$lib/api';

 let orders: Order[] = [];
 let loading = true;

 onMount(async () => {
   try {
     const [w, o] = await Promise.all([
       api.get<Wallet[]>('/wallets'),
       api.get<Order[]>('/orders'),
     ]);
     wallets.set(w);
     orders = o;
   } catch {
     // handle error
   }
   loading = false;
 });

 function formatDecimal(s: string) {
   return parseFloat(s).toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 8 });
 }

 $: totalEgp = $wallets
   .filter((w) => w.currency === 'EGP')
   .reduce((sum, w) => sum + parseFloat(w.balance), 0);
</script>

<div class="space-y-6">
  <h1 class="text-2xl font-bold">Dashboard</h1>

  {#if loading}
    <div class="flex justify-center py-12">
      <div class="animate-spin rounded-full h-8 w-8 border-2 border-blue-500 border-t-transparent"></div>
    </div>
  {:else}
    <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
      <div class="card">
        <p class="text-gray-400 text-xs uppercase tracking-wider">EGP Balance</p>
        <p class="text-2xl font-bold text-accent-green">{totalEgp.toFixed(2)} EGP</p>
      </div>
      {#each $wallets.filter(w => w.currency !== 'EGP') as wallet}
        <div class="card">
          <p class="text-gray-400 text-xs uppercase tracking-wider">{wallet.currency}</p>
          <p class="text-2xl font-bold">{formatDecimal(wallet.balance)}</p>
          <p class="text-xs text-gray-500">Locked: {formatDecimal(wallet.locked)}</p>
        </div>
      {/each}
    </div>

    <div class="card">
      <div class="flex items-center justify-between mb-4">
        <h2 class="text-lg font-semibold">Recent Orders</h2>
        <a href="/trade" class="btn-primary text-xs">New Order</a>
      </div>
      {#if orders.length === 0}
        <p class="text-gray-500 text-sm py-4 text-center">No orders yet</p>
      {:else}
        <div class="overflow-x-auto">
          <table class="w-full text-sm">
            <thead>
              <tr class="text-gray-400 text-left border-b border-dark-600">
                <th class="pb-2 pr-4">Pair</th>
                <th class="pb-2 pr-4">Side</th>
                <th class="pb-2 pr-4">Type</th>
                <th class="pb-2 pr-4">Price</th>
                <th class="pb-2 pr-4">Qty</th>
                <th class="pb-2 pr-4">Filled</th>
                <th class="pb-2">Status</th>
              </tr>
            </thead>
            <tbody>
              {#each orders.slice(0, 10) as order}
                <tr class="border-b border-dark-700 hover:bg-dark-700/50">
                  <td class="py-2 pr-4">{order.base_currency}/{order.quote_currency}</td>
                  <td class="py-2 pr-4" class:text-accent-green={order.side === 'buy'} class:text-accent-red={order.side === 'sell'}>
                    {order.side.toUpperCase()}
                  </td>
                  <td class="py-2 pr-4">{order.order_type}</td>
                  <td class="py-2 pr-4">{order.price ? formatDecimal(order.price) : 'Market'}</td>
                  <td class="py-2 pr-4">{formatDecimal(order.quantity)}</td>
                  <td class="py-2 pr-4">{formatDecimal(order.filled)}</td>
                  <td class="py-2">
                    <span class="px-2 py-0.5 rounded text-xs" class:bg-green-900/50:text-green-300={order.status === 'filled'} class:bg-yellow-900/50:text-yellow-300={order.status === 'open' || order.status === 'partial'} class:bg-red-900/50:text-red-300={order.status === 'cancelled'}>
                      {order.status}
                    </span>
                  </td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      {/if}
    </div>
  {/if}
</div>
