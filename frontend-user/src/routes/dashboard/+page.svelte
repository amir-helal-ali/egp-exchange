<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { isAuthenticated, wallets, positions, p2pOrders } from '$lib/stores/exchange';
  import { api, type Order, type Wallet } from '$lib/api';
  import PositionsList from '$lib/components/PositionsList.svelte';

  let orders: Order[] = [];
  let loading = true;

  onMount(async () => {
    if (!$isAuthenticated) goto('/login');
    try {
      const [w, o] = await Promise.all([
        api.get<Wallet[]>('/wallets'),
        api.get<Order[]>('/orders'),
      ]);
      wallets.set(w);
      orders = o;
      try {
        const pos = await api.get<import('$lib/api').FuturesPosition[]>('/futures/positions');
        positions.set(pos);
      } catch {}
      try {
        const p2p = await api.get<import('$lib/api').P2pOrder[]>('/p2p/orders');
        p2pOrders.set(p2p);
      } catch {}
    } catch {}
    loading = false;
  });

  function fmt(s: string, dp: number = 2) {
    return parseFloat(s).toLocaleString('en-US', { minimumFractionDigits: dp, maximumFractionDigits: dp });
  }

  $: totalEgp = $wallets
    .filter(w => w.currency === 'EGP')
    .reduce((sum, w) => sum + parseFloat(w.balance), 0);
  $: cryptoWallets = $wallets.filter(w => w.currency !== 'EGP');
  $: activeP2pOrders = $p2pOrders.filter(o => o.status === 'pending' || o.status === 'paid');
</script>

<div class="max-w-7xl mx-auto p-4 space-y-6 animate-fade-in">
  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-bold">لوحة التحكم</h1>
    <span class="text-xs text-gray-500">مرحباً بك في EGPEX</span>
  </div>

  {#if loading}
    <div class="flex justify-center py-16">
      <div class="animate-spin rounded-full h-8 w-8 border-2 border-amber-500 border-t-transparent"></div>
    </div>
  {:else}
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
      <div class="card border-amber-500/20">
        <p class="text-xs text-gray-400 mb-1">رصيد الجنيه المصري</p>
        <p class="text-2xl font-bold text-amber-400">{totalEgp.toFixed(2)} <span class="text-sm font-normal text-gray-500">EGP</span></p>
      </div>
      {#each cryptoWallets as wallet}
        <div class="card">
          <p class="text-xs text-gray-400 mb-1">رصيد {wallet.currency}</p>
          <p class="text-2xl font-bold">{fmt(wallet.balance, 4)} <span class="text-sm font-normal text-gray-500">{wallet.currency}</span></p>
          <p class="text-xs text-gray-500 mt-1">المجمد: {fmt(wallet.locked, 4)}</p>
        </div>
      {/each}
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
      <div class="card">
        <div class="flex items-center justify-between mb-4">
          <h2 class="text-lg font-semibold">الطلبات الأخيرة</h2>
          <a href="/trade" class="text-xs text-amber-400 hover:text-amber-300 transition-colors">طلب جديد</a>
        </div>
        {#if orders.length === 0}
          <p class="text-gray-500 text-sm py-6 text-center">لا توجد طلبات بعد</p>
        {:else}
          <div class="overflow-x-auto">
            <table class="w-full text-sm">
              <thead>
                <tr class="text-gray-400 border-b border-dark-600">
                  <th class="pb-2">الزوج</th>
                  <th class="pb-2">النوع</th>
                  <th class="pb-2">السعر</th>
                  <th class="pb-2">الكمية</th>
                  <th class="pb-2">الحالة</th>
                </tr>
              </thead>
              <tbody>
                {#each orders.slice(0, 5) as order}
                  <tr class="hover:bg-dark-700/30">
                    <td class="py-2">{order.base_currency}/{order.quote_currency}</td>
                    <td class="py-2">
                      <span class:text-emerald-400={order.side === 'buy'} class:text-red-400={order.side === 'sell'}>
                        {order.side === 'buy' ? 'شراء' : 'بيع'}
                      </span>
                    </td>
                    <td class="py-2">{order.price ? fmt(order.price) : 'سوقي'}</td>
                    <td class="py-2">{fmt(order.quantity, 4)}</td>
                    <td class="py-2">
                      <span class:badge-green={order.status === 'filled'} class:badge-yellow={order.status === 'open' || order.status === 'partial'} class:badge-red={order.status === 'cancelled'}>
                        {order.status === 'filled' ? 'منفذ' : order.status === 'open' ? 'مفتوح' : order.status === 'partial' ? 'جزئي' : order.status === 'cancelled' ? 'ملغي' : order.status}
                      </span>
                    </td>
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
        {/if}
      </div>

      <div class="card">
        <div class="flex items-center justify-between mb-4">
          <h2 class="text-lg font-semibold">طلبات P2P النشطة</h2>
          <a href="/p2p/orders" class="text-xs text-amber-400 hover:text-amber-300 transition-colors">عرض الكل</a>
        </div>
        {#if activeP2pOrders.length === 0}
          <p class="text-gray-500 text-sm py-6 text-center">لا توجد طلبات P2P نشطة</p>
        {:else}
          {#each activeP2pOrders as order}
            <div class="flex items-center justify-between py-2 border-b border-dark-700 last:border-0">
              <div>
                <span class="text-sm">{order.base_currency}/{order.quote_currency}</span>
                <span class="text-xs text-gray-500 mr-2">{parseFloat(order.quantity).toFixed(4)}</span>
              </div>
              <span class:badge-yellow={order.status === 'pending'} class:badge-blue={order.status === 'paid'}>
                {order.status === 'pending' ? 'قيد الانتظار' : 'تم الدفع'}
              </span>
            </div>
          {/each}
        {/if}
      </div>
    </div>

    <PositionsList />
  {/if}
</div>
