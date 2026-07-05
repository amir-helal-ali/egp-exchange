<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { api, getToken } from '$lib/api';
  import type { TradingPair } from '$lib/api';
  import { toasts } from '$lib/stores/toast';

  let pairs: TradingPair[] = [];
  let loading = true;
  let showModal = false;
  let editing = false;
  let editId = 0;
  let deleting = '';

  let form: Partial<TradingPair> = {
    base_currency: '',
    quote_currency: '',
    maker_fee: '0.001',
    taker_fee: '0.002',
    max_leverage: 1,
    futures_enabled: false,
    spot_enabled: true,
    is_active: true,
  };

  onMount(() => {
    if (!getToken()) goto('/login');
    fetchPairs();
  });

  async function fetchPairs() {
    loading = true;
    try {
      pairs = await api.get<TradingPair[]>('/admin/pairs');
    } catch (e: unknown) {
      toasts.add(e instanceof Error ? e.message : 'فشل تحميل أزواج التداول', 'error');
    }
    loading = false;
  }

  function openAdd() {
    form = { base_currency: '', quote_currency: '', maker_fee: '0.001', taker_fee: '0.002', max_leverage: 1, futures_enabled: false, spot_enabled: true, is_active: true };
    editing = false;
    editId = 0;
    showModal = true;
  }

  function openEdit(p: TradingPair) {
    form = { ...p };
    editing = true;
    editId = p.id;
    showModal = true;
  }

  function closeModal() {
    showModal = false;
  }

  async function handleSave() {
    if (!form.base_currency || !form.quote_currency) {
      toasts.add('يرجى إدخال زوج التداول', 'error');
      return;
    }
    try {
      if (editing) {
        await api.put<TradingPair>(`/admin/pairs/${editId}`, form);
        toasts.add('تم تحديث زوج التداول بنجاح', 'success');
      } else {
        await api.post<TradingPair>('/admin/pairs', form);
        toasts.add('تمت إضافة زوج التداول بنجاح', 'success');
      }
      closeModal();
      await fetchPairs();
    } catch (e: unknown) {
      toasts.add(e instanceof Error ? e.message : 'فشل الحفظ', 'error');
    }
  }

  async function handleDelete(id: number) {
    const key = String(id);
    if (deleting === key) {
      try {
        await api.del(`/admin/pairs/${id}`);
        pairs = pairs.filter(p => p.id !== id);
        toasts.add('تم حذف زوج التداول بنجاح', 'success');
      } catch (e: unknown) {
        toasts.add(e instanceof Error ? e.message : 'فشل الحذف', 'error');
      }
      deleting = '';
    } else {
      deleting = key;
      setTimeout(() => { if (deleting === key) deleting = ''; }, 3000);
    }
  }

  async function toggleField(id: number, field: 'futures_enabled' | 'spot_enabled' | 'is_active') {
    try {
      const pair = pairs.find(p => p.id === id);
      if (!pair) return;
      await api.put<TradingPair>(`/admin/pairs/${id}`, { ...pair, [field]: !pair[field] });
      pairs = pairs.map(p => p.id === id ? { ...p, [field]: !p[field] } : p);
      toasts.add('تم التبديل بنجاح', 'success');
    } catch (e: unknown) {
      toasts.add(e instanceof Error ? e.message : 'فشل التبديل', 'error');
    }
  }

  function formatPair(base: string, quote: string) {
    return `${base}/${quote}`;
  }
</script>

