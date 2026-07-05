<script lang="ts">
 import { onMount } from 'svelte';
 import { page } from '$app/stores';
 import { currentUser, isAuthenticated, logout } from '$lib/stores/exchange';
 import { api, setToken, getToken } from '$lib/api';
 import '../app.css';

 let loading = true;

 onMount(async () => {
   const t = getToken();
   if (t) {
     try {
       const user = await api.get<import('$lib/api').User>('/users/me');
       currentUser.set(user);
     } catch {
       logout();
     }
   }
   loading = false;
 });

 function handleLogout() {
   logout();
 }
</script>

<div class="min-h-screen flex flex-col">
  <nav class="bg-dark-800 border-b border-dark-600 px-4 py-3 flex items-center justify-between">
    <div class="flex items-center gap-6">
      <a href="/" class="text-lg font-bold text-accent-gold tracking-wider">EGP<span class="text-gray-400">EX</span></a>
      {#if $isAuthenticated}
        <div class="flex gap-4 text-sm">
          <a href="/dashboard" class="text-gray-300 hover:text-white transition-colors">Dashboard</a>
          <a href="/trade" class="text-gray-300 hover:text-white transition-colors">Trade</a>
          <a href="/wallet" class="text-gray-300 hover:text-white transition-colors">Wallet</a>
        </div>
      {/if}
    </div>
    <div class="flex items-center gap-3">
      {#if $isAuthenticated}
        <span class="text-sm text-gray-400">{$currentUser?.email}</span>
        <button class="btn-ghost text-sm" on:click={handleLogout}>Logout</button>
      {:else if !loading}
        <a href="/login" class="btn-primary text-sm">Login</a>
        <a href="/register" class="btn-ghost text-sm">Register</a>
      {/if}
    </div>
  </nav>

  <main class="flex-1 p-4">
    {#if loading}
      <div class="flex items-center justify-center h-64">
        <div class="animate-spin rounded-full h-8 w-8 border-2 border-blue-500 border-t-transparent"></div>
      </div>
    {:else}
      <slot />
    {/if}
  </main>
</div>
