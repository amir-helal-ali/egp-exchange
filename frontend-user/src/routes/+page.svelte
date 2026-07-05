<script lang="ts">
 import { isAuthenticated } from '$lib/stores/exchange';
 import { page } from '$app/stores';
</script>

<div class="flex flex-col items-center justify-center min-h-[60vh] text-center">
  <h1 class="text-4xl font-bold mb-4">EGP<span class="text-accent-gold">EX</span></h1>
  <p class="text-gray-400 text-lg mb-8 max-w-md">
    Egyptian Pound Cryptocurrency Exchange — Trade BTC, ETH, and USDT with EGP
  </p>
  {#if !$isAuthenticated}
    <div class="flex gap-4">
      <a href="/register" class="btn-primary px-6 py-3 text-base">Get Started</a>
      <a href="/login" class="btn-ghost px-6 py-3 text-base">Sign In</a>
    </div>
  {:else}
    <a href="/trade" class="btn-primary px-6 py-3 text-base">Start Trading</a>
  {/if}
  <div class="mt-12 grid grid-cols-1 md:grid-cols-3 gap-6 max-w-3xl">
    <div class="card text-left">
      <h3 class="text-accent-green font-semibold mb-2">Spot Trading</h3>
      <p class="text-gray-400 text-sm">Real-time order book with limit and market orders</p>
    </div>
    <div class="card text-left">
      <h3 class="text-blue-400 font-semibold mb-2">Multi-Currency</h3>
      <p class="text-gray-400 text-sm">BTC, ETH, USDT pairs against the Egyptian Pound</p>
    </div>
    <div class="card text-left">
      <h3 class="text-accent-gold font-semibold mb-2">EGP Gateway</h3>
      <p class="text-gray-400 text-sm">Bank-grade EGP deposits and withdrawals via admin approval</p>
    </div>
  </div>
</div>
