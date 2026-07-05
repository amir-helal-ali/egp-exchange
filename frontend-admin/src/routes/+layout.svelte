<script lang="ts">
 import { onMount } from 'svelte';
 import { writable, derived } from 'svelte/store';
 import { page } from '$app/stores';
 import { api, setToken, getToken } from '$lib/api';
 import type { User } from '$lib/api';
 import '../app.css';

 const currentUser = writable<User | null>(null);
 const isAuthenticated = derived(currentUser, ($u) => $u !== null);

 onMount(async () => {
   const t = getToken();
   if (t) {
     try {
       const user = await api.get<User>('/users/me');
       currentUser.set(user);
     } catch {
       setToken(null);
       currentUser.set(null);
     }
   }
 });

 function handleLogout() {
   setToken(null);
   currentUser.set(null);
 }
</script>

<div class="min-h-screen flex">
  <aside class="w-56 bg-dark-800 border-r border-dark-600 flex flex-col">
    <div class="p-4 border-b border-dark-600">
      <a href="/" class="text-lg font-bold text-accent-gold">EGP<span class="text-gray-400">EX</span></a>
      <p class="text-xs text-gray-500 mt-0.5">Admin Panel</p>
    </div>
    {#if $isAuthenticated}
      <nav class="flex-1 p-3 space-y-1">
        <a href="/deposits" class="block px-3 py-2 text-sm rounded-lg hover:bg-dark-700 text-gray-300 hover:text-white transition-colors">Deposits</a>
        <a href="/withdrawals" class="block px-3 py-2 text-sm rounded-lg hover:bg-dark-700 text-gray-300 hover:text-white transition-colors">Withdrawals</a>
        <a href="/users" class="block px-3 py-2 text-sm rounded-lg hover:bg-dark-700 text-gray-300 hover:text-white transition-colors">Users</a>
        <a href="/liquidity" class="block px-3 py-2 text-sm rounded-lg hover:bg-dark-700 text-gray-300 hover:text-white transition-colors">Liquidity</a>
      </nav>
      <div class="p-3 border-t border-dark-600">
        <p class="text-xs text-gray-400 mb-2">{$currentUser?.email}</p>
        <button class="btn-ghost text-xs w-full" on:click={handleLogout}>Logout</button>
      </div>
    {/if}
  </aside>

  <main class="flex-1 p-6 overflow-auto">
    <slot />
  </main>
</div>
