<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { orderbook, lastPrice } from '$lib/stores/exchange';
  import { getWs } from '$lib/ws';
  import type { OrderbookSnapshot } from '$lib/api';

  export let base = 'BTC';
  export let quote = 'EGP';

  let channel = `orderbook:${base}${quote}`;

  onMount(() => {
    const ws = getWs();
    ws.subscribe(channel, handleOrderbook);
    ws.connect();
  });

  onDestroy(() => {
    const ws = getWs();
    ws.unsubscribe(channel, handleOrderbook);
  });

  function handleOrderbook(data: unknown) {
    const ob = data as OrderbookSnapshot;
    orderbook.set(ob);
    if (ob.last_price) lastPrice.set(ob.last_price);
  }

  $: asks = ($orderbook?.asks?.slice(0, 15).reverse() ?? []).map(a => ({
    ...a,
    priceNum: parseFloat(a.price),
    qtyNum: parseFloat(a.quantity),
    total: parseFloat(a.price) * parseFloat(a.quantity),
  }));
  $: bids = ($orderbook?.bids?.slice(0, 15) ?? []).map(b => ({
    ...b,
    priceNum: parseFloat(b.price),
    qtyNum: parseFloat(b.quantity),
    total: parseFloat(b.price) * parseFloat(b.quantity),
  }));

  $: maxAskQty = Math.max(...asks.map(a => a.qtyNum), 0.001);
  $: maxBidQty = Math.max(...bids.map(b => b.qtyNum), 0.001);
  $: bestAsk = asks.length > 0 ? asks[asks.length - 1]?.priceNum : 0;
  $: bestBid = bids.length > 0 ? bids[0]?.priceNum : 0;
  $: spread = bestAsk && bestBid ? bestAsk - bestBid : 0;
  $: spreadPct = bestAsk && bestBid && bestAsk !== 0 ? (spread / bestAsk) * 100 : 0;

  function fmtPrice(v: number) {
    return v.toLocaleString('en-US', { minimumFractionDigits: 2, maximumFractionDigits: 2 });
  }

  function fmtQty(v: number) {
    return v.toLocaleString('en-US', { minimumFractionDigits: 4, maximumFractionDigits: 4 });
  }

  function fmtTotal(v: number) {
    return v.toLocaleString('en-US', { minimumFractionDigits: 2, maximumFractionDigits: 2 });
  }

  $: bidTotal = bids.reduce((s, b) => s + b.total, 0);
  $: askTotal = asks.reduce((s, a) => s + a.total, 0);
</script>

<div class="card h-full flex flex-col">
  <div class="flex items-center justify-between mb-3">
    <h3 class="text-sm font-semibold text-gray-300">دفتر الأوامر</h3>
    <span class="text-xs text-gray-500">{base}/{quote}</span>
  </div>

  <div class="grid grid-cols-3 text-xs text-gray-500 mb-1 pb-1 border-b border-dark-600">
    <span>السعر ({quote})</span>
    <span class="text-left">الكمية ({base})</span>
    <span class="text-left">الإجمالي</span>
  </div>

  <div class="flex-1 min-h-0 overflow-y-auto">
    <div class="space-y-0.5">
      {#each asks as ask}
        <div class="grid grid-cols-3 text-xs relative py-0.5">
          <div class="depth-bar left-0 bg-red-900/20 rounded" style="width: {(ask.qtyNum / maxAskQty) * 100}%"></div>
          <span class="text-red-400 z-10">{fmtPrice(ask.priceNum)}</span>
          <span class="text-left text-gray-300 z-10">{fmtQty(ask.qtyNum)}</span>
          <span class="text-left text-gray-500 z-10">{fmtTotal(ask.total)}</span>
        </div>
      {/each}
    </div>

    <div class="flex items-center justify-between py-2 my-1 border-y border-dark-600 text-center">
      <span class="text-xs text-gray-500">السبريد: {fmtPrice(spread)} ({spreadPct.toFixed(2)}%)</span>
      {#if $lastPrice}
        <span class="text-base font-bold text-gray-100">{fmtPrice(parseFloat($lastPrice))}</span>
      {:else}
        <span class="text-gray-500">---</span>
      {/if}
    </div>

    <div class="space-y-0.5">
      {#each bids as bid}
        <div class="grid grid-cols-3 text-xs relative py-0.5">
          <div class="depth-bar left-0 bg-emerald-900/20 rounded" style="width: {(bid.qtyNum / maxBidQty) * 100}%"></div>
          <span class="text-emerald-400 z-10">{fmtPrice(bid.priceNum)}</span>
          <span class="text-left text-gray-300 z-10">{fmtQty(bid.qtyNum)}</span>
          <span class="text-left text-gray-500 z-10">{fmtTotal(bid.total)}</span>
        </div>
      {/each}
    </div>
  </div>

  <div class="grid grid-cols-2 gap-4 mt-2 pt-2 border-t border-dark-600 text-xs text-gray-400">
    <div class="text-right">الطلب: <span class="text-emerald-400">{fmtTotal(bidTotal)}</span></div>
    <div class="text-left">العرض: <span class="text-red-400">{fmtTotal(askTotal)}</span></div>
  </div>
</div>
