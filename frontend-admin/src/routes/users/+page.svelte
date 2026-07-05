<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { api, getToken } from '$lib/api';
  import type { User, Wallet } from '$lib/api';

  let users: User[] = [];
  let wallets: Wallet[] = [];
  let loading = true;
  let selectedUserId: string | null = null;

  onMount(() => {
    if (!getToken()) goto('/login');
    fetchData();
  });

  async function fetchData() {
    loading = true;
    try {
      const [u, w] = await Promise.all([
        api.get<User[]>('/admin/users'),
        api.get<Wallet[]>('/admin/wallets'),
      ]);
      users = u;
      wallets = w;
    } catch {}
    loading = false;
  }

  function getUserWallets(userId: string) {
    return wallets.filter(w => w.user_id === userId);
  }

  function formatBalance(s: string) {
    return parseFloat(s).toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 8 });
  }

  function formatDate(s: string) {
    return new Date(s).toLocaleDateString('ar-EG');
  }

  function roleBadge(role: string) {
    if (role === 'admin') return 'badge bg-purple-900/50 text-purple-300 border border-purple-700';
    return 'badge bg-gray-700/50 text-gray-300';
  }
</script>

<div class="animate-fade-in">
  <div class="flex items-center justify-between mb-6">
    <h1 class="text-2xl font-bold">إدارة المستخدمين</h1>
    <button class="btn-ghost btn-sm flex items-center gap-1.5" on:click={fetchData} disabled={loading}>
      <svg class="w-4 h-4" class:animate-spin={loading} fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"/></svg>
      تحديث
    </button>
  </div>

  {#if loading && users.length === 0}
    <div class="flex justify-center py-12">
      <div class="animate-spin rounded-full h-8 w-8 border-2 border-amber-500 border-t-transparent"></div>
    </div>
  {:else}
    <div class="card-table">
      <table>
        <thead>
          <tr class="border-b border-dark-600">
            <th>البريد الإلكتروني</th>
            <th>الدور</th>
            <th>تاريخ التسجيل</th>
            <th>المحافظ</th>
            <th>المعرف</th>
          </tr>
        </thead>
        <tbody>
          {#each users as user}
            <tr class="border-b border-dark-700 hover:bg-dark-700/50 transition-colors">
              <td class="font-medium">{user.email}</td>
              <td>
                <span class={roleBadge(user.role)}>{user.role === 'admin' ? 'مشرف' : 'مستخدم'}</span>
              </td>
              <td class="text-xs text-gray-400">{user.created_at ? formatDate(user.created_at) : '---'}</td>
              <td>
                <button
                  class="text-amber-400 hover:text-amber-300 text-xs flex items-center gap-1"
                  on:click={() => selectedUserId = selectedUserId === user.id ? null : user.id}
                >
                  <svg class="w-3.5 h-3.5" class:rotate-90={selectedUserId === user.id} fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"/></svg>
                  {getUserWallets(user.id).length} محفظة
                </button>
              </td>
              <td class="text-xs text-gray-500">{user.id.slice(0, 8)}...</td>
            </tr>
            {#if selectedUserId === user.id}
              <tr class="bg-dark-700/30">
                <td colspan="5" class="px-6 py-4">
                  {#if getUserWallets(user.id).length === 0}
                    <p class="text-gray-500 text-xs text-center">لا توجد محافظ</p>
                  {:else}
                    <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-3">
                      {#each getUserWallets(user.id) as wallet}
                        <div class="bg-dark-800/80 rounded-lg px-3 py-2.5 border border-dark-600">
                          <p class="text-xs text-amber-400 font-medium mb-1">{wallet.currency}</p>
                          <p class="text-sm font-mono">{formatBalance(wallet.balance)}</p>
                          {#if parseFloat(wallet.locked) > 0}
                            <p class="text-xs text-yellow-400 mt-0.5">مجمّد: {formatBalance(wallet.locked)}</p>
                          {/if}
                        </div>
                      {/each}
                    </div>
                  {/if}
                </td>
              </tr>
            {/if}
          {/each}
        </tbody>
      </table>
    </div>
  {/if}
</div>
