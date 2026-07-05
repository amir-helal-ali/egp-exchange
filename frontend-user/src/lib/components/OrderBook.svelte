<script lang="ts">
 import { onMount, onDestroy } from 'svelte';
 import { orderbook } from '$lib/stores/exchange';
 import { api } from '$lib/api';
 import type { OrderbookSnapshot } from '$lib/api';

 export let base = 'BTC';
 export let quote = 'EGP';

 let polling: ReturnType<typeof setInterval>;

 onMount(() => {
   fetchOrderbook();
   polling = setInterval(fetchOrderbook, 1000);
 });

 onDestroy(() => {
   if (polling) clearInterval(polling);
 });

 async function fetchOrderbook() {
   try {
     const ob = await api.get<OrderbookSnapshot>(`/orderbook/${base}/${quote}`);
     orderbook.set(ob);
   } catch {}
 }

 function formatPrice(s: string) {
   return parseFloat(s).toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 });
 }

 function formatQty(s: string) {
   return parseFloat(s).toLocaleString(undefined, { minimumFractionDigits: 4, maximumFractionDigits: 8 });
 }

 $: asks = $orderbook?.asks?.slice(0, 10).reverse() ?? [];
 $: bids = $orderbook?.bids?.slice(0, 10) ?? [];
 $: lastPrice = $orderbook?.last_price;

 $: maxAskQty = Math.max(...asks.map(a => parseFloat(a.quantity)), 0.001);
 $: maxBidQty = Math.max(...bids.map(b => parseFloat(b.quantity)), 0.001);
</script>

<div class="card">
  <div class="flex items-center justify-between mb-3">
    <h3 class="text-sm font-semibold text-gray-300">Order Book</h3>
    <span class="text-xs text-gray-500">{base}/{quote}</span>
  </div>

  <div class="grid grid-cols-3 text-xs text-gray-500 mb-1 pb-1 border-b border-dark-600">
    <span>Price ({quote})</span>
    <span class="text-right">Qty ({base})</span>
    <span class="text-right">Total</span>
  </div>

  <div class="space-y-0.5">
    {#each asks as ask}
      <div class="grid grid-cols-3 text-xs relative">
        <div class="absolute right-0 top-0 h-full bg-red-900/20" style="width: {(parseFloat(ask.quantity) / maxAskQty) * 100}%"></div>
        <span class="text-accent-red z-10">{formatPrice(ask.price)}</span>
        <span class="text-right z-10">{formatQty(ask.quantity)}</span>
        <span class="text-right text-gray-500 z-10">{(parseFloat(ask.price) * parseFloat(ask.quantity)).toFixed(2)}</span>
      </div>
    {/each}
  </div>

  <div class="text-center py-2 my-1 border-y border-dark-600">
    {#if lastPrice}
      <span class="text-lg font-bold">{formatPrice(lastPrice)}</span>
    {:else}
      <span class="text-gray-500">---</span>
    {/if}
  </div>

  <div class="space-y-0.5">
    {#each bids as bid}
      <div class="grid grid-cols-3 text-xs relative">
        <div class="absolute right-0 top-0 h-full bg-green-900/20" style="width: {(parseFloat(bid.quantity) / maxBidQty) * 100}%"></div>
        <span class="text-accent-green z-10">{formatPrice(bid.price)}</span>
        <span class="text-right z-10">{formatQty(bid.quantity)}</span>
        <span class="text-right text-gray-500 z-10">{(parseFloat(bid.price) * parseFloat(bid.quantity)).toFixed(2)}</span>
      </div>
    {/each}
  </div>
</div>
