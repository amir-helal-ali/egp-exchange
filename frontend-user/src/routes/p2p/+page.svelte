<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { isAuthenticated, p2pOffers } from '$lib/stores/exchange';
  import { api, type P2pOffer, type CreateOfferRequest } from '$lib/api';
  import P2POfferCard from '$lib/components/P2POfferCard.svelte';

  let typeFilter = 'all';
  let searchQuery = '';
  let loading = true;
  let showCreateModal = false;

  let newOffer: CreateOfferRequest = {
    type: 'sell',
    currency: 'USDT',
    price: 0,
    available: 0,
    min_amount: 0,
    max_amount: 0,
    payment_method: 'تحويل بنكي',
  };
  let createError = '';
  let createSubmitting = false;

  onMount(async () => {
    if (!$isAuthenticated) goto('/login');
    try {
      const offers = await api.get<P2pOffer[]>('/p2p/offers');
      p2pOffers.set(offers);
    } catch {}
    loading = false;
  });

  $: filtered = $p2pOffers.filter(o => {
    if (typeFilter !== 'all' && o.type !== typeFilter) return false;
    if (searchQuery && !o.currency.toLowerCase().includes(searchQuery.toLowerCase())) return false;
    return o.status === 'active';
  });

  async function createOffer() {
    createSubmitting = true;
    createError = '';
    try {
      const offer = await api.post<P2pOffer>('/p2p/offers', newOffer);
      p2pOffers.update(o => [offer, ...o]);
      showCreateModal = false;
      newOffer = {
        type: 'sell',
        currency: 'USDT',
        price: 0,
        available: 0,
        min_amount: 0,
        max_amount: 0,
        payment_method: 'تحويل بنكي',
      };
    } catch (e: unknown) {
      createError = e instanceof Error ? e.message : 'فشل إنشاء العرض';
    } finally {
      createSubmitting = false;
    }
  }
</script>

<div class="max-w-7xl mx-auto p-4 space-y-6 animate-fade-in">
  <div class="flex items-center justify-between flex-wrap gap-4">
    <div>
      <h1 class="text-2xl font-bold">سوق P2P</h1>
      <p class="text-sm text-gray-400">تداول مباشر مع مستخدمين آخرين</p>
    </div>
    <button class="btn-gold text-sm" on:click={() => showCreateModal = true}>+ إنشاء عرض</button>
  </div>

  <div class="flex flex-wrap items-center gap-3">
    <div class="flex rounded-lg overflow-hidden border border-dark-600">
      <button class="tab-btn px-4 py-1.5 text-xs" class:tab-btn-active={typeFilter === 'all'} class:tab-btn-inactive={typeFilter !== 'all'} on:click={() => typeFilter = 'all'}>الكل</button>
      <button class="tab-btn px-4 py-1.5 text-xs" class:tab-btn-active={typeFilter === 'buy'} class:tab-btn-inactive={typeFilter !== 'buy'} on:click={() => typeFilter = 'buy'}>شراء</button>
      <button class="tab-btn px-4 py-1.5 text-xs" class:tab-btn-active={typeFilter === 'sell'} class:tab-btn-inactive={typeFilter !== 'sell'} on:click={() => typeFilter = 'sell'}>بيع</button>
    </div>
    <input type="text" class="input w-48 text-xs" placeholder="بحث عن عملة..." bind:value={searchQuery} />
  </div>

  {#if loading}
    <div class="flex justify-center py-16">
      <div class="animate-spin rounded-full h-8 w-8 border-2 border-amber-500 border-t-transparent"></div>
    </div>
  {:else if filtered.length === 0}
    <div class="card text-center py-12">
      <p class="text-gray-500">لا توجد عروض متاحة حالياً</p>
    </div>
  {:else}
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      {#each filtered as offer}
        <P2POfferCard {offer} />
      {/each}
    </div>
  {/if}
</div>

{#if showCreateModal}
  <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm" on:click={() => showCreateModal = false}>
    <div class="card max-w-md w-full mx-4 animate-slide-up" on:click|stopPropagation>
      <div class="flex items-center justify-between mb-4">
        <h2 class="text-lg font-semibold">إنشاء عرض جديد</h2>
        <button class="text-gray-400 hover:text-white transition-colors" on:click={() => showCreateModal = false}>
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/></svg>
        </button>
      </div>

      <form on:submit|preventDefault={createOffer} class="space-y-3">
        <div class="flex gap-2">
          <button type="button" class="flex-1 py-2 rounded-lg text-sm font-medium transition-all" class:btn-long={newOffer.type === 'buy'} class:bg-dark-700:text-gray-400={newOffer.type !== 'buy'} on:click={() => newOffer.type = 'buy'}>شراء</button>
          <button type="button" class="flex-1 py-2 rounded-lg text-sm font-medium transition-all" class:btn-short={newOffer.type === 'sell'} class:bg-dark-700:text-gray-400={newOffer.type !== 'sell'} on:click={() => newOffer.type = 'sell'}>بيع</button>
        </div>

        <div class="grid grid-cols-2 gap-3">
          <div>
            <label class="label">العملة</label>
            <select class="input" bind:value={newOffer.currency}>
              <option value="BTC">BTC</option>
              <option value="ETH">ETH</option>
              <option value="USDT">USDT</option>
            </select>
          </div>
          <div>
            <label class="label">السعر (EGP)</label>
            <input type="number" step="0.01" class="input" bind:value={newOffer.price} required />
          </div>
        </div>

        <div>
          <label class="label">الكمية المتاحة ({newOffer.currency})</label>
          <input type="number" step="0.0001" class="input" bind:value={newOffer.available} required />
        </div>

        <div class="grid grid-cols-2 gap-3">
          <div>
            <label class="label">الحد الأدنى (EGP)</label>
            <input type="number" step="0.01" class="input" bind:value={newOffer.min_amount} required />
          </div>
          <div>
            <label class="label">الحد الأقصى (EGP)</label>
            <input type="number" step="0.01" class="input" bind:value={newOffer.max_amount} required />
          </div>
        </div>

        <div>
          <label class="label">طريقة الدفع</label>
          <input type="text" class="input" bind:value={newOffer.payment_method} placeholder="تحويل بنكي - فودافون كاش" required />
        </div>

        {#if createError}
          <div class="bg-red-900/50 border border-red-700 text-red-300 rounded-lg p-2.5 text-xs">{createError}</div>
        {/if}

        <button type="submit" class="btn-gold w-full" disabled={createSubmitting}>
          {createSubmitting ? 'جاري...' : 'إنشاء العرض'}
        </button>
      </form>
    </div>
  </div>
{/if}
