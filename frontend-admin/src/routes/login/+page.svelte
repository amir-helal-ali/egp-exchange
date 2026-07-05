<script lang="ts">
 import { goto } from '$app/navigation';
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
     if (res.user.role !== 'admin') {
       error = 'Access denied. Admins only.';
       submitting = false;
       return;
     }
     setToken(res.token);
     goto('/deposits');
   } catch (e: unknown) {
     error = e instanceof Error ? e.message : 'Login failed';
   } finally {
     submitting = false;
   }
 }
</script>

<div class="max-w-sm mx-auto mt-16">
  <h1 class="text-2xl font-bold mb-6 text-center">Admin Sign In</h1>
  <form on:submit|preventDefault={handleSubmit} class="card space-y-4">
    {#if error}
      <div class="bg-red-900/50 border border-red-700 text-red-300 rounded-lg p-3 text-sm">{error}</div>
    {/if}
    <div>
      <label class="label">Email</label>
      <input type="email" class="input" bind:value={email} required />
    </div>
    <div>
      <label class="label">Password</label>
      <input type="password" class="input" bind:value={password} required />
    </div>
    <button type="submit" class="btn-primary w-full" disabled={submitting}>
      {submitting ? 'Signing in...' : 'Sign In'}
    </button>
  </form>
</div>
