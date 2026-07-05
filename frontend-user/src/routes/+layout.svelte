<script lang="ts">
  import { onMount } from 'svelte';
  import { page } from '$app/stores';
  import { currentUser, isAuthenticated, wsConnected, logout } from '$lib/stores/exchange';
  import { api, setToken, getToken } from '$lib/api';
  import { getWs, destroyWs } from '$lib/ws';
  import '../app.css';

  let loading = true;
  let mobileMenuOpen = false;
  let tradeDropdownOpen = false;

  onMount(async () => {
    const t = getToken();
    if (t) {
      try {
        const user = await api.get<import('$lib/api').User>('/users/me');
        currentUser.set(user);
        const ws = getWs();
        ws.onConnect(() => wsConnected.set(true));
        ws.onDisconnect(() => wsConnected.set(false));
        ws.connect();
      } catch {
        logout();
      }
    }
    loading = false;
  });

  function handleLogout() {
    destroyWs();
    logout();
  }

  $: isActive = (path: string) => $page.url.pathname === path || $page.url.pathname.startsWith(path + '/');
  $: isTradePage = $page.url.pathname.startsWith('/trade');
</script>

<div class="min-h-screen flex flex-col">
  <nav class="bg-dark-800 border-b border-dark-600 sticky top-0 z-50">
    <div class="max-w-7xl mx-auto px-4">
      <div class="flex items-center justify-between h-14">
        <div class="flex items-center gap-1">
          <a href="/" class="text-xl font-bold tracking-wider ml-6">
            <span class="text-amber-400">EGP</span><span class="text-gray-400">EX</span>
          </a>
          {#if $isAuthenticated}
            <div class="hidden md:flex items-center gap-1">
              <a href="/dashboard" class="nav-link" class:nav-link-active={isActive('/dashboard')}>لوحة التحكم</a>
              <div class="relative">
                <button class="nav-link flex items-center gap-1" class:nav-link-active={isTradePage} on:click={() => tradeDropdownOpen = !tradeDropdownOpen} on:mouseenter={() => tradeDropdownOpen = true}>
                  تداول
                  <svg class="w-3 h-3 transition-transform" class:rotate-180={tradeDropdownOpen} fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"/></svg>
                </button>
                {#if tradeDropdownOpen}
                  <div class="absolute top-full right-0 mt-1 bg-dark-800 border border-dark-600 rounded-lg shadow-xl py-1 min-w-[160px] z-50" on:mouseleave={() => tradeDropdownOpen = false}>
                    <a href="/trade" class="block px-4 py-2 text-sm text-gray-300 hover:bg-dark-700 hover:text-amber-400 transition-colors">تداول فوري</a>
                    <a href="/trade/futures" class="block px-4 py-2 text-sm text-gray-300 hover:bg-dark-700 hover:text-amber-400 transition-colors">عقود آجلة</a>
                  </div>
                {/if}
              </div>
              <a href="/p2p" class="nav-link" class:nav-link-active={$page.url.pathname.startsWith('/p2p')}>سوق P2P</a>
              <a href="/wallet" class="nav-link" class:nav-link-active={isActive('/wallet')}>المحفظة</a>
            </div>
          {/if}
        </div>

        <div class="flex items-center gap-3">
          {#if $wsConnected}
            <span class="hidden sm:flex items-center gap-1 text-[10px] text-emerald-400">
              <span class="w-1.5 h-1.5 rounded-full bg-emerald-400 animate-pulse"></span>
              متصل
            </span>
          {:else if $isAuthenticated}
            <span class="hidden sm:flex items-center gap-1 text-[10px] text-gray-500">
              <span class="w-1.5 h-1.5 rounded-full bg-gray-500"></span>
              غير متصل
            </span>
          {/if}

          {#if $isAuthenticated}
            <span class="hidden sm:block text-sm text-gray-400">{$currentUser?.email}</span>
            <button class="btn-ghost text-xs px-3 py-1.5" on:click={handleLogout}>تسجيل خروج</button>
          {:else if !loading}
            <a href="/login" class="btn-gold text-xs px-4 py-1.5">دخول</a>
            <a href="/register" class="btn-ghost text-xs px-3 py-1.5">تسجيل</a>
          {/if}

          <button class="md:hidden p-2 text-gray-400 hover:text-white" on:click={() => mobileMenuOpen = !mobileMenuOpen}>
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              {#if mobileMenuOpen}
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
              {:else}
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"/>
              {/if}
            </svg>
          </button>
        </div>
      </div>
    </div>

    {#if mobileMenuOpen && $isAuthenticated}
      <div class="md:hidden border-t border-dark-600 bg-dark-800 animate-slide-up">
        <div class="px-4 py-3 space-y-1">
          <a href="/dashboard" class="block nav-link" class:nav-link-active={isActive('/dashboard')}>لوحة التحكم</a>
          <a href="/trade" class="block nav-link" class:nav-link-active={isActive('/trade') && !$page.url.pathname.includes('/futures')}>تداول فوري</a>
          <a href="/trade/futures" class="block nav-link" class:nav-link-active={$page.url.pathname.includes('/futures')}>عقود آجلة</a>
          <a href="/p2p" class="block nav-link" class:nav-link-active={$page.url.pathname.startsWith('/p2p')}>سوق P2P</a>
          <a href="/wallet" class="block nav-link" class:nav-link-active={isActive('/wallet')}>المحفظة</a>
          <div class="pt-2 border-t border-dark-600">
            <span class="block text-sm text-gray-400 px-3 py-1">{$currentUser?.email}</span>
          </div>
        </div>
      </div>
    {/if}
  </nav>

  <main class="flex-1">
    {#if loading}
      <div class="flex items-center justify-center h-96">
        <div class="text-center">
          <div class="animate-spin rounded-full h-10 w-10 border-2 border-amber-500 border-t-transparent mx-auto mb-3"></div>
          <p class="text-gray-500 text-sm">جاري التحميل...</p>
        </div>
      </div>
    {:else}
      <slot />
    {/if}
  </main>

  <footer class="bg-dark-800 border-t border-dark-600 py-4 text-center text-xs text-gray-500">
    <span class="text-amber-400">EGPEX</span> &copy; 2024 - جميع الحقوق محفوظة
  </footer>
</div>
