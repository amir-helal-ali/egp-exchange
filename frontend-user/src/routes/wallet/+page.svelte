<script lang="ts">
 import { onMount } from 'svelte';
 import { goto } from '$app/navigation';
 import { isAuthenticated, wallets } from '$lib/stores/exchange';
 import { api } from '$lib/api';
 import type { Wallet, ManualTransaction } from '$lib/api';

 let depositAmount = '';
 let withdrawAmount = '';
 let withdrawCurrency = 'EGP';
 let error = '';
 let success = '';
 let submitting = false;
 let transactions: ManualTransaction[] = [];
 let loading = true;

 onMount(async () => {
   if (!$isAuthenticated) goto('/login');
   try {
     const [w, tx] = await Promise.all([
       api.get<Wallet[]>('/wallets'),
       api.get<ManualTransaction[]>('/wallets/transactions')
     ]);
     wallets.set(w);
     transactions = tx;
   } catch {}
   loading = false;
 });

 async function requestDeposit() {
   submitting = true;
   error = '';
   success = '';
   try {
     await api.post('/wallets/deposit/egp', { amount: parseFloat(depositAmount) });
     success = `Deposit of ${depositAmount} EGP submitted for approval`;
     depositAmount = '';
   } catch (e: unknown) {
     error = e instanceof Error ? e.message : 'Request failed';
   } finally {
     submitting = false;
   }
 }

 async function requestWithdrawal() {
   submitting = true;
   error = '';
   success = '';
   try {
     await api.post('/wallets/withdraw', { currency: withdrawCurrency, amount: parseFloat(withdrawAmount) });
     success = `Withdrawal of ${withdrawAmount} ${withdrawCurrency} submitted for approval`;
     withdrawAmount = '';
   } catch (e: unknown) {
     error = e instanceof Error ? e.message : 'Request failed';
   } finally {
     submitting = false;
   }
 }

 function formatDecimal(s: string) {
   return parseFloat(s).toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 8 });
 }

 $: pendingTx = transactions.filter(t => t.status === 'pending');
</script>

<div class="space-y-6">
  <h1 class="text-2xl font-bold">Wallet</h1>

  {#if loading}
    <div class="flex justify-center py-12">
      <div class="animate-spin rounded-full h-8 w-8 border-2 border-blue-500 border-t-transparent"></div>
    </div>
  {:else}
    {#if error}
      <div class="bg-red-900/50 border border-red-700 text-red-300 rounded-lg p-3 text-sm">{error}</div>
    {/if}
    {#if success}
      <div class="bg-green-900/50 border border-green-700 text-green-300 rounded-lg p-3 text-sm">{success}</div>
    {/if}

    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
      {#each $wallets as wallet}
        <div class="card">
          <div class="flex items-center justify-between mb-2">
            <span class="text-sm font-semibold">{wallet.currency}</span>
            {#if wallet.currency === 'EGP'}
              <span class="text-xs text-accent-gold">Fiat</span>
            {:else}
              <span class="text-xs text-blue-400">Crypto</span>
            {/if}
          </div>
          <p class="text-2xl font-bold">{formatDecimal(wallet.balance)}</p>
          <p class="text-xs text-gray-500">Locked: {formatDecimal(wallet.locked)}</p>
        </div>
      {/each}
    </div>

    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
      <div class="card">
        <h2 class="text-lg font-semibold mb-4">Deposit EGP</h2>
        <p class="text-sm text-gray-400 mb-4">Submit a manual deposit request. An admin will process it.</p>
        <form on:submit|preventDefault={requestDeposit} class="space-y-3">
          <div>
            <label class="label">Amount (EGP)</label>
            <input type="number" step="0.01" min="1" class="input" bind:value={depositAmount} required />
          </div>
          <button type="submit" class="btn-success w-full" disabled={submitting}>
            {submitting ? 'Submitting...' : 'Request Deposit'}
          </button>
        </form>
      </div>

      <div class="card">
        <h2 class="text-lg font-semibold mb-4">Withdraw</h2>
        <p class="text-sm text-gray-400 mb-4">Submit a withdrawal request. An admin will process it.</p>
        <form on:submit|preventDefault={requestWithdrawal} class="space-y-3">
          <div>
            <label class="label">Currency</label>
            <select class="input" bind:value={withdrawCurrency}>
              <option value="EGP">EGP</option>
              <option value="BTC">BTC</option>
              <option value="ETH">ETH</option>
              <option value="USDT">USDT</option>
            </select>
          </div>
          <div>
            <label class="label">Amount</label>
            <input type="number" step="0.00000001" min="0.00000001" class="input" bind:value={withdrawAmount} required />
          </div>
          <button type="submit" class="btn-primary w-full" disabled={submitting}>
            {submitting ? 'Submitting...' : 'Request Withdrawal'}
          </button>
        </form>
      </div>
    </div>
  {/if}
</div>
