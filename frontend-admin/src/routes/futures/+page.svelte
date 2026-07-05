<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { api, getToken } from '$lib/api';
  import type { FuturesPosition } from '$lib/api';
  import { toasts } from '$lib/stores/toast';

  let positions: FuturesPosition[] = [];
  let loading = true;
  let closing = '';

  onMount(() => {
    if (!getToken()) goto('/login');
    fetchPositions();
  });

  async function fetchPositions() {
    loading = true;
    try {
      positions = await api.get<FuturesPosition[]>('/admin/futures/positions');
    } catch (e: unknown) {
      toasts.add(e instanceof Error ? e.message : 'فشل تحميل المراكز', 'error');
    }
    loading = false;
  }

  async function forceClose(id: string) {
    closing = id;
    try {
      await api.post(`/admin/futures/positions/${id}/close`);
      positions = positions.map(p => p.id === id ? { ...p, status: 'closed' } : p);
      toasts.add('تم إغلاق المركز بنجاح', 'success');
    } catch (e: unknown) {
      toasts.add(e instanceof Error ? e.message : 'فشل إغلاق المركز', 'error');
    }
    closing = '';
  }

  function formatAmount(s: string) {
    return parseFloat(s).toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 8 });
  }

  function statusBadge(status: string) {
    if (status === 'open') return 'badge badge-open';
    return 'badge badge-closed';
  }

  function sideBadge(side: string) {
    if (side === 'long') return 'badge bg-green-900/50 text-green-300 border border-green-700';
    return 'badge bg-red-900/50 text-red-300 border border-red-700';
  }
</script>

<div class="animate-fade-in">
  <div class="flex items-center justify-between mb-6">
    <h1 class="text-2xl font-bold">العقود الآجلة</h1>
    <button class="btn-ghost btn-sm flex items-center gap-1.5" on:click={fetchPositions} disabled={loading}>
      <svg class="w-4 h-4" class:animate-spin={loading} fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"/></svg>
      تحديث
    </button>
  </div>

  {#if loading && positions.length === 0}
    <div class="flex justify-center py-12">
      <div class="animate-spin rounded-full h-8 w-8 border-2 border-amber-500 border-t-transparent"></div>
    </div>
  {:else if positions.length === 0}
    <div class="card text-center py-10">
      <svg class="w-12 h-12 mx-auto text-gray-600 mb-3" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7h8m0 0v8m0-8l-8 8-4-4-6 6"/></svg>
      <p class="text-gray-500">لا توجد مراكز مفتوحة</p>
    </div>
  {:else}
    <div class="card-table overflow-x-auto">
      <table class="min-w-[1200px]">
        <thead>
          <tr class="border-b border-dark-600">
            <th>المستخدم</th>
            <th>الزوج</th>
            <th>الاتجاه</th>
            <th>الكمية</th>
            <th>سعر الدخول</th>
            <th>سعر السوق</th>
            <th>التصفية</th>
            <th>الرافعة</th>
            <th>الهامش</th>
            <th>الربح/الخسارة</th>
            <th>الحالة</th>
            <th>الإجراءات</th>
          </tr>
        </thead>
        <tbody>
          {#each positions as p}
            <tr class="border-b border-dark-700 hover:bg-dark-700/50 transition-colors">
              <td class="text-xs font-medium">{p.user_email}</td>
              <td class="font-mono text-amber-400">{p.pair}</td>
              <td><span class={sideBadge(p.side)}>{p.side === 'long' ? 'شراء' : 'بيع'}</span></td>
              <td class="font-mono text-xs">{formatAmount(p.quantity)}</td>
              <td class="font-mono text-xs">{formatAmount(p.entry_price)}</td>
              <td class="font-mono text-xs">{formatAmount(p.mark_price)}</td>
              <td class="font-mono text-xs text-red-400">{formatAmount(p.liquidation_price)}</td>
              <td>{p.leverage}x</td>
              <td class="font-mono text-xs">{formatAmount(p.margin)}</td>
              <td class="font-mono text-xs" class:text-green-400={parseFloat(p.pnl) >= 0} class:text-red-400={parseFloat(p.pnl) < 0}>{formatAmount(p.pnl)}</td>
              <td><span class={statusBadge(p.status)}>{p.status === 'open' ? 'مفتوح' : 'مغلق'}</span></td>
              <td>
                {#if p.status === 'open'}
                  <button
                    class="btn-danger btn-xs"
                    disabled={closing === p.id}
                    on:click={() => forceClose(p.id)}
                  >
                    {closing === p.id ? '...' : 'إغلاق قسري'}
                  </button>
                {:else}
                  <span class="text-xs text-gray-500">---</span>
                {/if}
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {/if}
</div>
