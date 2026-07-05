<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { api, getToken } from '$lib/api';
  import type { PendingTransactions, User, FuturesPosition } from '$lib/api';

  let pendingDeposits = 0;
  let pendingWithdrawals = 0;
  let totalUsers = 0;
  let openPositions = 0;
  let loading = true;

  onMount(() => {
    if (!getToken()) goto('/login');
    loadStats();
  });

  async function loadStats() {
    try {
      const [txns, users, positions] = await Promise.all([
        api.get<PendingTransactions>('/admin/transactions/pending').catch(() => ({ deposits: [], withdrawals: [] } as PendingTransactions)),
        api.get<User[]>('/admin/users').catch(() => [] as User[]),
        api.get<FuturesPosition[]>('/admin/futures/positions').catch(() => [] as FuturesPosition[]),
      ]);
      pendingDeposits = txns.deposits.length;
      pendingWithdrawals = txns.withdrawals.length;
      totalUsers = users.length;
      openPositions = positions.filter(p => p.status === 'open').length;
    } catch {}
    loading = false;
  }
</script>

<div class="animate-fade-in">
  <h1 class="text-2xl font-bold mb-6">لوحة التحكم</h1>

  {#if loading}
    <div class="flex justify-center py-12">
      <div class="animate-spin rounded-full h-8 w-8 border-2 border-amber-500 border-t-transparent"></div>
    </div>
  {:else}
    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 mb-8">
      <a href="/deposits" class="stat-card hover:border-amber-600/50 transition-colors">
        <p class="text-gray-400 text-xs mb-1">إيداعات معلقة</p>
        <p class="text-3xl font-bold text-amber-400">{pendingDeposits}</p>
      </a>
      <a href="/withdrawals" class="stat-card hover:border-amber-600/50 transition-colors">
        <p class="text-gray-400 text-xs mb-1">سحوبات معلقة</p>
        <p class="text-3xl font-bold text-amber-400">{pendingWithdrawals}</p>
      </a>
      <a href="/users" class="stat-card hover:border-amber-600/50 transition-colors">
        <p class="text-gray-400 text-xs mb-1">إجمالي المستخدمين</p>
        <p class="text-3xl font-bold text-blue-400">{totalUsers}</p>
      </a>
      <a href="/futures" class="stat-card hover:border-amber-600/50 transition-colors">
        <p class="text-gray-400 text-xs mb-1">صفقات مفتوحة</p>
        <p class="text-3xl font-bold text-orange-400">{openPositions}</p>
      </a>
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
      <div class="card">
        <h2 class="text-lg font-semibold mb-3">روابط سريعة</h2>
        <div class="grid grid-cols-2 gap-3">
          <a href="/deposits" class="flex items-center gap-2 px-3 py-2.5 rounded-lg bg-dark-700/50 hover:bg-dark-700 text-sm text-gray-300 hover:text-white transition-colors">
            <svg class="w-4 h-4 text-green-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6"/></svg>
            إدارة الإيداعات
          </a>
          <a href="/withdrawals" class="flex items-center gap-2 px-3 py-2.5 rounded-lg bg-dark-700/50 hover:bg-dark-700 text-sm text-gray-300 hover:text-white transition-colors">
            <svg class="w-4 h-4 text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 12H4"/></svg>
            إدارة السحوبات
          </a>
          <a href="/currencies" class="flex items-center gap-2 px-3 py-2.5 rounded-lg bg-dark-700/50 hover:bg-dark-700 text-sm text-gray-300 hover:text-white transition-colors">
            <svg class="w-4 h-4 text-amber-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/></svg>
            إدارة العملات
          </a>
          <a href="/liquidity" class="flex items-center gap-2 px-3 py-2.5 rounded-lg bg-dark-700/50 hover:bg-dark-700 text-sm text-gray-300 hover:text-white transition-colors">
            <svg class="w-4 h-4 text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19.428 15.428a2 2 0 00-1.022-.547l-2.387-.477a6 6 0 00-3.86.517l-.318.158a6 6 0 01-3.86.517L6.05 15.21a2 2 0 00-1.806.547M8 4h8l-1 1v5.172a2 2 0 00.586 1.414l5 5c1.26 1.26.367 3.414-1.415 3.414H4.828c-1.782 0-2.674-2.154-1.414-3.414l5-5A2 2 0 009 10.172V5L8 4z"/></svg>
            مراقبة السيولة
          </a>
        </div>
      </div>
      <div class="card">
        <h2 class="text-lg font-semibold mb-3">حالة النظام</h2>
        <div class="space-y-3 text-sm">
          <div class="flex items-center justify-between py-2 border-b border-dark-700">
            <span class="text-gray-400">عدد الإيداعات المعلقة</span>
            <span class="font-medium text-amber-400">{pendingDeposits}</span>
          </div>
          <div class="flex items-center justify-between py-2 border-b border-dark-700">
            <span class="text-gray-400">عدد السحوبات المعلقة</span>
            <span class="font-medium text-amber-400">{pendingWithdrawals}</span>
          </div>
          <div class="flex items-center justify-between py-2 border-b border-dark-700">
            <span class="text-gray-400">إجمالي المستخدمين</span>
            <span class="font-medium text-blue-400">{totalUsers}</span>
          </div>
          <div class="flex items-center justify-between py-2">
            <span class="text-gray-400">صفقات العقود الآجلة المفتوحة</span>
            <span class="font-medium text-orange-400">{openPositions}</span>
          </div>
        </div>
      </div>
    </div>
  {/if}
</div>
