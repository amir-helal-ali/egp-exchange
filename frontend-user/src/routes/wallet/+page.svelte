<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { isAuthenticated, wallets } from '$lib/stores/exchange';
  import { api, type Wallet, type ManualTransaction } from '$lib/api';

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
      success = `تم تقديم طلب إيداع ${depositAmount} جنيه مصري بنجاح`;
      depositAmount = '';
    } catch (e: unknown) {
      error = e instanceof Error ? e.message : 'فشل الطلب';
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
      success = `تم تقديم طلب سحب ${withdrawAmount} ${withdrawCurrency} بنجاح`;
      withdrawAmount = '';
    } catch (e: unknown) {
      error = e instanceof Error ? e.message : 'فشل الطلب';
    } finally {
      submitting = false;
    }
  }

  function fmt(s: string, dp: number = 4) {
    return parseFloat(s).toLocaleString('en-US', { minimumFractionDigits: dp, maximumFractionDigits: dp });
  }

  function txTypeText(t: string) {
    switch (t) {
      case 'deposit': return 'إيداع';
      case 'withdrawal': return 'سحب';
      default: return t;
    }
  }

  function txStatusText(s: string) {
    switch (s) {
      case 'pending': return 'قيد المراجعة';
      case 'approved': return 'تمت الموافقة';
      case 'rejected': return 'مرفوض';
      default: return s;
    }
  }

  function txStatusBadge(s: string) {
    switch (s) {
      case 'pending': return 'badge-yellow';
      case 'approved': return 'badge-green';
      case 'rejected': return 'badge-red';
      default: return 'badge-gray';
    }
  }
</script>

<div class="max-w-7xl mx-auto p-4 space-y-6 animate-fade-in">
  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-bold">المحفظة</h1>
  </div>

  {#if loading}
    <div class="flex justify-center py-16">
      <div class="animate-spin rounded-full h-8 w-8 border-2 border-amber-500 border-t-transparent"></div>
    </div>
  {:else}
    {#if error}
      <div class="bg-red-900/50 border border-red-700 text-red-300 rounded-lg p-3 text-sm">{error}</div>
    {/if}
    {#if success}
      <div class="bg-emerald-900/50 border border-emerald-700 text-emerald-300 rounded-lg p-3 text-sm">{success}</div>
    {/if}

    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
      {#each $wallets as wallet}
        <div class="card hover:border-amber-500/30 transition-all">
          <div class="flex items-center justify-between mb-2">
            <div class="flex items-center gap-2">
              <span class="text-lg font-bold">{wallet.currency}</span>
              {#if wallet.currency === 'EGP'}
                <span class="badge-yellow text-[10px]">نقدي</span>
              {:else}
                <span class="badge-blue text-[10px]">رقمي</span>
              {/if}
            </div>
          </div>
          <p class="text-2xl font-bold text-gray-100">{fmt(wallet.balance)}</p>
          <p class="text-xs text-gray-500 mt-1">المجمد: {fmt(wallet.locked)}</p>
        </div>
      {/each}
    </div>

    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
      <div class="card">
        <h2 class="text-lg font-semibold mb-2">إيداع جنيه مصري</h2>
        <p class="text-sm text-gray-400 mb-4">تقديم طلب إيداع نقدي - ستتم معالجته من قبل الإدارة</p>
        <form on:submit|preventDefault={requestDeposit} class="space-y-3">
          <div>
            <label class="label">مبلغ الإيداع (EGP)</label>
            <input type="number" step="0.01" min="1" class="input" placeholder="أدخل المبلغ" bind:value={depositAmount} required />
          </div>
          <button type="submit" class="btn-success w-full" disabled={submitting}>
            {submitting ? 'جاري...' : 'طلب إيداع'}
          </button>
        </form>
      </div>

      <div class="card">
        <h2 class="text-lg font-semibold mb-2">سحب</h2>
        <p class="text-sm text-gray-400 mb-4">تقديم طلب سحب - ستتم معالجته من قبل الإدارة</p>
        <form on:submit|preventDefault={requestWithdrawal} class="space-y-3">
          <div>
            <label class="label">العملة</label>
            <select class="input" bind:value={withdrawCurrency}>
              <option value="EGP">EGP</option>
              <option value="BTC">BTC</option>
              <option value="ETH">ETH</option>
              <option value="USDT">USDT</option>
            </select>
          </div>
          <div>
            <label class="label">المبلغ</label>
            <input type="number" step="0.00000001" min="0.00000001" class="input" placeholder="أدخل المبلغ" bind:value={withdrawAmount} required />
          </div>
          <button type="submit" class="btn-primary w-full" disabled={submitting}>
            {submitting ? 'جاري...' : 'طلب سحب'}
          </button>
        </form>
      </div>
    </div>

    <div class="card">
      <h2 class="text-lg font-semibold mb-4">سجل المعاملات</h2>
      {#if transactions.length === 0}
        <p class="text-gray-500 text-sm py-6 text-center">لا توجد معاملات بعد</p>
      {:else}
        <div class="overflow-x-auto">
          <table class="w-full text-sm">
            <thead>
              <tr class="text-gray-400 border-b border-dark-600">
                <th class="pb-2">النوع</th>
                <th class="pb-2">العملة</th>
                <th class="pb-2">المبلغ</th>
                <th class="pb-2">الحالة</th>
                <th class="pb-2">التاريخ</th>
              </tr>
            </thead>
            <tbody>
              {#each transactions as tx}
                <tr class="hover:bg-dark-700/30">
                  <td class="py-2">{txTypeText(tx.tx_type)}</td>
                  <td class="py-2">{tx.currency}</td>
                  <td class="py-2 font-medium">{fmt(tx.amount)}</td>
                  <td class="py-2">
                    <span class={txStatusBadge(tx.status)}>{txStatusText(tx.status)}</span>
                  </td>
                  <td class="py-2 text-gray-400 text-xs">{new Date(tx.created_at).toLocaleDateString('ar-EG')}</td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      {/if}
    </div>
  {/if}
</div>
