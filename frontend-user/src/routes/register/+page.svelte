<script lang="ts">
  import { goto } from '$app/navigation';
  import { currentUser } from '$lib/stores/exchange';
  import { api, setToken } from '$lib/api';

  let email = '';
  let password = '';
  let confirm = '';
  let error = '';
  let submitting = false;

  async function handleSubmit() {
    submitting = true;
    error = '';
    if (password !== confirm) {
      error = 'كلمة المرور غير متطابقة';
      submitting = false;
      return;
    }
    if (password.length < 8) {
      error = 'كلمة المرور يجب أن تكون 8 أحرف على الأقل';
      submitting = false;
      return;
    }
    try {
      const res = await api.post<{ token: string; user: import('$lib/api').User }>('/auth/register', { email, password });
      setToken(res.token);
      currentUser.set(res.user);
      goto('/dashboard');
    } catch (e: unknown) {
      error = e instanceof Error ? e.message : 'فشل التسجيل';
    } finally {
      submitting = false;
    }
  }
</script>

<div class="min-h-[calc(100vh-3.5rem)] flex items-center justify-center px-4">
  <div class="w-full max-w-sm animate-fade-in">
    <div class="text-center mb-8">
      <h1 class="text-3xl font-bold mb-2"><span class="text-amber-400">EGP</span><span class="text-gray-400">EX</span></h1>
      <p class="text-gray-400 text-sm">إنشاء حساب جديد</p>
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
        <input type="password" class="input" placeholder="••••••••" bind:value={password} required minlength={8} />
      </div>

      <div>
        <label class="label">تأكيد كلمة المرور</label>
        <input type="password" class="input" placeholder="••••••••" bind:value={confirm} required />
      </div>

      <button type="submit" class="btn-gold w-full py-2.5" disabled={submitting}>
        {submitting ? 'جاري إنشاء الحساب...' : 'إنشاء حساب'}
      </button>

      <p class="text-center text-sm text-gray-400">
        لديك حساب بالفعل؟ <a href="/login" class="text-amber-400 hover:text-amber-300 transition-colors">تسجيل دخول</a>
      </p>
    </form>
  </div>
</div>
