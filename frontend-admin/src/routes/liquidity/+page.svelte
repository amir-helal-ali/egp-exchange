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
    loading = true;
    try {
      data = await api.get<LiquidityData>('/admin/liquidity');
    } catch {}
    loading = false;
  }

  function formatDecimal(s: string | null) {
    if (!s) return '---';
    return parseFloat(s).toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 8 });
  }
</script>

<div class="animate-fade-in">
  <div class="flex items-center justify-between mb-6">
    <h1 class="text-2xl font-bold">مراقبة السيولة</h1>
    <button class="btn-ghost btn-sm flex items-center gap-1.5" on:click={fetchData} disabled={loading}>
      <svg class="w-4 h-4" class:animate-spin={loading} fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"/></svg>
      تحديث
    </button>
  </div>

  {#if loading && !data}
    <div class="flex justify-center py-12">
      <div class="animate-spin rounded-full h-8 w-8 border-2 border-amber-500 border-t-transparent"></div>
    </div>
  {:else}
    {#if data?.circuit_breaker}
      <div class="flex items-center gap-3 bg-red-900/40 border border-red-700 text-red-300 rounded-xl p-4 mb-6 animate-fade-in">
        <svg class="w-6 h-6 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L4.082 16.5c-.77.833.192 2.5 1.732 2.5z"/></svg>
        <div>
          <p class="font-semibold">قاطع الدائرة مفتوح</p>
          <p class="text-sm text-red-400/80">تم فصل مزود أسعار Binance. التداول موقف.</p>
        </div>
      </div>
    {:else}
      <div class="flex items-center gap-3 bg-green-900/40 border border-green-700 text-green-300 rounded-xl p-4 mb-6 animate-fade-in">
        <svg class="w-6 h-6 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"/></svg>
        <div>
          <p class="font-semibold">مزود الأسعار نشط</p>
          <p class="text-sm text-green-400/80">جميع الأنظمة تعمل بشكل طبيعي.</p>
        </div>
      </div>
    {/if}

    {#if data?.pairs?.length}
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        {#each data.pairs as pair}
          <div class="card hover:border-amber-600/30 transition-colors">
            <div class="flex items-center justify-between mb-4">
              <h3 class="text-lg font-semibold text-amber-400">{pair.pair}</h3>
              <span class="badge badge-active">نشط</span>
            </div>
            <div class="grid grid-cols-2 gap-y-4 gap-x-6 text-sm">
              <div>
                <p class="text-gray-500 text-xs mb-0.5">أفضل سعر شراء</p>
                <p class="font-mono text-base text-green-400">{formatDecimal(pair.best_bid)}</p>
              </div>
              <div>
                <p class="text-gray-500 text-xs mb-0.5">أفضل سعر بيع</p>
                <p class="font-mono text-base text-red-400">{formatDecimal(pair.best_ask)}</p>
              </div>
              <div>
                <p class="text-gray-500 text-xs mb-0.5">عمق الشراء</p>
                <p class="font-mono text-sm">{formatDecimal(pair.bid_depth)}</p>
                <p class="text-xs text-gray-500">{pair.bid_count} طلب</p>
              </div>
              <div>
                <p class="text-gray-500 text-xs mb-0.5">عمق البيع</p>
                <p class="font-mono text-sm">{formatDecimal(pair.ask_depth)}</p>
                <p class="text-xs text-gray-500">{pair.ask_count} طلب</p>
              </div>
            </div>
            <div class="mt-4 pt-3 border-t border-dark-600">
              <div class="flex gap-1.5 h-2">
                <div class="bg-green-600/60 rounded-full transition-all" style="flex: {parseFloat(pair.bid_depth) || 1}"></div>
                <div class="bg-red-600/60 rounded-full transition-all" style="flex: {parseFloat(pair.ask_depth) || 1}"></div>
              </div>
              <div class="flex justify-between text-xs text-gray-500 mt-1">
                <span>شراء</span>
                <span>بيع</span>
              </div>
            </div>
          </div>
        {/each}
      </div>
    {:else}
      <div class="card text-center py-10">
        <p class="text-gray-500">لا توجد أزواج تداول نشطة</p>
      </div>
    {/if}
  {/if}
</div>