<div class="animate-fade-in">
  <div class="flex items-center justify-between mb-6">
    <h1 class="text-2xl font-bold">أزواج التداول</h1>
    <div class="flex gap-2">
      <button class="btn-ghost btn-sm flex items-center gap-1.5" on:click={fetchPairs} disabled={loading}>
        <svg class="w-4 h-4" class:animate-spin={loading} fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"/></svg>
        تحديث
      </button>
      <button class="btn-primary btn-sm" on:click={openAdd}>+ إضافة زوج</button>
    </div>
  </div>

  {#if loading && pairs.length === 0}
    <div class="flex justify-center py-12">
      <div class="animate-spin rounded-full h-8 w-8 border-2 border-amber-500 border-t-transparent"></div>
    </div>
  {:else if pairs.length === 0}
    <div class="card text-center py-10">
      <p class="text-gray-500">لا توجد أزواج تداول</p>
    </div>
  {:else}
    <div class="card-table overflow-x-auto">
      <table class="min-w-[1000px]">
        <thead>
          <tr class="border-b border-dark-600">
            <th>الزوج</th>
            <th>رسوم الصانع</th>
            <th>رسوم الآخذ</th>
            <th>الرافعة</th>
            <th>العقود الآجلة</th>
            <th>التداول الفوري</th>
            <th>نشط</th>
            <th>الإجراءات</th>
          </tr>
        </thead>
        <tbody>
          {#each pairs as p}
            <tr class="border-b border-dark-700 hover:bg-dark-700/50 transition-colors">
              <td class="font-mono font-bold text-amber-400">{formatPair(p.base_currency, p.quote_currency)}</td>
              <td class="font-mono text-xs">{(parseFloat(p.maker_fee) * 100).toFixed(2)}%</td>
              <td class="font-mono text-xs">{(parseFloat(p.taker_fee) * 100).toFixed(2)}%</td>
              <td>{p.max_leverage}x</td>
              <td>
                <button class="toggle {p.futures_enabled ? 'toggle-on' : 'toggle-off'}" on:click={() => toggleField(p.id, 'futures_enabled')}>
                  <span class="toggle-dot {p.futures_enabled ? 'toggle-dot-on' : 'toggle-dot-off'}"></span>
                </button>
              </td>
              <td>
                <button class="toggle {p.spot_enabled ? 'toggle-on' : 'toggle-off'}" on:click={() => toggleField(p.id, 'spot_enabled')}>
                  <span class="toggle-dot {p.spot_enabled ? 'toggle-dot-on' : 'toggle-dot-off'}"></span>
                </button>
              </td>
              <td>
                <span class="badge {p.is_active ? 'badge-active' : 'badge-inactive'}">{p.is_active ? 'نشط' : 'غير نشط'}</span>
              </td>
              <td>
                <div class="flex gap-1.5">
                  <button class="btn-ghost btn-xs" on:click={() => openEdit(p)}>تعديل</button>
                  <button
                    class="btn-xs {deleting === String(p.id) ? 'btn-danger' : 'btn-ghost'}"
                    on:click={() => handleDelete(p.id)}
                  >
                    {deleting === String(p.id) ? 'تأكيد' : 'حذف'}
                  </button>
                </div>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {/if}
</div>

{#if showModal}
  <div class="modal-overlay animate-fade-in" on:click|self={closeModal}>
    <div class="modal-content" on:click|stopPropagation>
      <div class="flex items-center justify-between mb-6">
        <h2 class="text-lg font-semibold">{editing ? 'تعديل زوج التداول' : 'إضافة زوج تداول جديد'}</h2>
        <button class="text-gray-400 hover:text-white" on:click={closeModal}>
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/></svg>
        </button>
      </div>
      <form on:submit|preventDefault={handleSave} class="space-y-4">
        <div class="grid grid-cols-2 gap-4">
          <div>
            <label class="label">عملة الأساس *</label>
            <input class="input" bind:value={form.base_currency} placeholder="BTC" required maxlength="10" />
          </div>
          <div>
            <label class="label">عملة المقابلة *</label>
            <input class="input" bind:value={form.quote_currency} placeholder="USDT" required maxlength="10" />
          </div>
        </div>
        <div class="grid grid-cols-3 gap-4">
          <div>
            <label class="label">رسوم الصانع</label>
            <input class="input" type="number" step="0.000001" bind:value={form.maker_fee} />
          </div>
          <div>
            <label class="label">رسوم الآخذ</label>
            <input class="input" type="number" step="0.000001" bind:value={form.taker_fee} />
          </div>
          <div>
            <label class="label">أقصى رافعة</label>
            <input class="input" type="number" min="1" max="125" bind:value={form.max_leverage} />
          </div>
        </div>
        <div class="flex gap-6">
          <label class="flex items-center gap-2 text-sm">
            <input type="checkbox" bind:checked={form.spot_enabled} class="rounded bg-dark-700 border-dark-500 text-amber-600 focus:ring-amber-500" />
            تداول فوري
          </label>
          <label class="flex items-center gap-2 text-sm">
            <input type="checkbox" bind:checked={form.futures_enabled} class="rounded bg-dark-700 border-dark-500 text-amber-600 focus:ring-amber-500" />
            عقود آجلة
          </label>
          <label class="flex items-center gap-2 text-sm">
            <input type="checkbox" bind:checked={form.is_active} class="rounded bg-dark-700 border-dark-500 text-amber-600 focus:ring-amber-500" />
            نشط
          </label>
        </div>
        <div class="flex gap-3 pt-2">
          <button type="submit" class="btn-primary">{editing ? 'حفظ التغييرات' : 'إضافة'}</button>
          <button type="button" class="btn-ghost" on:click={closeModal}>إلغاء</button>
        </div>
      </form>
    </div>
  </div>
{/if}
