<script lang="ts">
  import { goto } from '$app/navigation';
  import { currentUser } from '$lib/stores/exchange';
  import { api, setToken } from '$lib/api';

  let email = '';
  let password = '';
  let error = '';
  let submitting = false;

  async function handleSubmit() {
    submitting = true;
    error = '';
    try {
      const res = await api.post<{ token: string; user: import('$lib/api').User }>('/auth/login', { email, password });
      setToken(res.token);
      currentUser.set(res.user);
      goto('/dashboard');
    } catch (e: unknown) {
      error = e instanceof Error ? e.message : 'فشل تسجيل الدخول';
    } finally {
      submitting = false;
    }
  }
</script>

<div class="min-h-[calc(100vh-3.5rem)] flex items-center justify-center px-4">
  <div class="w-full max-w-sm animate-fade-in">
    <div class="text-center mb-8">
      <h1 class="text-3xl font-bold mb-2"><span class="text-amber-400">EGP</span><span class="text-gray-400">EX</span></h1>
      <p class="text-gray-400 text-sm">تسجيل الدخول إلى حسابك</p>
    </div>

    <form on:submit|preventDefault={handleSubmit} class="card space-y-4">
      {#if error}
        <div class="bg-red-900/50 border border-red-700 text-red-300 rounded-lg p-3 text-sm">{error}</div>
      {/if}

      <div>
        <label class="label">البريد الإلكتروني</label>
        <input type="email" class="input" placeholder="example@email.com" bind:value={email} required />
      </div>

      <div>
        <label class="label">كلمة المرور</label>
        <input type="password" class="input" placeholder="••••••••" bind:value={password} required />
      </div>

      <button type="submit" class="btn-gold w-full py-2.5" disabled={submitting}>
        {submitting ? 'جاري تسجيل الدخول...' : 'تسجيل الدخول'}
      </button>

      <p class="text-center text-sm text-gray-400">
        ليس لديك حساب؟ <a href="/register" class="text-amber-400 hover:text-amber-300 transition-colors">إنشاء حساب</a>
      </p>
    </form>
  </div>
</div>
