<script lang="ts">
  import { goto } from '$app/navigation';
  import { api, setToken, getToken } from '$lib/api';
  import type { LoginResponse } from '$lib/api';

  let email = '';
  let password = '';
  let error = '';
  let submitting = false;

  if (getToken()) goto('/');

  async function handleSubmit() {
    submitting = true;
    error = '';
    try {
      const res = await api.post<LoginResponse>('/auth/login', { email, password });
      if (res.user.role !== 'admin') {
        error = 'وصول مرفوض. هذه اللوحة مخصصة للمشرفين فقط.';
        submitting = false;
        return;
      }
      setToken(res.token);
      goto('/');
    } catch (e: unknown) {
      error = e instanceof Error ? e.message : 'فشل تسجيل الدخول';
    } finally {
      submitting = false;
    }
  }
</script>

<div class="min-h-screen flex items-center justify-center bg-dark-900 px-4">
  <div class="w-full max-w-sm">
    <div class="text-center mb-8">
      <h1 class="text-3xl font-bold text-amber-400">EGP<span class="text-gray-400">EX</span></h1>
      <p class="text-gray-500 mt-2">لوحة تحكم المشرفين</p>
    </div>

    <form on:submit|preventDefault={handleSubmit} class="bg-dark-800 border border-dark-600 rounded-xl p-6 shadow-lg space-y-5">
      {#if error}
        <div class="bg-red-900/50 border border-red-700 text-red-300 rounded-lg p-3 text-sm">{error}</div>
      {/if}

      <div>
        <label class="label" for="email">البريد الإلكتروني</label>
        <input id="email" type="email" class="input" bind:value={email} placeholder="admin@example.com" required />
      </div>

      <div>
        <label class="label" for="password">كلمة المرور</label>
        <input id="password" type="password" class="input" bind:value={password} placeholder="••••••••" required />
      </div>

      <button type="submit" class="btn-primary w-full py-2.5" disabled={submitting}>
        {submitting ? 'جاري تسجيل الدخول...' : 'تسجيل الدخول'}
      </button>
    </form>
  </div>
</div>
