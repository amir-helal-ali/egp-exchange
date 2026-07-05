<script lang="ts">
  import { toasts } from '$lib/stores/toast';

  function toastClass(type: string) {
    if (type === 'success') return 'bg-green-900/90 text-green-100 border-green-700';
    if (type === 'error') return 'bg-red-900/90 text-red-100 border-red-700';
    return 'bg-blue-900/90 text-blue-100 border-blue-700';
  }
</script>

<div class="fixed top-4 left-1/2 -translate-x-1/2 z-50 flex flex-col gap-2 w-96 pointer-events-none">
  {#each $toasts as toast (toast.id)}
    <div
      class="px-4 py-3 rounded-lg shadow-lg text-sm font-medium border pointer-events-auto animate-slide-down cursor-pointer {toastClass(toast.type)}"
      role="button"
      tabindex="0"
      on:click={() => toasts.dismiss(toast.id)}
      on:keydown={(e) => e.key === 'Enter' && toasts.dismiss(toast.id)}
    >
      {toast.message}
    </div>
  {/each}
</div>
