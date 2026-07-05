<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { api, getToken } from '$lib/api';
  import type { Currency } from '$lib/api';
  import { toasts } from '$lib/stores/toast';

  let currencies: Currency[] = [];
  let loading = true;
  let showModal = false;
  let editing = false;
  let editCode = '';
  let deleting = '';

  let form: Partial<Currency> = {
    code: '',
    name: '',
    name_ar: '',
    type: 'crypto',
    decimals: 8,
    withdrawal_fee: '0',
    min_withdrawal: '0',
    deposit_enabled: true,
    withdrawal_enabled: true,
  };

  onMount(() => {
    if (!getToken()) goto('/login');
    fetchCurrencies();
  });

  async function fetchCurrencies() {
    loading = true;
    try {
      currencies = await api.get<Currency[]>('/admin/currencies');
    } catch (e: unknown) {
      toasts.add(e instanceof Error ? e.message : 'فشل تحميل العملات', 'error');
    }
    loading = false;
  }

  function openAdd() {
    form = { code: '', name: '', name_ar: '', type: 'crypto', decimals: 8, withdrawal_fee: '0', min_withdrawal: '0', deposit_enabled: true, withdrawal_enabled: true };
    editing = false;
    editCode = '';
    showModal = true;
  }

  function openEdit(c: Currency) {
    form = { ...c };
    editing = true;
    editCode = c.code;
    showModal = true;
  }

  function closeModal() {
    showModal = false;
  }

  async function handleSave() {
    if (!form.code || !form.name) {
      toasts.add('يرجى ملء الحقول المطلوبة', 'error');
      return;
    }
    try {
      if (editing) {
        await api.put<Currency>(`/admin/currencies/${editCode}`, form);
        toasts.add('تم تحديث العملة بنجاح', 'success');
      } else {
        await api.post<Currency>('/admin/currencies', form);
        toasts.add('تمت إضافة العملة بنجاح', 'success');
      }
      closeModal();
      await fetchCurrencies();
    } catch (e: unknown) {
      toasts.add(e instanceof Error ? e.message : 'فشل الحفظ', 'error');
    }
  }

  async function handleDelete(code: string) {
    if (deleting === code) {
      try {
        await api.del(`/admin/currencies/${code}`);
        currencies = currencies.filter(c => c.code !== code);
        toasts.add('تم حذف العملة بنجاح', 'success');
      } catch (e: unknown) {
        toasts.add(e instanceof Error ? e.message : 'فشل الحذف', 'error');
      }
      deleting = '';
    } else {
      deleting = code;
      setTimeout(() => { if (deleting === code) deleting = ''; }, 3000);
    }
  }

  async function toggleField(code: string, field: 'deposit_enabled' | 'withdrawal_enabled') {
    try {
      await api.post(`/admin/currencies/${code}/toggle`, { field });
      currencies = currencies.map(c => c.code === code ? { ...c, [field]: !c[field] } : c);
      toasts.add('تم التبديل بنجاح', 'success');
    } catch (e: unknown) {
      toasts.add(e instanceof Error ? e.message : 'فشل التبديل', 'error');
    }
  }
</script>

