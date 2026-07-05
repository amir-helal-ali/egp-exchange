<script lang="ts">
 import { onMount } from 'svelte';
 import { goto } from '$app/navigation';
 import { api, getToken } from '$lib/api';
 import type { LiquidityData } from '$lib/api';

 let data: LiquidityData | null = null;
 let loading = true;

 onMount(() => {
   if (!getToken()) goto('/login');
   fetchData();
 });

 async function fetchData() {
   try {
     data = await api.get<LiquidityData>('/admin/liquidity');
   } catch {}
   loading = false;
 }

 function formatDecimal(s: string | null) {
   if (!s) return '---';
   return parseFloat(s).toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 });
 }
</script>

<div>
  <div class="flex items-center justify-between mb-6">
    <h1 class="text-2xl font-bold">System Liquidity Monitor</h1>
    <button class="btn-ghost text-sm" on:click={fetchData}>Refresh</button>
  </div>

  {#if loading}
    <div class="flex justify-center py-12">
      <div class="animate-spin rounded-full h-8 w-8 border-2 border-blue-500 border-t-transparent"></div>
    </div>
  {:else}
    {#if data?.circuit_breaker}
      <div class="bg-red-900/50 border border-red-700 text-red-300 rounded-lg p-4 mb-6">
        <strong>Circuit Breaker OPEN</strong> — Binance price feed has been disconnected. Trading is halted.
      </div>
    {:else}
      <div class="bg-green-900/50 border border-green-700 text-green-300 rounded-lg p-4 mb-6">
        Price feed active — All systems operational
      </div>
    {/if}

    {#if data?.pairs?.length}
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        {#each data.pairs as pair}
          <div class="card">
            <h3 class="text-lg font-semibold mb-3">{pair.pair}</h3>
            <div class="grid grid-cols-2 gap-4 text-sm">
              <div>
                <p class="text-gray-400 text-xs">Best Bid</p>
                <p class="font-mono">{formatDecimal(pair.best_bid)}</p>
              </div>
              <div>
                <p class="text-gray-400 text-xs">Best Ask</p>
                <p class="font-mono">{formatDecimal(pair.best_ask)}</p>
              </div>
              <div>
                <p class="text-gray-400 text-xs">Bid Depth</p>
                <p class="font-mono text-accent-green">{formatDecimal(pair.bid_depth)}</p>
              </div>
              <div>
                <p class="text-gray-400 text-xs">Ask Depth</p>
                <p class="font-mono text-accent-red">{formatDecimal(pair.ask_depth)}</p>
              </div>
              <div>
                <p class="text-gray-400 text-xs">Bid Orders</p>
                <p class="font-mono">{pair.bid_count}</p>
              </div>
              <div>
                <p class="text-gray-400 text-xs">Ask Orders</p>
                <p class="font-mono">{pair.ask_count}</p>
              </div>
            </div>
          </div>
        {/each}
      </div>
    {:else}
      <div class="card text-center py-8">
        <p class="text-gray-500">No active trading pairs</p>
      </div>
    {/if}
  {/if}
</div>
