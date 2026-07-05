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
     error = 'Passwords do not match';
     submitting = false;
     return;
   }
   try {
     const res = await api.post<{ token: string; user: import('$lib/api').User }>('/auth/register', { email, password });
     setToken(res.token);
     currentUser.set(res.user);
     goto('/dashboard');
   } catch (e: unknown) {
     error = e instanceof Error ? e.message : 'Registration failed';
   } finally {
     submitting = false;
   }
 }
</script>

<div class="max-w-sm mx-auto mt-16">
  <h1 class="text-2xl font-bold mb-6 text-center">Create Account</h1>
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
      <input type="password" class="input" bind:value={password} required minlength={8} />
    </div>
    <div>
      <label class="label">Confirm Password</label>
      <input type="password" class="input" bind:value={confirm} required />
    </div>
    <button type="submit" class="btn-primary w-full" disabled={submitting}>
      {submitting ? 'Creating account...' : 'Register'}
    </button>
    <p class="text-center text-sm text-gray-400">
      Already have an account? <a href="/login" class="text-blue-400 hover:underline">Sign in</a>
    </p>
  </form>
</div>
