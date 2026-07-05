import { writable } from 'svelte/store';

export interface Toast {
  id: string;
  message: string;
  type: 'success' | 'error' | 'info';
}

function createToastStore() {
  const { subscribe, update } = writable<Toast[]>([]);
  return {
    subscribe,
    add(message: string, type: 'success' | 'error' | 'info' = 'info') {
      const id = Math.random().toString(36).slice(2, 10);
      update(t => [...t, { id, message, type }]);
      setTimeout(() => {
        update(t => t.filter(x => x.id !== id));
      }, 4000);
    },
    dismiss(id: string) {
      update(t => t.filter(x => x.id !== id));
    }
  };
}

export const toasts = createToastStore();
