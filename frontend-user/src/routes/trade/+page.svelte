<script lang="ts">
 import { onMount } from 'svelte';
 import { goto } from '$app/navigation';
 import { isAuthenticated, wallets } from '$lib/stores/exchange';
 import { api } from '$lib/api';
 import type { Wallet } from '$lib/api';
 import OrderBook from '$lib/components/OrderBook.svelte';
 import TradeChart from '$lib/components/TradeChart.svelte';
 import BuySellWidget from '$lib/components/BuySellWidget.svelte';

 let pair = 'BTC-EGP';
 $: [base, quote] = pair.split('-');

 onMount(() => {
   if (!$isAuthenticated) goto('/login');
   api.get<Wallet[]>('/wallets').then(w => wallets.set(w)).catch(() => {});
 });
</script>

<div class="space-y-4">
  <div class="flex items-center gap-3">
    <h1 class="text-xl font-bold">Trade</h1>
    <select class="input w-auto" bind:value={pair}>
      <option value="BTC-EGP">BTC/EGP</option>
      <option value="ETH-EGP">ETH/EGP</option>
      <option value="USDT-EGP">USDT/EGP</option>
    </select>
  </div>

  <div class="grid grid-cols-1 lg:grid-cols-3 gap-4">
    <div class="lg:col-span-2 space-y-4">
      <TradeChart {base} {quote} />
      <OrderBook {base} {quote} />
    </div>
    <div>
      <BuySellWidget {base} {quote} />
    </div>
  </div>
</div>
