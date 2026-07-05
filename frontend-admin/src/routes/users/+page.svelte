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
</script>

<div>
  <h1 class="text-2xl font-bold mb-6">User Management</h1>

  {#if loading}
    <div class="flex justify-center py-12">
      <div class="animate-spin rounded-full h-8 w-8 border-2 border-blue-500 border-t-transparent"></div>
    </div>
  {:else}
    <div class="card overflow-hidden">
      <table class="w-full text-sm">
        <thead>
          <tr class="text-gray-400 text-left border-b border-dark-600">
            <th class="pb-3 pr-4">Email</th>
            <th class="pb-3 pr-4">Role</th>
            <th class="pb-3 pr-4">Wallets</th>
            <th class="pb-3">Actions</th>
          </tr>
        </thead>
        <tbody>
          {#each users as user}
            <tr class="border-b border-dark-700 hover:bg-dark-700/50">
              <td class="py-3 pr-4">{user.email}</td>
              <td class="py-3 pr-4">
                <span class="px-2 py-0.5 rounded text-xs" class:bg-blue-900/50:text-blue-300={user.role === 'admin'} class:bg-gray-700:text-gray-300={user.role !== 'admin'}>
                  {user.role}
                </span>
              </td>
              <td class="py-3 pr-4">
                <button class="text-blue-400 hover:underline text-xs" on:click={() => selectedUserId = selectedUserId === user.id ? null : user.id}>
                  {getUserWallets(user.id).length} wallets
                </button>
              </td>
              <td class="py-3">
                <span class="text-xs text-gray-500">{user.id.slice(0, 8)}...</span>
              </td>
            </tr>
            {#if selectedUserId === user.id}
              <tr class="bg-dark-700/30">
                <td colspan="4" class="py-3 px-4">
                  <div class="grid grid-cols-4 gap-3">
                    {#each getUserWallets(user.id) as wallet}
                      <div class="text-xs">
                        <span class="text-gray-400">{wallet.currency}</span>
                        <span class="ml-2 font-mono">{formatBalance(wallet.balance)}</span>
                        {#if parseFloat(wallet.locked) > 0}
                          <span class="text-yellow-400 ml-1">(locked: {formatBalance(wallet.locked)})</span>
                        {/if}
                      </div>
                    {/each}
                  </div>
                </td>
              </tr>
            {/if}
          {/each}
        </tbody>
      </table>
    </div>
  {/if}
</div>
