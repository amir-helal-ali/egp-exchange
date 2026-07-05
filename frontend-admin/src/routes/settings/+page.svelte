<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { api, getToken } from '$lib/api';
  import type { SystemSetting } from '$lib/api';
  import { toasts } from '$lib/stores/toast';

  let settings: SystemSetting[] = [];
  let loading = true;
  let editingKey = '';
  let editValue = '';
  let showAdd = false;
  let newKey = '';
  let newValue = '';
  let newDescription = '';

  onMount(() => {
    if (!getToken()) goto('/login');
    fetchSettings();
  });

  async function fetchSettings() {
    loading = true;
    try {
      settings = await api.get<SystemSetting[]>('/admin/settings');
    } catch (e: unknown) {
      toasts.add(e instanceof Error ? e.message : 'فشل تحميل الإعدادات', 'error');
    }
    loading = false;
  }

  function startEdit(s: SystemSetting) {
    editingKey = s.key;
    editValue = s.value;
  }

  function cancelEdit() {
    editingKey = '';
    editValue = '';
  }

  async function saveEdit(key: string) {
    try {
      const updated = await api.put<SystemSetting>(`/admin/settings/${key}`, { value: editValue });
      settings = settings.map(s => s.key === key ? updated : s);
      toasts.add('تم حفظ الإعداد بنجاح', 'success');
      editingKey = '';
    } catch (e: unknown) {
      toasts.add(e instanceof Error ? e.message : 'فشل الحفظ', 'error');
    }
  }

  async function addSetting() {
    if (!newKey || !newValue) {
      toasts.add('يرجى إدخال المفتاح والقيمة', 'error');
      return;
    }
    try {
      const created = await api.put<SystemSetting>(`/admin/settings/${newKey}`, { value: newValue });
      settings = [...settings, created];
      toasts.add('تمت إضافة الإعداد بنجاح', 'success');
      showAdd = false;
      newKey = '';
      newValue = '';
      newDescription = '';
    } catch (e: unknown) {
      toasts.add(e instanceof Error ? e.message : 'فشل الإضافة', 'error');
    }
  }

  function formatJSON(val: string) {
    try {
      return JSON.stringify(JSON.parse(val), null, 2);
    } catch {
      return val;
    }
  }
</script>

<div class="animate-fade-in">
  <div class="flex items-center justify-between mb-6">
    <h1 class="text-2xl font-bold">إعدادات النظام</h1>
    <div class="flex gap-2">
      <button class="btn-ghost btn-sm flex items-center gap-1.5" on:click={fetchSettings} disabled={loading}>
        <svg class="w-4 h-4" class:animate-spin={loading} fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"/></svg>
        تحديث
      </button>
      <button class="btn-primary btn-sm" on:click={() => showAdd = true}>+ إضافة إعداد</button>
    </div>
  </div>

  {#if loading && settings.length === 0}
    <div class="flex justify-center py-12">
      <div class="animate-spin rounded-full h-8 w-8 border-2 border-amber-500 border-t-transparent"></div>
    </div>
  {:else if settings.length === 0}
    <div class="card text-center py-10">
      <p class="text-gray-500">لا توجد إعدادات</p>
    </div>
  {:else}
    <div class="space-y-3">
      {#each settings as s}
        <div class="card">
          <div class="flex items-start justify-between">
            <div class="flex-1 ml-4">
              <div class="flex items-center gap-2 mb-1">
                <code class="text-xs font-mono bg-dark-700 px-2 py-0.5 rounded text-amber-400">{s.key}</code>
                {#if s.description}
                  <span class="text-xs text-gray-500">{s.description}</span>
                {/if}
              </div>
              {#if editingKey === s.key}
                <textarea
                  class="input font-mono text-xs mt-2 min-h-[100px]"
                  bind:value={editValue}
                ></textarea>
                <div class="flex gap-2 mt-2">
                  <button class="btn-success btn-xs" on:click={() => saveEdit(s.key)}>حفظ</button>
                  <button class="btn-ghost btn-xs" on:click={cancelEdit}>إلغاء</button>
                </div>
              {:else}
                <pre class="text-xs text-gray-300 font-mono bg-dark-900/50 rounded p-2 mt-1 overflow-x-auto max-h-32 whitespace-pre-wrap">{formatJSON(s.value)}</pre>
              {/if}
            </div>
            <button class="btn-ghost btn-xs shrink-0" on:click={() => startEdit(s)}>تعديل</button>
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>

{#if showAdd}
  <div class="modal-overlay animate-fade-in" on:click|self={() => showAdd = false}>
    <div class="modal-content" on:click|stopPropagation>
      <div class="flex items-center justify-between mb-6">
        <h2 class="text-lg font-semibold">إضافة إعداد جديد</h2>
        <button class="text-gray-400 hover:text-white" on:click={() => showAdd = false}>
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/></svg>
        </button>
      </div>
      <form on:submit|preventDefault={addSetting} class="space-y-4">
        <div>
          <label class="label">المفتاح *</label>
          <input class="input" bind:value={newKey} placeholder="setting_key" required />
        </div>
        <div>
          <label class="label">القيمة * (JSON)</label>
          <textarea class="input font-mono text-xs min-h-[120px]" bind:value={newValue} placeholder="أدخل القيمة بصيغة JSON" required></textarea>
        </div>
        <div>
          <label class="label">الوصف</label>
          <input class="input" bind:value={newDescription} placeholder="وصف الإعداد" />
        </div>
        <div class="flex gap-3 pt-2">
          <button type="submit" class="btn-primary">إضافة</button>
          <button type="button" class="btn-ghost" on:click={() => showAdd = false}>إلغاء</button>
        </div>
      </form>
    </div>
  </div>
{/if}
