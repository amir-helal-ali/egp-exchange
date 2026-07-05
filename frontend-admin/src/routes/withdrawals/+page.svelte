<script lang="ts">
 import { onMount } from 'svelte';
 import { goto } from '$app/navigation';
 import { api, getToken } from '$lib/api';
 import type { PendingTransactions, QueueItem } from '$lib/api';

 let withdrawals: QueueItem[] = [];
 let loading = true;
 let processing = '';

 onMount(() => {
   if (!getToken()) goto('/login');
   fetchPending();
 });

 async function fetchPending() {
   try {
     const data = await api.get<PendingTransactions>('/admin/transactions/pending');
     withdrawals = data.withdrawals;
   } catch {}
   loading = false;
 }

 async function processTx(id: string, action: string) {
   processing = id;
   try {
     await api.post(`/admin/transactions/${id}`, { action, notes: null });
     withdrawals = withdrawals.filter(w => w.tx_id !== id);
   } catch {}
   processing = '';
 }

 function formatAmount(s: string) {
   return parseFloat(s).toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 8 });
 }
</script>

<div>
  <div class="flex items-center justify-between mb-6">
    <h1 class="text-2xl font-bold">Pending Withdrawals</h1>
    <button class="btn-ghost text-sm" on:click={fetchPending}>Refresh</button>
  </div>

  {#if loading}
    <div class="flex justify-center py-12">
      <div class="animate-spin rounded-full h-8 w-8 border-2 border-blue-500 border-t-transparent"></div>
    </div>
  {:else if withdrawals.length === 0}
    <div class="card text-center py-8">
      <p class="text-gray-500">No pending withdrawals</p>
    </div>
  {:else}
    <div class="card overflow-hidden">
      <table class="w-full text-sm">
        <thead>
          <tr class="text-gray-400 text-left border-b border-dark-600">
            <th class="pb-3 pr-4">Date</th>
            <th class="pb-3 pr-4">User</th>
            <th class="pb-3 pr-4">Amount</th>
            <th class="pb-3 pr-4">Currency</th>
            <th class="pb-3">Actions</th>
          </tr>
        </thead>
        <tbody>
          {#each withdrawals as w, i}
            <tr class="border-b border-dark-700 hover:bg-dark-700/50">
              <td class="py-3 pr-4 text-xs">{new Date(w.created_at).toLocaleString()}</td>
              <td class="py-3 pr-4">{w.user_email}</td>
              <td class="py-3 pr-4 font-mono">{formatAmount(w.amount)}</td>
              <td class="py-3 pr-4">{w.currency}</td>
              <td class="py-3 flex gap-2">
                <button class="btn-success text-xs px-3 py-1" disabled={processing === w.tx_id} on:click={() => processTx(w.tx_id, 'approve')}>
                  {processing === w.tx_id ? '...' : 'Approve'}
                </button>
                <button class="btn-danger text-xs px-3 py-1" disabled={processing === w.tx_id} on:click={() => processTx(w.tx_id, 'reject')}>
                  Reject
                </button>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {/if}
</div>