<div class="animate-fade-in">
  <div class="flex items-center justify-between mb-6">
    <h1 class="text-2xl font-bold">إدارة العملات</h1>
    <div class="flex gap-2">
      <button class="btn-ghost btn-sm flex items-center gap-1.5" on:click={fetchCurrencies} disabled={loading}>
        <svg class="w-4 h-4" class:animate-spin={loading} fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"/></svg>
        تحديث
      </button>
      <button class="btn-primary btn-sm" on:click={openAdd}>+ إضافة عملة</button>
    </div>
  </div>

  {#if loading && currencies.length === 0}
    <div class="flex justify-center py-12">
      <div class="animate-spin rounded-full h-8 w-8 border-2 border-amber-500 border-t-transparent"></div>
    </div>
  {:else if currencies.length === 0}
    <div class="card text-center py-10">
      <p class="text-gray-500">لا توجد عملات</p>
    </div>
  {:else}
    <div class="card-table overflow-x-auto">
      <table class="min-w-[900px]">
        <thead>
          <tr class="border-b border-dark-600">
            <th>الرمز</th>
            <th>الاسم</th>
            <th>الاسم عربي</th>
            <th>النوع</th>
            <th>الخانات</th>
            <th>رسوم السحب</th>
            <th>الحد الأدنى</th>
            <th>الإيداع</th>
            <th>السحب</th>
            <th>الإجراءات</th>
          </tr>
        </thead>
        <tbody>
          {#each currencies as c}
            <tr class="border-b border-dark-700 hover:bg-dark-700/50 transition-colors">
              <td class="font-mono font-bold text-amber-400">{c.code}</td>
              <td>{c.name}</td>
              <td class="text-gray-400">{c.name_ar}</td>
              <td><span class="badge {c.type === 'crypto' ? 'bg-blue-900/50 text-blue-300 border-blue-700' : 'bg-green-900/50 text-green-300 border-green-700'}">{c.type === 'crypto' ? 'رقمية' : 'نقدية'}</span></td>
              <td class="text-xs">{c.decimals}</td>
              <td class="font-mono text-xs">{c.withdrawal_fee}</td>
              <td class="font-mono text-xs">{c.min_withdrawal}</td>
              <td>
                <button class="toggle {c.deposit_enabled ? 'toggle-on' : 'toggle-off'}" on:click={() => toggleField(c.code, 'deposit_enabled')}>
                  <span class="toggle-dot {c.deposit_enabled ? 'toggle-dot-on' : 'toggle-dot-off'}"></span>
                </button>
              </td>
              <td>
                <button class="toggle {c.withdrawal_enabled ? 'toggle-on' : 'toggle-off'}" on:click={() => toggleField(c.code, 'withdrawal_enabled')}>
                  <span class="toggle-dot {c.withdrawal_enabled ? 'toggle-dot-on' : 'toggle-dot-off'}"></span>
                </button>
              </td>
              <td>
                <div class="flex gap-1.5">
                  <button class="btn-ghost btn-xs" on:click={() => openEdit(c)}>تعديل</button>
                  <button
                    class="btn-xs {deleting === c.code ? 'btn-danger' : 'btn-ghost'}"
                    on:click={() => handleDelete(c.code)}
                  >
                    {deleting === c.code ? 'تأكيد' : 'حذف'}
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
        <h2 class="text-lg font-semibold">{editing ? 'تعديل العملة' : 'إضافة عملة جديدة'}</h2>
        <button class="text-gray-400 hover:text-white" on:click={closeModal}>
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/></svg>
        </button>
      </div>
      <form on:submit|preventDefault={handleSave} class="space-y-4">
        <div class="grid grid-cols-2 gap-4">
          <div>
            <label class="label">الرمز *</label>
            <input class="input" bind:value={form.code} placeholder="BTC" required maxlength="10" disabled={editing} />
          </div>
          <div>
            <label class="label">النوع</label>
            <select class="input" bind:value={form.type}>
              <option value="crypto">رقمية</option>
              <option value="fiat">نقدية</option>
            </select>
          </div>
        </div>
        <div class="grid grid-cols-2 gap-4">
          <div>
            <label class="label">الاسم *</label>
            <input class="input" bind:value={form.name} placeholder="Bitcoin" required />
          </div>
          <div>
            <label class="label">الاسم بالعربية</label>
            <input class="input" bind:value={form.name_ar} placeholder="بيتكوين" />
          </div>
        </div>
        <div class="grid grid-cols-3 gap-4">
          <div>
            <label class="label">الخانات العشرية</label>
            <input class="input" type="number" bind:value={form.decimals} min="0" max="18" />
          </div>
          <div>
            <label class="label">رسوم السحب</label>
            <input class="input" type="number" step="any" bind:value={form.withdrawal_fee} />
          </div>
          <div>
            <label class="label">الحد الأدنى للسحب</label>
            <input class="input" type="number" step="any" bind:value={form.min_withdrawal} />
          </div>
        </div>
        <div class="flex gap-6">
          <label class="flex items-center gap-2 text-sm">
            <input type="checkbox" bind:checked={form.deposit_enabled} class="rounded bg-dark-700 border-dark-500 text-amber-600 focus:ring-amber-500" />
            الإيداع مفعّل
          </label>
          <label class="flex items-center gap-2 text-sm">
            <input type="checkbox" bind:checked={form.withdrawal_enabled} class="rounded bg-dark-700 border-dark-500 text-amber-600 focus:ring-amber-500" />
            السحب مفعّل
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
