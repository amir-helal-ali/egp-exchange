<script lang="ts">
 import { goto } from '$app/navigation';
 import { getToken } from '$lib/api';
</script>

<div class="flex flex-col items-center justify-center min-h-[60vh] text-center">
  <h1 class="text-3xl font-bold mb-4">Admin Panel</h1>
  <p class="text-gray-400 mb-8">Manage deposits, withdrawals, users, and monitor liquidity</p>
  {#if getToken()}
    <a href="/deposits" class="btn-primary px-6 py-3">Go to Dashboard</a>
  {:else}
    <a href="/login" class="btn-primary px-6 py-3">Sign In</a>
  {/if}
</div>
