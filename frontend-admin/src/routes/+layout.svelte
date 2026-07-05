<script lang="ts">
  import { onMount } from 'svelte';
  import { writable, derived } from 'svelte/store';
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { api, setToken, getToken } from '$lib/api';
  import type { User } from '$lib/api';
  import Toast from '$lib/components/Toast.svelte';
  import '../app.css';

  const currentUser = writable<User | null>(null);
  const isAuthenticated = derived(currentUser, ($u) => $u !== null);
  let sidebarOpen = false;
  let loading = true;

  $: isLoginPage = $page.url.pathname === '/login';

  onMount(async () => {
    const t = getToken();
    if (t) {
      try {
        const user = await api.get<User>('/users/me');
        currentUser.set(user);
      } catch {
        setToken(null);
      }
    }
    loading = false;
  });

  function handleLogout() {
    setToken(null);
    currentUser.set(null);
    goto('/login');
  }

  function isActive(path: string) {
    if (path === '/') return $page.url.pathname === '/';
    return $page.url.pathname.startsWith(path);
  }

  function navClass(path: string) {
    return isActive(path) ? 'sidebar-link sidebar-link-active' : 'sidebar-link sidebar-link-inactive';
  }
</script>

{#if loading}
  <div class="min-h-screen flex items-center justify-center bg-dark-900">
    <div class="animate-spin rounded-full h-8 w-8 border-2 border-amber-500 border-t-transparent"></div>
  </div>
{:else if !$isAuthenticated}
  <slot />
  <Toast />
{:else}
  <div class="min-h-screen bg-dark-900">
    {#if sidebarOpen}
      <div class="fixed inset-0 bg-black/50 z-20 lg:hidden" role="presentation" on:click={() => sidebarOpen = false}></div>
    {/if}

    <aside class="fixed top-0 right-0 h-full w-64 bg-dark-800 border-l border-dark-600 flex flex-col z-30 transition-transform duration-300 {sidebarOpen ? 'translate-x-0' : 'translate-x-full'} lg:translate-x-0">
      <div class="p-5 border-b border-dark-600">
        <a href="/" class="text-xl font-bold text-amber-400">EGP<span class="text-gray-400">EX</span></a>
        <p class="text-xs text-gray-500 mt-0.5">لوحة التحكم</p>
      </div>

      <nav class="flex-1 p-3 space-y-1 overflow-y-auto">
        <a href="/" class={navClass('/')}>
          <svg class="w-4 h-4 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6"/></svg>
          لوحة التحكم
        </a>

        <div class="pt-3 pb-1">
          <p class="text-xs text-gray-500 px-3 font-medium">المعاملات</p>
        </div>
        <a href="/deposits" class={navClass('/deposits')}>
          <svg class="w-4 h-4 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6"/></svg>
          الإيداعات
        </a>
        <a href="/withdrawals" class={navClass('/withdrawals')}>
          <svg class="w-4 h-4 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 12H4"/></svg>
          السحوبات
        </a>

        <div class="pt-3 pb-1">
          <p class="text-xs text-gray-500 px-3 font-medium">الإدارة</p>
        </div>
        <a href="/users" class={navClass('/users')}>
          <svg class="w-4 h-4 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197m13.5-9a2.5 2.5 0 11-5 0 2.5 2.5 0 015 0z"/></svg>
          المستخدمين
        </a>
        <a href="/currencies" class={navClass('/currencies')}>
          <svg class="w-4 h-4 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/></svg>
          العملات
        </a>
        <a href="/pairs" class={navClass('/pairs')}>
          <svg class="w-4 h-4 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7h12m0 0l-4-4m4 4l-4 4m0 6H4m0 0l4 4m-4-4l4-4"/></svg>
          أزواج التداول
        </a>
        <a href="/settings" class={navClass('/settings')}>
          <svg class="w-4 h-4 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.066 2.573c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.573 1.066c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.066-2.573c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"/><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"/></svg>
          الإعدادات
        </a>

        <div class="pt-3 pb-1">
          <p class="text-xs text-gray-500 px-3 font-medium">الأسواق</p>
        </div>
        <a href="/futures" class={navClass('/futures')}>
          <svg class="w-4 h-4 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7h8m0 0v8m0-8l-8 8-4-4-6 6"/></svg>
          العقود الآجلة
        </a>
        <a href="/p2p" class={navClass('/p2p')}>
          <svg class="w-4 h-4 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"/></svg>
          السوق P2P
        </a>
        <a href="/liquidity" class={navClass('/liquidity')}>
          <svg class="w-4 h-4 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19.428 15.428a2 2 0 00-1.022-.547l-2.387-.477a6 6 0 00-3.86.517l-.318.158a6 6 0 01-3.86.517L6.05 15.21a2 2 0 00-1.806.547M8 4h8l-1 1v5.172a2 2 0 00.586 1.414l5 5c1.26 1.26.367 3.414-1.415 3.414H4.828c-1.782 0-2.674-2.154-1.414-3.414l5-5A2 2 0 009 10.172V5L8 4z"/></svg>
          السيولة
        </a>
      </nav>

      <div class="p-4 border-t border-dark-600">
        <p class="text-xs text-gray-400 truncate mb-2">{$currentUser?.email}</p>
        <button class="btn-danger btn-sm w-full" on:click={handleLogout}>تسجيل الخروج</button>
      </div>
    </aside>

    <div class="mr-0 lg:mr-64 min-h-screen flex flex-col">
      <header class="h-14 bg-dark-800 border-b border-dark-600 flex items-center justify-between px-4 sticky top-0 z-10">
        <button on:click={() => sidebarOpen = !sidebarOpen} class="lg:hidden p-2 text-gray-400 hover:text-white rounded-lg hover:bg-dark-700">
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"/></svg>
        </button>
        <div class="flex items-center gap-3">
          <span class="text-sm text-gray-400 hidden sm:block">{$currentUser?.email}</span>
          <button on:click={handleLogout} class="text-xs bg-red-600 hover:bg-red-700 text-white px-3 py-1.5 rounded-lg">تسجيل الخروج</button>
        </div>
      </header>
      <main class="flex-1 p-6">
        <slot />
      </main>
    </div>
    <Toast />
  </div>
{/if}
