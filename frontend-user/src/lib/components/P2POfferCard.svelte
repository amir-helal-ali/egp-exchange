<script lang="ts">
  import { api, type P2pOffer, type P2pOrder } from '$lib/api';
  import { p2pOrders } from '$lib/stores/exchange';

  export let offer: P2pOffer;
  export let onAccept: (() => void) | null = null;

  let amount = '';
  let submitting = false;
  let error = '';
  let success = '';

  async function acceptOffer() {
    if (!amount) return;
    submitting = true;
    error = '';
    success = '';
    try {
      const order = await api.post<P2pOrder>(`/p2p/offers/${offer.id}/accept`, { amount: parseFloat(amount) });
      p2pOrders.update(o => [order, ...o]);
      success = 'تم قبول العرض بنجاح';
      if (onAccept) onAccept();
    } catch (e: unknown) {
      error = e instanceof Error ? e.message : 'فشل قبول العرض';
    } finally {
      submitting = false;
    }
  }

  $: availableNum = parseFloat(offer.available);
  $: priceNum = parseFloat(offer.price);
  $: minNum = parseFloat(offer.min_amount);
  $: maxNum = parseFloat(offer.max_amount);
</script>

<div class="card animate-fade-in relative overflow-hidden">
  <div class="absolute top-0 left-0 w-1 h-full" class:bg-emerald-500={offer.type === 'buy'} class:bg-red-500={offer.type === 'sell'}></div>
  <div class="flex items-start justify-between mb-3">
    <div>
      <div class="flex items-center gap-2 mb-1">
        <span class:badge-green={offer.type === 'buy'} class:badge-red={offer.type === 'sell'}>
          {offer.type === 'buy' ? 'شراء' : 'بيع'}
        </span>
        <span class="text-sm font-medium text-gray-200">مستخدم</span>
      </div>
      <span class="text-xs text-gray-500">{offer.currency}/{offer.fiat_currency} عبر {offer.payment_method}</span>
    </div>
    <div class="text-left">
      <p class="text-lg font-bold text-amber-400">{priceNum.toFixed(2)}</p>
      <p class="text-xs text-gray-500">{offer.fiat_currency}</p>
    </div>
  </div>

  <div class="grid grid-cols-2 gap-3 text-xs mb-4">
    <div>
      <span class="text-gray-500">المتاح:</span>
      <span class="text-gray-200 mr-1">{availableNum.toFixed(4)} {offer.currency}</span>
    </div>
    <div class="text-left">
      <span class="text-gray-500">الحد:</span>
      <span class="text-gray-200 mr-1">{minNum.toFixed(2)} - {maxNum.toFixed(2)} {offer.fiat_currency}</span>
    </div>
  </div>

  {#if success}
    <div class="bg-emerald-900/50 border border-emerald-700 text-emerald-300 rounded-lg p-2 text-xs mb-3">{success}</div>
  {:else}
    <form on:submit|preventDefault={acceptOffer} class="space-y-2">
      <div>
        <input type="number" step="0.0001" class="input text-xs" placeholder={`المبلغ (${offer.currency})`} bind:value={amount} required />
      </div>
      {#if error}
        <div class="bg-red-900/50 border border-red-700 text-red-300 rounded-lg p-2 text-xs">{error}</div>
      {/if}
      <button type="submit" class="w-full btn-gold text-xs py-2" disabled={submitting}>
        {submitting ? 'جاري...' : 'قبول العرض'}
      </button>
    </form>
  {/if}
</div>
