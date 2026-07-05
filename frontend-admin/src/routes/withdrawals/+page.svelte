<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { api, getToken } from '$lib/api';
  import type { PendingTransactions, QueueItem } from '$lib/api';
  import { toasts } from '$lib/stores/toast';

  let withdrawals: QueueItem[] = [];
  let loading = true;
  let processing = '';

  onMount(() => {
    if (!getToken()) goto('/login');
    fetchPending();
  });

  async function fetchPending() {
    loading = true;
    try {
      const data = await api.get<PendingTransactions>('/admin/transactions/pending');
      withdrawals = data.withdrawals;
    } catch (e: unknown) {
      toasts.add(e instanceof Error ? e.message : 'فشل تحميل البيانات', 'error');
    }
    loading = false;
  }

  async function processTx(id: string, action: string) {
    processing = id;
    try {
      await api.post(`/admin/transactions/${id}`, { action, notes: null });
      withdrawals = withdrawals.filter(w => w.tx_id !== id);
      toasts.add(action === 'approve' ? 'تمت الموافقة على السحب' : 'تم رفض السحب', 'success');
    } catch (e: unknown) {
      toasts.add(e instanceof Error ? e.message : 'فشل تنفيذ العملية', 'error');
    }
    processing = '';
  }

  function formatAmount(s: string) {
    return parseFloat(s).toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 8 });
  }

  function formatDate(s: string) {
    return new Date(s).toLocaleString('ar-EG');
  }
</script>

<div class="animate-fade-in">
  <div class="flex items-center justify-between mb-6">
    <h1 class="text-2xl font-bold">السحوبات المعلقة</h1>
    <button class="btn-ghost btn-sm flex items-center gap-1.5" on:click={fetchPending} disabled={loading}>
      <svg class="w-4 h-4" class:animate-spin={loading} fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"/></svg>
      تحديث
    </button>
  </div>

  {#if loading && withdrawals.length === 0}
    <div class="flex justify-center py-12">
      <div class="animate-spin rounded-full h-8 w-8 border-2 border-amber-500 border-t-transparent"></div>
    </div>
  {:else if withdrawals.length === 0}
    <div class="card text-center py-10">
      <svg class="w-12 h-12 mx-auto text-gray-600 mb-3" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"/></svg>
      <p class="text-gray-500">لا توجد سحوبات معلقة</p>
    </div>
  {:else}
    <div class="card-table">
      <table>
        <thead>
          <tr class="border-b border-dark-600">
            <th>التاريخ</th>
            <th>المستخدم</th>
            <th>المبلغ</th>
            <th>العملة</th>
            <th>الإجراءات</th>
          </tr>
        </thead>
        <tbody>
          {#each withdrawals as w, i}
            <tr class="border-b border-dark-700 hover:bg-dark-700/50 transition-colors">
              <td class="text-xs text-gray-400">{formatDate(w.created_at)}</td>
              <td class="font-medium">{w.user_email}</td>
              <td class="font-mono">{formatAmount(w.amount)}</td>
              <td><span class="badge badge-active">{w.currency}</span></td>
              <td>
                <div class="flex gap-2">
                  <button
                    class="btn-success btn-xs"
                    disabled={processing === w.tx_id}
                    on:click={() => processTx(w.tx_id, 'approve')}
                  >
                    {processing === w.tx_id ? '...' : 'موافقة'}
                  </button>
                  <button
                    class="btn-danger btn-xs"
                    disabled={processing === w.tx_id}
                    on:click={() => processTx(w.tx_id, 'reject')}
                  >
                    {processing === w.tx_id ? '...' : 'رفض'}
                  </button>
                </div>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {/if}
</div>
