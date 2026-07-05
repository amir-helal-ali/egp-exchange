<script lang="ts">
  import { onMount } from 'svelte';
  import { api, type FuturesPosition, type SetTpSlRequest } from '$lib/api';
  import { positions } from '$lib/stores/exchange';
  import { getWs } from '$lib/ws';

  export let userId: string | null = null;

  let loading = true;
  let editingTpSl: Record<string, { tp: string; sl: string }> = {};
  let partialQty: Record<string, string> = {};

  onMount(async () => {
    try {
      const pos = await api.get<FuturesPosition[]>('/futures/positions');
      positions.set(pos);
    } catch {}
    loading = false;

    if (userId) {
      const ws = getWs();
      ws.subscribe(`positions:${userId}`, (data: unknown) => {
        const pos = data as FuturesPosition[];
        positions.set(pos);
      });
      ws.connect();
    }
  });

  async function closePosition(id: string) {
    try {
      await api.delete(`/futures/positions/${id}`);
      positions.update(p => p.filter(pos => pos.id !== id));
    } catch {}
  }

  function startEditTpSl(pos: FuturesPosition) {
    editingTpSl[pos.id] = { tp: pos.take_profit ?? '', sl: pos.stop_loss ?? '' };
    editingTpSl = editingTpSl;
  }

  async function saveTpSl(id: string) {
    const vals = editingTpSl[id];
    if (!vals) return;
    const body: SetTpSlRequest = {
      take_profit: vals.tp ? parseFloat(vals.tp) : null,
      stop_loss: vals.sl ? parseFloat(vals.sl) : null,
    };
    try {
      const updated = await api.post<FuturesPosition>(`/futures/positions/${id}/tp-sl`, body);
      positions.update(p => p.map(pos => pos.id === id ? updated : pos));
      delete editingTpSl[id];
      editingTpSl = editingTpSl;
    } catch {}
  }

  async function partialClose(id: string) {
    const qtyStr = partialQty[id];
    if (!qtyStr) return;
    const qty = parseFloat(qtyStr);
    if (qty <= 0) return;
    try {
      const result = await api.post<{ remaining_quantity: string }>(`/futures/positions/${id}/partial-close`, { quantity: qty });
      positions.update(p => p.map(pos =>
        pos.id === id ? { ...pos, quantity: result.remaining_quantity } : pos
      ));
      delete partialQty[id];
      partialQty = partialQty;
    } catch {}
  }

  function fmt(v: string, dp: number = 2) {
    return parseFloat(v).toLocaleString('en-US', { minimumFractionDigits: dp, maximumFractionDigits: dp });
  }

  function calcROE(pos: FuturesPosition) {
    const pnl = parseFloat(pos.unrealized_pnl);
    const margin = parseFloat(pos.margin);
    return margin > 0 ? (pnl / margin) * 100 : 0;
  }
</script>

<div class="card">
  <h3 class="text-sm font-semibold text-gray-300 mb-4">المراكز المفتوحة</h3>
  {#if loading}
    <div class="flex justify-center py-8">
      <div class="animate-spin rounded-full h-6 w-6 border-2 border-amber-500 border-t-transparent"></div>
    </div>
  {:else if $positions.length === 0}
    <p class="text-gray-500 text-sm text-center py-6">لا توجد مراكز مفتوحة</p>
  {:else}
    <div class="overflow-x-auto">
      <table class="w-full text-xs">
        <thead>
          <tr class="text-gray-400 border-b border-dark-600">
            <th class="pb-2 px-2">الزوج</th>
            <th class="pb-2 px-2">الاتجاه</th>
            <th class="pb-2 px-2">الكمية</th>
            <th class="pb-2 px-2">سعر الدخول</th>
            <th class="pb-2 px-2">الحالي</th>
            <th class="pb-2 px-2">التصفية</th>
            <th class="pb-2 px-2">الهامش</th>
            <th class="pb-2 px-2">الرافعة</th>
            <th class="pb-2 px-2">TP/SL</th>
            <th class="pb-2 px-2">PnL</th>
            <th class="pb-2 px-2">ROE%</th>
            <th class="pb-2 px-2"></th>
          </tr>
        </thead>
        <tbody>
          {#each $positions as pos}
            <tr class="hover:bg-dark-700/30 border-b border-dark-700/50">
              <td class="py-2 px-2 font-medium">{pos.pair}</td>
              <td class="py-2 px-2">
                <span class:badge-green={pos.side === 'long'} class:badge-red={pos.side === 'short'}>
                  {pos.side === 'long' ? 'شراء' : 'بيع'}
                </span>
              </td>
              <td class="py-2 px-2">{fmt(pos.quantity, 4)}</td>
              <td class="py-2 px-2">{fmt(pos.entry_price)}</td>
              <td class="py-2 px-2">{fmt(pos.mark_price)}</td>
              <td class="py-2 px-2 text-red-400">{fmt(pos.liquidation_price)}</td>
              <td class="py-2 px-2">{fmt(pos.margin)}</td>
              <td class="py-2 px-2">{pos.leverage}x</td>
              <td class="py-2 px-2">
                {#if editingTpSl[pos.id]}
                  <div class="flex gap-1 items-center">
                    <input type="number" step="0.01" class="w-14 p-0.5 text-[10px] bg-dark-700 rounded border border-dark-500 text-center"
                      bind:value={editingTpSl[pos.id].tp} placeholder="TP" />
                    <span class="text-gray-600">/</span>
                    <input type="number" step="0.01" class="w-14 p-0.5 text-[10px] bg-dark-700 rounded border border-dark-500 text-center"
                      bind:value={editingTpSl[pos.id].sl} placeholder="SL" />
                    <button class="text-[10px] text-amber-400 hover:text-amber-300" on:click={() => saveTpSl(pos.id)}>حفظ</button>
                  </div>
                {:else}
                  <button class="text-[10px] text-gray-400 hover:text-amber-400" on:click={() => startEditTpSl(pos)}>
                    {pos.take_profit || pos.stop_loss
                      ? `TP: ${pos.take_profit ? fmt(pos.take_profit) : '—'} / SL: ${pos.stop_loss ? fmt(pos.stop_loss) : '—'}`
                      : 'تعيين'}
                  </button>
                {/if}
              </td>
              <td class="py-2 px-2 font-medium" class:text-emerald-400={parseFloat(pos.unrealized_pnl) >= 0} class:text-red-400={parseFloat(pos.unrealized_pnl) < 0}>
                {parseFloat(pos.unrealized_pnl) >= 0 ? '+' : ''}{fmt(pos.unrealized_pnl)}
              </td>
              <td class="py-2 px-2 font-medium" class:text-emerald-400={calcROE(pos) >= 0} class:text-red-400={calcROE(pos) < 0}>
                {calcROE(pos) >= 0 ? '+' : ''}{calcROE(pos).toFixed(2)}%
              </td>
              <td class="py-2 px-2">
                <div class="flex gap-1 items-center">
                  <input type="number" step="0.001" class="w-14 p-0.5 text-[10px] bg-dark-700 rounded border border-dark-500 text-center"
                    bind:value={partialQty[pos.id]} placeholder="جزئي" />
                  <button class="text-[10px] text-blue-400 hover:text-blue-300" on:click={() => partialClose(pos.id)}>جزء</button>
                  <button class="text-[10px] bg-red-600/20 text-red-400 hover:bg-red-600/40 px-1.5 py-0.5 rounded"
                    on:click={() => closePosition(pos.id)}>إغلاق</button>
                </div>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {/if}
</div>
