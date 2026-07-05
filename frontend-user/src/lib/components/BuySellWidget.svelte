<script lang="ts">
  import { api, type Order, type PlaceOrderRequest } from '$lib/api';
  import { wallets } from '$lib/stores/exchange';

  export let base = 'BTC';
  export let quote = 'EGP';

  let side = 'buy';
  let orderType: 'limit' | 'market' | 'stop-limit' = 'limit';
  let price = '';
  let stopPrice = '';
  let quantity = '';
  let error = '';
  let success = '';
  let submitting = false;

  $: quoteWallet = $wallets.find(w => w.currency === quote);
  $: baseWallet = $wallets.find(w => w.currency === base);
  $: balanceDisplay = side === 'buy' ? quoteWallet?.balance ?? '0' : baseWallet?.balance ?? '0';
  $: totalDisplay = price && quantity ? (parseFloat(price) * parseFloat(quantity)).toFixed(2) : '—';

  async function handleSubmit() {
    submitting = true;
    error = '';
    success = '';
    try {
      const body: PlaceOrderRequest = {
        side,
        order_type: orderType,
        base_currency: base,
        quote_currency: quote,
        quantity: parseFloat(quantity),
      };
      if (orderType === 'limit' || orderType === 'stop-limit') body.price = parseFloat(price);
      if (orderType === 'stop-limit') body.stop_price = parseFloat(stopPrice);
      await api.post<Order>('/orders', body);
      success = side === 'buy' ? '✅ تم تقديم أمر الشراء بنجاح' : '✅ تم تقديم أمر البيع بنجاح';
      quantity = '';
      price = '';
      stopPrice = '';
    } catch (e: unknown) {
      error = e instanceof Error ? e.message : 'فشل الأمر';
    } finally {
      submitting = false;
    }
  }

  function setPct(pct: number) {
    const bal = parseFloat(balanceDisplay);
    if (side === 'buy' && (orderType === 'limit' || orderType === 'stop-limit') && price) {
      quantity = ((bal * pct) / parseFloat(price)).toFixed(8);
    } else {
      quantity = (bal * pct).toFixed(8);
    }
  }

  function setMax() { setPct(1); }
</script>

<div class="card">
  <div class="flex rounded-lg overflow-hidden mb-4">
    <button
      class="flex-1 py-2.5 text-sm font-medium transition-all duration-200"
      class:bg-emerald-600:text-white={side === 'buy'}
      class:bg-dark-700:text-gray-400={side !== 'buy'}
      on:click={() => side = 'buy'}
    >شراء</button>
    <button
      class="flex-1 py-2.5 text-sm font-medium transition-all duration-200"
      class:bg-red-600:text-white={side === 'sell'}
      class:bg-dark-700:text-gray-400={side !== 'sell'}
      on:click={() => side = 'sell'}
    >بيع</button>
  </div>

  <form on:submit|preventDefault={handleSubmit} class="space-y-3">
    <div class="flex gap-1">
      <button type="button"
        class="flex-1 py-1.5 text-xs rounded transition-all duration-200"
        class:bg-amber-600:text-white={orderType === 'limit'}
        class:bg-dark-700:text-gray-400={orderType !== 'limit'}
        on:click={() => orderType = 'limit'}
      >محدد</button>
      <button type="button"
        class="flex-1 py-1.5 text-xs rounded transition-all duration-200"
        class:bg-amber-600:text-white={orderType === 'market'}
        class:bg-dark-700:text-gray-400={orderType !== 'market'}
        on:click={() => orderType = 'market'}
      >سوقي</button>
      <button type="button"
        class="flex-1 py-1.5 text-xs rounded transition-all duration-200"
        class:bg-amber-600:text-white={orderType === 'stop-limit'}
        class:bg-dark-700:text-gray-400={orderType !== 'stop-limit'}
        on:click={() => orderType = 'stop-limit'}
      >إيقاف</button>
    </div>

    {#if orderType === 'limit' || orderType === 'stop-limit'}
      <div>
        <label class="label">السعر ({quote})</label>
        <input type="number" step="0.01" class="input" bind:value={price} required />
      </div>
    {/if}

    {#if orderType === 'stop-limit'}
      <div>
        <label class="label">سعر الإيقاف ({quote})</label>
        <input type="number" step="0.01" class="input" bind:value={stopPrice} required placeholder="مثلاً 52000" />
      </div>
    {/if}

    <div>
      <div class="flex justify-between items-center">
        <label class="label">الكمية ({base})</label>
        <span class="text-xs text-gray-500">الرصيد: {parseFloat(balanceDisplay).toFixed(8)}</span>
      </div>
      <input type="number" step="0.00000001" class="input" bind:value={quantity} required />
    </div>

    {#if price && quantity && orderType !== 'market'}
      <div class="flex justify-between text-xs text-gray-400">
        <span>الإجمالي</span>
        <span>{totalDisplay} {quote}</span>
      </div>
    {/if}

    <div class="flex gap-1">
      <button type="button" class="flex-1 py-1 text-xs btn-ghost rounded" on:click={() => setPct(0.25)}>25%</button>
      <button type="button" class="flex-1 py-1 text-xs btn-ghost rounded" on:click={() => setPct(0.5)}>50%</button>
      <button type="button" class="flex-1 py-1 text-xs btn-ghost rounded" on:click={() => setPct(0.75)}>75%</button>
      <button type="button" class="flex-1 py-1 text-xs btn-ghost rounded" on:click={setMax}>100%</button>
    </div>

    {#if error}<div class="bg-red-900/50 border border-red-700 text-red-300 rounded-lg p-2.5 text-xs">{error}</div>{/if}
    {#if success}<div class="bg-emerald-900/50 border border-emerald-700 text-emerald-300 rounded-lg p-2.5 text-xs">{success}</div>{/if}

    <button type="submit"
      class="w-full py-2.5 text-sm font-medium rounded-lg text-white transition-all duration-200"
      class:btn-buy={side === 'buy'}
      class:btn-sell={side === 'sell'}
      class:opacity-50:cursor-not-allowed={submitting}
      disabled={submitting}>
      {submitting ? 'جاري التنفيذ...' : side === 'buy' ? `شراء ${base}` : `بيع ${base}`}
    </button>
  </form>
</div>
