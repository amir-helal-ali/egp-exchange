<script lang="ts">
  import { api, type OpenPositionRequest, type FuturesPosition } from '$lib/api';
  import { wallets } from '$lib/stores/exchange';

  export let base = 'BTC';
  export let quote = 'EGP';

  let side = 'long';
  let orderType = 'isolated';
  let leverage = 10;
  let quantity = '';
  let margin = '';
  let takeProfit = '';
  let stopLoss = '';
  let error = '';
  let success = '';
  let submitting = false;

  $: entryPrice = 0;
  $: marginNum = parseFloat(margin) || 0;
  $: qtyNum = parseFloat(quantity) || 0;
  $: positionValue = entryPrice * qtyNum;
  $: requiredMargin = positionValue / leverage;
  $: liqPrice = side === 'long'
    ? entryPrice - entryPrice / leverage
    : entryPrice + entryPrice / leverage;
  $: pnlPercent = side === 'long'
    ? leverage * 0.01
    : -leverage * 0.01;

  $: quoteWallet = $wallets.find(w => w.currency === quote);

  async function handleSubmit() {
    submitting = true;
    error = '';
    success = '';
    try {
      const body: OpenPositionRequest = {
        pair: `${base}/${quote}`,
        side,
        quantity: qtyNum,
        leverage,
      };
      await api.post<FuturesPosition>('/futures/positions', body);
      success = side === 'long' ? 'تم فتح صفقة شراء بنجاح' : 'تم فتح صفقة بيع بنجاح';
      quantity = '';
      margin = '';
    } catch (e: unknown) {
      error = e instanceof Error ? e.message : 'فشل فتح الصفقة';
    } finally {
      submitting = false;
    }
  }

  function setPct(pct: number) {
    if (quoteWallet) {
      margin = (parseFloat(quoteWallet.balance) * pct).toFixed(2);
    }
  }
</script>

<div class="card">
  <div class="flex rounded-lg overflow-hidden mb-4">
    <button
      class="flex-1 py-2.5 text-sm font-medium transition-all duration-200"
      class:bg-emerald-600:text-white={side === 'long'}
      class:bg-dark-700:text-gray-400={side !== 'long'}
      on:click={() => side = 'long'}
    >شراء (Long)</button>
    <button
      class="flex-1 py-2.5 text-sm font-medium transition-all duration-200"
      class:bg-red-600:text-white={side === 'short'}
      class:bg-dark-700:text-gray-400={side !== 'short'}
      on:click={() => side = 'short'}
    >بيع (Short)</button>
  </div>

  <form on:submit|preventDefault={handleSubmit} class="space-y-3">
    <div>
      <label class="label">الرافعة المالية ({leverage}x)</label>
      <input type="range" min="1" max="125" step="1" class="w-full accent-amber-500" bind:value={leverage} />
      <div class="flex justify-between text-xs text-gray-500">
        <span>1x</span>
        <span class="text-amber-400 font-bold">{leverage}x</span>
        <span>125x</span>
      </div>
    </div>

    <div>
      <div class="flex justify-between items-center">
        <label class="label">الكمية ({base})</label>
      </div>
      <input type="number" step="0.0001" class="input" bind:value={quantity} required />
    </div>

    <div>
      <div class="flex justify-between items-center">
        <label class="label">الهامش ({quote})</label>
        <span class="text-xs text-gray-500">الرصيد: {quoteWallet ? parseFloat(quoteWallet.balance).toFixed(2) : '0'} {quote}</span>
      </div>
      <input type="number" step="0.01" class="input" bind:value={margin} />
    </div>

    <div class="flex gap-1">
      <button type="button" class="flex-1 py-1 text-xs btn-ghost rounded" on:click={() => setPct(0.25)}>25%</button>
      <button type="button" class="flex-1 py-1 text-xs btn-ghost rounded" on:click={() => setPct(0.5)}>50%</button>
      <button type="button" class="flex-1 py-1 text-xs btn-ghost rounded" on:click={() => setPct(0.75)}>75%</button>
      <button type="button" class="flex-1 py-1 text-xs btn-ghost rounded" on:click={() => setPct(1)}>100%</button>
    </div>

    {#if qtyNum > 0 && entryPrice > 0}
      <div class="bg-dark-700/50 rounded-lg p-3 space-y-1.5 text-xs">
        <div class="flex justify-between">
          <span class="text-gray-400">قيمة الصفقة</span>
          <span>{positionValue.toFixed(2)} {quote}</span>
        </div>
        <div class="flex justify-between">
          <span class="text-gray-400">الهامش المطلوب</span>
          <span>{requiredMargin.toFixed(2)} {quote}</span>
        </div>
        <div class="flex justify-between">
          <span class="text-gray-400">سعر التصفية التقريبي</span>
          <span class="text-red-400">{liqPrice.toFixed(2)}</span>
        </div>
        <div class="flex justify-between">
          <span class="text-gray-400">ربح/خسارة 1%</span>
          <span class:text-emerald-400={pnlPercent >= 0} class:text-red-400={pnlPercent < 0}>
            {pnlPercent >= 0 ? '+' : ''}{pnlPercent.toFixed(2)}%
          </span>
        </div>
      </div>
    {/if}

    <div class="border-t border-dark-600 pt-3 mt-2">
      <p class="text-xs text-gray-400 mb-2">وقف الربح / وقف الخسارة</p>
      <div class="grid grid-cols-2 gap-2">
        <div>
          <label class="label text-[10px]">وقف الربح (TP)</label>
          <input type="number" step="0.01" class="input text-xs" bind:value={takeProfit} placeholder="السعر" />
        </div>
        <div>
          <label class="label text-[10px]">وقف الخسارة (SL)</label>
          <input type="number" step="0.01" class="input text-xs" bind:value={stopLoss} placeholder="السعر" />
        </div>
      </div>
    </div>

    {#if error}
      <div class="bg-red-900/50 border border-red-700 text-red-300 rounded-lg p-2.5 text-xs">{error}</div>
    {/if}
    {#if success}
      <div class="bg-emerald-900/50 border border-emerald-700 text-emerald-300 rounded-lg p-2.5 text-xs">{success}</div>
    {/if}

    <button type="submit"
      class="w-full py-2.5 text-sm font-medium rounded-lg text-white transition-all duration-200"
      class:btn-long={side === 'long'}
      class:btn-short={side === 'short'}
      class:opacity-50:cursor-not-allowed={submitting}
      disabled={submitting}>
      {submitting ? 'جاري التنفيذ...' : side === 'long' ? `فتح شراء ${base}` : `فتح بيع ${base}`}
    </button>
  </form>
</div>
