<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { isAuthenticated, p2pOrders } from '$lib/stores/exchange';
  import { api, type P2pOrder } from '$lib/api';
  import P2POrderCard from '$lib/components/P2POrderCard.svelte';

  let tab: 'active' | 'completed' | 'cancelled' = 'active';
  let loading = true;

  onMount(async () => {
    if (!$isAuthenticated) goto('/login');
    try {
      const orders = await api.get<P2pOrder[]>('/p2p/orders');
      p2pOrders.set(orders);
    } catch {}
    loading = false;
  });

  $: active = $p2pOrders.filter(o => o.status === 'pending' || o.status === 'paid');
  $: completed = $p2pOrders.filter(o => o.status === 'completed');
  $: cancelled = $p2pOrders.filter(o => o.status === 'cancelled' || o.status === 'disputed');

  $: currentOrders = tab === 'active' ? active : tab === 'completed' ? completed : cancelled;
</script>

<div class="max-w-4xl mx-auto p-4 space-y-6 animate-fade-in">
  <h1 class="text-2xl font-bold">طلبات P2P الخاصة بي</h1>

  <div class="flex rounded-lg overflow-hidden border border-dark-600">
    <button
      class="flex-1 py-2 text-sm font-medium transition-all duration-200"
      class:bg-amber-600:text-white={tab === 'active'}
      class:bg-dark-700:text-gray-400={tab !== 'active'}
      on:click={() => tab = 'active'}
    >نشطة ({active.length})</button>
    <button
      class="flex-1 py-2 text-sm font-medium transition-all duration-200"
      class:bg-amber-600:text-white={tab === 'completed'}
      class:bg-dark-700:text-gray-400={tab !== 'completed'}
      on:click={() => tab = 'completed'}
    >مكتملة ({completed.length})</button>
    <button
      class="flex-1 py-2 text-sm font-medium transition-all duration-200"
      class:bg-amber-600:text-white={tab === 'cancelled'}
      class:bg-dark-700:text-gray-400={tab !== 'cancelled'}
      on:click={() => tab = 'cancelled'}
    >ملغية ({cancelled.length})</button>
  </div>

  {#if loading}
    <div class="flex justify-center py-16">
      <div class="animate-spin rounded-full h-8 w-8 border-2 border-amber-500 border-t-transparent"></div>
    </div>
  {:else if currentOrders.length === 0}
    <div class="card text-center py-12">
      <p class="text-gray-500">
        {tab === 'active' ? 'لا توجد طلبات نشطة' : tab === 'completed' ? 'لا توجد طلبات مكتملة' : 'لا توجد طلبات ملغية'}
      </p>
    </div>
  {:else}
    <div class="space-y-4">
      {#each currentOrders as order}
        <P2POrderCard {order} />
      {/each}
    </div>
  {/if}
</div>
