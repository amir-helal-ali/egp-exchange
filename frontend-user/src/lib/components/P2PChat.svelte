<script lang="ts">
  import { onMount, onDestroy, afterUpdate } from 'svelte';
  import { api, type P2pMessage, type SendMessageRequest } from '$lib/api';
  import { getWs } from '$lib/ws';

  export let orderId: string;

  let messages: P2pMessage[] = [];
  let newMessage = '';
  let chatEl: HTMLDivElement;
  let loading = true;

  function scrollToBottom() {
    if (chatEl) {
      chatEl.scrollTop = chatEl.scrollHeight;
    }
  }

  onMount(async () => {
    try {
      messages = await api.get<P2pMessage[]>(`/p2p/orders/${orderId}/messages`);
    } catch {}
    loading = false;
    afterUpdate(scrollToBottom);

    const ws = getWs();
    ws.subscribe(`p2p_orders:${orderId}`, (data: unknown) => {
      const msg = data as P2pMessage;
      messages = [...messages, msg];
      afterUpdate(scrollToBottom);
    });
    ws.connect();
  });

  onDestroy(() => {
    const ws = getWs();
    ws.unsubscribe(`p2p_orders:${orderId}`);
  });

  async function send() {
    if (!newMessage.trim()) return;
    try {
      const body: SendMessageRequest = { message: newMessage };
      const msg = await api.post<P2pMessage>(`/p2p/orders/${orderId}/messages`, body);
      messages = [...messages, msg];
      newMessage = '';
      afterUpdate(scrollToBottom);
    } catch {}
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      send();
    }
  }
</script>

<div class="space-y-3">
  <div bind:this={chatEl} class="h-48 overflow-y-auto space-y-2 px-2">
    {#if loading}
      <div class="flex justify-center py-4">
        <div class="animate-spin rounded-full h-5 w-5 border-2 border-amber-500 border-t-transparent"></div>
      </div>
    {:else if messages.length === 0}
      <p class="text-gray-500 text-xs text-center py-4">لا توجد رسائل بعد</p>
    {:else}
      {#each messages as msg}
        <div class="flex flex-col" class:items-start={msg.sender_id !== orderId} class:items-end={msg.sender_id === orderId}>
          <div class="max-w-[80%] rounded-lg px-3 py-2 text-xs"
            class:bg-dark-700:text-gray-200={msg.sender_id !== orderId}
            class:bg-amber-600:text-white={msg.sender_id === orderId}>
            <p>{msg.message}</p>
            <span class="block text-[10px] opacity-60 mt-1">{new Date(msg.created_at).toLocaleTimeString('ar-EG')}</span>
          </div>
        </div>
      {/each}
    {/if}
  </div>

  <div class="flex gap-2">
    <input type="text" class="input text-xs flex-1" placeholder="اكتب رسالة..." bind:value={newMessage} on:keydown={handleKeydown} />
    <button class="btn-primary text-xs px-3" on:click={send}>إرسال</button>
  </div>
</div>
