<script lang="ts">
 import { api } from '$lib/api';
 import { wallets } from '$lib/stores/exchange';
 import type { Order } from '$lib/api';

 export let base = 'BTC';
 export let quote = 'EGP';

 let side = 'buy';
 let orderType = 'limit';
 let price = '';
 let quantity = '';
 let error = '';
 let success = '';
 let submitting = false;

 $: egpWallet = $wallets.find(w => w.currency === quote);
 $: baseWallet = $wallets.find(w => w.currency === base);
 $: balanceDisplay = side === 'buy' ? egpWallet?.balance ?? '0' : baseWallet?.balance ?? '0';

 async function handleSubmit() {
   submitting = true;
   error = '';
   success = '';
   try {
     const body: Record<string, unknown> = {
       side,
       order_type: orderType,
       base_currency: base,
       quote_currency: quote,
       quantity: parseFloat(quantity),
     };
     if (orderType === 'limit') body.price = parseFloat(price);

     await api.post<Order>('/orders', body);
     success = `${side.toUpperCase()} order placed successfully`;
     quantity = '';
     price = '';
   } catch (e: unknown) {
     error = e instanceof Error ? e.message : 'Order failed';
   } finally {
     submitting = false;
   }
 }

 function setMax() {
   if (side === 'buy' && orderType === 'limit' && price) {
     quantity = (parseFloat(balanceDisplay) / parseFloat(price)).toFixed(8);
   } else {
     quantity = balanceDisplay;
   }
 }

 $: pct25 = side === 'buy' && orderType === 'limit' && price
   ? ((parseFloat(balanceDisplay) * 0.25) / parseFloat(price)).toFixed(8)
   : (parseFloat(balanceDisplay) * 0.25).toFixed(8);
 $: pct50 = side === 'buy' && orderType === 'limit' && price
   ? ((parseFloat(balanceDisplay) * 0.5) / parseFloat(price)).toFixed(8)
   : (parseFloat(balanceDisplay) * 0.5).toFixed(8);
 $: pct75 = side === 'buy' && orderType === 'limit' && price
   ? ((parseFloat(balanceDisplay) * 0.75) / parseFloat(price)).toFixed(8)
   : (parseFloat(balanceDisplay) * 0.75).toFixed(8);
</script>

<div class="card">
  <div class="flex mb-4">
    <button class="flex-1 py-2 text-sm font-medium rounded-l-lg" class:bg-accent-green:text-white={side === 'buy'} class:bg-dark-700:text-gray-400={side !== 'buy'} on:click={() => side = 'buy'}>
      Buy
    </button>
    <button class="flex-1 py-2 text-sm font-medium rounded-r-lg" class:bg-accent-red:text-white={side === 'sell'} class:bg-dark-700:text-gray-400={side !== 'sell'} on:click={() => side = 'sell'}>
      Sell
    </button>
  </div>

  <form on:submit|preventDefault={handleSubmit} class="space-y-3">
    <div class="flex gap-2">
      <button class="flex-1 py-1.5 text-xs rounded" class:bg-blue-600:text-white={orderType === 'limit'} class:bg-dark-700:text-gray-400={orderType !== 'limit'} on:click={() => orderType = 'limit'}>Limit</button>
      <button class="flex-1 py-1.5 text-xs rounded" class:bg-blue-600:text-white={orderType === 'market'} class:bg-dark-700:text-gray-400={orderType !== 'market'} on:click={() => orderType = 'market'}>Market</button>
    </div>

    {#if orderType === 'limit'}
      <div>
        <label class="label">Price ({quote})</label>
        <input type="number" step="0.01" class="input" bind:value={price} required />
      </div>
    {/if}

    <div>
      <label class="label">Quantity ({base})</label>
      <input type="number" step="0.00000001" class="input" bind:value={quantity} required />
    </div>

    <div class="flex gap-1">
      <button type="button" class="flex-1 py-1 text-xs btn-ghost" on:click={() => quantity = pct25}>25%</button>
      <button type="button" class="flex-1 py-1 text-xs btn-ghost" on:click={() => quantity = pct50}>50%</button>
      <button type="button" class="flex-1 py-1 text-xs btn-ghost" on:click={() => quantity = pct75}>75%</button>
      <button type="button" class="flex-1 py-1 text-xs btn-ghost" on:click={setMax}>Max</button>
    </div>

    <div class="text-xs text-gray-400">
      Available: {parseFloat(balanceDisplay).toFixed(8)} {side === 'buy' ? quote : base}
    </div>

    {#if error}
      <div class="bg-red-900/50 border border-red-700 text-red-300 rounded-lg p-2 text-xs">{error}</div>
    {/if}
    {#if success}
      <div class="bg-green-900/50 border border-green-700 text-green-300 rounded-lg p-2 text-xs">{success}</div>
    {/if}

    <button type="submit" class="w-full py-2.5 text-sm font-medium rounded-lg text-white"
      class:bg-accent-green:hover:bg-green-700={side === 'buy'}
      class:bg-accent-red:hover:bg-red-700={side === 'sell'}
      disabled={submitting}>
      {submitting ? 'Processing...' : `${side === 'buy' ? 'Buy' : 'Sell'} ${base}`}
    </button>
  </form>
</div>
