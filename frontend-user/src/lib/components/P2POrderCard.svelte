<script lang="ts">
  import { api, type P2pOrder } from '$lib/api';
  import { p2pOrders } from '$lib/stores/exchange';
  import P2PChat from './P2PChat.svelte';

  export let order: P2pOrder;

  let showChat = false;
  let submitting = false;
  let error = '';

  async function confirmPaid() {
    submitting = true;
    try {
      await api.post(`/p2p/orders/${order.id}/paid`);
      order.status = 'paid';
      p2pOrders.update(o => o.map(ord => ord.id === order.id ? { ...ord, status: 'paid' } : ord));
    } catch (e: unknown) {
      error = e instanceof Error ? e.message : 'فشل';
    } finally {
      submitting = false;
    }
  }

  async function releaseCrypto() {
    submitting = true;
    try {
      await api.post(`/p2p/orders/${order.id}/release`);
      order.status = 'completed';
      p2pOrders.update(o => o.map(ord => ord.id === order.id ? { ...ord, status: 'completed' } : ord));
    } catch (e: unknown) {
      error = e instanceof Error ? e.message : 'فشل';
    } finally {
      submitting = false;
    }
  }

  async function openDispute() {
    submitting = true;
    try {
      await api.post(`/p2p/orders/${order.id}/dispute`);
      order.status = 'disputed';
      p2pOrders.update(o => o.map(ord => ord.id === order.id ? { ...ord, status: 'disputed' } : ord));
    } catch (e: unknown) {
      error = e instanceof Error ? e.message : 'فشل';
    } finally {
      submitting = false;
    }
  }

  function statusBadge(status: string) {
    switch (status) {
      case 'pending': return 'badge-yellow';
      case 'paid': return 'badge-blue';
      case 'completed': return 'badge-green';
      case 'cancelled': return 'badge-red';
      case 'disputed': return 'badge-red';
      default: return 'badge-gray';
    }
  }

  function statusText(status: string) {
    switch (status) {
      case 'pending': return 'قيد الانتظار';
      case 'paid': return 'تم الدفع';
      case 'completed': return 'مكتملة';
      case 'cancelled': return 'ملغية';
      case 'disputed': return 'نزاع';
      default: return status;
    }
  }
</script>

<div class="card animate-fade-in {order.status === 'pending' ? 'border-amber-500/50' : order.status === 'completed' ? 'border-emerald-500/50' : 'border-red-500/50'}">
  <div class="flex items-start justify-between mb-3">
    <div>
      <div class="flex items-center gap-2 mb-1">
        <span class={statusBadge(order.status)}>{statusText(order.status)}</span>
      </div>
      <span class="text-xs text-gray-500">رقم الطلب: {order.id.slice(0, 8)}...</span>
    </div>
    <div class="text-left">
      <p class="text-lg font-bold text-amber-400">{parseFloat(order.total).toFixed(2)}</p>
      <p class="text-xs text-gray-500">EGP</p>
    </div>
  </div>

  <div class="grid grid-cols-2 gap-3 text-xs mb-4">
    <div>
      <span class="text-gray-500">الكمية:</span>
      <span class="text-gray-200 mr-1">{parseFloat(order.amount).toFixed(4)}</span>
    </div>
    <div class="text-left">
      <span class="text-gray-500">الإجمالي:</span>
      <span class="text-gray-200 mr-1">{parseFloat(order.total).toFixed(2)} EGP</span>
    </div>
  </div>

  {#if error}
    <div class="bg-red-900/50 border border-red-700 text-red-300 rounded-lg p-2 text-xs mb-3">{error}</div>
  {/if}

  <div class="flex flex-wrap gap-2">
    {#if order.status === 'pending'}
      <button class="btn-success text-xs px-3 py-1.5" on:click={confirmPaid} disabled={submitting}>تأكيد الدفع</button>
      <button class="btn-primary text-xs px-3 py-1.5" on:click={releaseCrypto} disabled={submitting}>الإفراج</button>
      <button class="btn-danger text-xs px-3 py-1.5" on:click={openDispute} disabled={submitting}>نزاع</button>
    {/if}
    {#if order.status === 'paid'}
      <button class="btn-primary text-xs px-3 py-1.5" on:click={releaseCrypto} disabled={submitting}>الإفراج عن العملة</button>
      <button class="btn-danger text-xs px-3 py-1.5" on:click={openDispute} disabled={submitting}>نزاع</button>
    {/if}
    <button class="btn-ghost text-xs px-3 py-1.5" on:click={() => showChat = !showChat}>
      {showChat ? 'إخفاء المحادثة' : 'المحادثة'}
    </button>
  </div>

  {#if showChat}
    <div class="mt-4 border-t border-dark-600 pt-4">
      <P2PChat orderId={order.id} />
    </div>
  {/if}
</div>
