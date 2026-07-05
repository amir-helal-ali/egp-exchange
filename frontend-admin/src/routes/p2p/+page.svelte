<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { api, getToken } from '$lib/api';
  import type { P2pOrder } from '$lib/api';
  import { toasts } from '$lib/stores/toast';

  let orders: P2pOrder[] = [];
  let loading = true;
  let resolving = '';

  onMount(() => {
    if (!getToken()) goto('/login');
    fetchOrders();
  });

  async function fetchOrders() {
    loading = true;
    try {
      orders = await api.get<P2pOrder[]>('/admin/p2p/orders');
    } catch (e: unknown) {
      toasts.add(e instanceof Error ? e.message : 'فشل تحميل الطلبات', 'error');
    }
    loading = false;
  }

  async function resolveOrder(id: string, action: string) {
    resolving = id;
    try {
      await api.post(`/admin/p2p/orders/${id}/resolve`, { action });
      orders = orders.map(o => o.id === id ? { ...o, status: action === 'release' ? 'completed' : 'cancelled' } : o);
      toasts.add(action === 'release' ? 'تم تحرير الدفعة للمشتري' : 'تم رد المبلغ للبائع', 'success');
    } catch (e: unknown) {
      toasts.add(e instanceof Error ? e.message : 'فشل حل النزاع', 'error');
    }
    resolving = '';
  }

  function formatAmount(s: string) {
    return parseFloat(s).toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 8 });
  }

  function formatDate(s: string) {
    return new Date(s).toLocaleString('ar-EG');
  }

  function statusBadge(status: string) {
    switch (status) {
      case 'pending': return 'badge badge-pending';
      case 'escrow': return 'badge bg-blue-900/50 text-blue-300 border border-blue-700';
      case 'disputed': return 'badge badge-disputed';
      case 'completed': return 'badge badge-approved';
      case 'cancelled': return 'badge badge-rejected';
      default: return 'badge';
    }
  }

  function statusText(status: string) {
    switch (status) {
      case 'pending': return 'معلق';
      case 'escrow': return 'ضمان';
      case 'disputed': return 'نزاع';
      case 'completed': return 'مكتمل';
      case 'cancelled': return 'ملغي';
      default: return status;
    }
  }
</script>

<div class="animate-fade-in">
  <div class="flex items-center justify-between mb-6">
    <h1 class="text-2xl font-bold">السوق P2P</h1>
    <button class="btn-ghost btn-sm flex items-center gap-1.5" on:click={fetchOrders} disabled={loading}>
      <svg class="w-4 h-4" class:animate-spin={loading} fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"/></svg>
      تحديث
    </button>
  </div>

  {#if loading && orders.length === 0}
    <div class="flex justify-center py-12">
      <div class="animate-spin rounded-full h-8 w-8 border-2 border-amber-500 border-t-transparent"></div>
    </div>
  {:else if orders.length === 0}
    <div class="card text-center py-10">
      <svg class="w-12 h-12 mx-auto text-gray-600 mb-3" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"/></svg>
      <p class="text-gray-500">لا توجد طلبات P2P</p>
    </div>
  {:else}
    <div class="card-table overflow-x-auto">
      <table class="min-w-[900px]">
        <thead>
          <tr class="border-b border-dark-600">
            <th>التاريخ</th>
            <th>المشتري</th>
            <th>البائع</th>
            <th>المبلغ</th>
            <th>الإجمالي</th>
            <th>الحالة</th>
            <th>الإجراءات</th>
          </tr>
        </thead>
        <tbody>
          {#each orders as o}
            <tr class="border-b border-dark-700 hover:bg-dark-700/50 transition-colors">
              <td class="text-xs text-gray-400">{formatDate(o.created_at)}</td>
              <td class="text-xs">{o.buyer_email}</td>
              <td class="text-xs">{o.seller_email}</td>
              <td class="font-mono text-xs">{formatAmount(o.amount)}</td>
              <td class="font-mono text-xs">{formatAmount(o.total)}</td>
              <td><span class={statusBadge(o.status)}>{statusText(o.status)}</span></td>
              <td>
                {#if o.status === 'disputed'}
                  <div class="flex gap-1.5">
                    <button
                      class="btn-success btn-xs"
                      disabled={resolving === o.id}
                      on:click={() => resolveOrder(o.id, 'release')}
                    >
                      {resolving === o.id ? '...' : 'تحرير الدفعة'}
                    </button>
                    <button
                      class="btn-danger btn-xs"
                      disabled={resolving === o.id}
                      on:click={() => resolveOrder(o.id, 'refund')}
                    >
                      {resolving === o.id ? '...' : 'رد المبلغ'}
                    </button>
                  </div>
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
