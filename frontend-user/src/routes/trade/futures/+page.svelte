<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { isAuthenticated, wallets, lastPrice, positions } from '$lib/stores/exchange';
  import { api, type Wallet, type Trade } from '$lib/api';
  import TradeChart from '$lib/components/TradeChart.svelte';
  import OrderBook from '$lib/components/OrderBook.svelte';
  import FuturesWidget from '$lib/components/FuturesWidget.svelte';
  import PositionsList from '$lib/components/PositionsList.svelte';
  import DepthChart from '$lib/components/DepthChart.svelte';

  let pair = 'BTC-EGP';
  $: [base, quote] = pair.split('-');

  let recentTrades: Trade[] = [];

  onMount(() => {
    if (!$isAuthenticated) goto('/login');
    api.get<Wallet[]>('/wallets').then(w => wallets.set(w)).catch(() => {});
    api.get<Trade[]>(`/trades/${base}/${quote}`).then(t => recentTrades = t).catch(() => {});
    api.get<import('$lib/api').FuturesPosition[]>('/futures/positions').then(pos => positions.set(pos)).catch(() => {});
  });

  function fmtPrice(v: string) {
    return parseFloat(v).toLocaleString('en-US', { minimumFractionDigits: 2, maximumFractionDigits: 2 });
  }

  function fmtQty(v: string) {
    return parseFloat(v).toLocaleString('en-US', { minimumFractionDigits: 4, maximumFractionDigits: 4 });
  }
</script>

<div class="max-w-7xl mx-auto p-4 space-y-4 animate-fade-in">
  <div class="flex items-center gap-3 flex-wrap">
    <h1 class="text-xl font-bold">العقود الآجلة</h1>
    <select class="input w-auto text-sm bg-dark-800 border-dark-600 rounded-lg px-3 py-1.5" bind:value={pair}>
      <option value="BTC-EGP">BTC/EGP</option>
      <option value="ETH-EGP">ETH/EGP</option>
      <option value="USDT-EGP">USDT/EGP</option>
    </select>
    {#if $lastPrice}
      <span class="text-lg font-bold" class:text-emerald-400={true}>{fmtPrice($lastPrice)} <span class="text-xs font-normal text-gray-500">{quote}</span></span>
    {/if}
  </div>

  <div class="grid grid-cols-1 lg:grid-cols-4 gap-4">
    <div class="lg:col-span-3 space-y-4">
      <TradeChart {base} {quote} />
      <DepthChart {base} {quote} />
      <div class="card">
        <h3 class="text-sm font-semibold text-gray-300 mb-3">آخر الصفقات</h3>
        <div class="grid grid-cols-3 text-xs text-gray-500 mb-2 pb-1 border-b border-dark-600">
          <span>السعر ({quote})</span>
          <span class="text-left">الكمية ({base})</span>
          <span class="text-left">الوقت</span>
        </div>
        <div class="space-y-0.5 max-h-48 overflow-y-auto">
          {#each recentTrades.slice(0, 50) as trade}
            <div class="grid grid-cols-3 text-xs py-0.5">
              <span class:text-emerald-400={trade.taker_side === 'sell'} class:text-red-400={trade.taker_side === 'buy'}>
                {fmtPrice(trade.price)}
              </span>
              <span class="text-left text-gray-300">{fmtQty(trade.quantity)}</span>
              <span class="text-left text-gray-500">{new Date(trade.created_at).toLocaleTimeString('ar-EG')}</span>
            </div>
          {:else}
            <p class="text-gray-500 text-xs text-center py-4">لا توجد صفقات بعد</p>
          {/each}
        </div>
      </div>
    </div>
    <div class="space-y-4">
      <div class="h-[420px] overflow-hidden">
        <OrderBook {base} {quote} />
      </div>
      <FuturesWidget {base} {quote} />
    </div>
  </div>

  <PositionsList />
</div>
