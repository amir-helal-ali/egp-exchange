<script lang="ts">
  import { isAuthenticated } from '$lib/stores/exchange';
</script>

<div class="min-h-[calc(100vh-3.5rem)] flex flex-col">
  <div class="flex-1 flex flex-col items-center justify-center px-4 text-center relative overflow-hidden">
    <div class="absolute inset-0 overflow-hidden pointer-events-none">
      <div class="absolute -top-40 -right-40 w-80 h-80 bg-amber-500/10 rounded-full blur-3xl"></div>
      <div class="absolute -bottom-40 -left-40 w-80 h-80 bg-blue-500/10 rounded-full blur-3xl"></div>
    </div>

    <div class="relative z-10 max-w-3xl animate-fade-in">
      <div class="inline-flex items-center gap-2 bg-amber-900/20 border border-amber-700/30 rounded-full px-4 py-1.5 mb-6">
        <span class="w-2 h-2 rounded-full bg-amber-400 animate-pulse"></span>
        <span class="text-xs text-amber-300 font-medium">أول منصة مصرية لتداول العملات الرقمية</span>
      </div>

      <h1 class="text-4xl md:text-6xl font-bold mb-6 leading-tight">
        منصة <span class="text-gradient-gold">تداول</span> العملات الرقمية
      </h1>

      <p class="text-lg md:text-xl text-gray-400 mb-8 max-w-2xl mx-auto">
        أول منصة مصرية لتداول العملات الرقمية بالجنيه المصري - تداول فوري، عقود آجلة، وسوق P2P آمن
      </p>

      {#if !$isAuthenticated}
        <div class="flex items-center justify-center gap-4 flex-wrap">
          <a href="/register" class="btn-gold px-8 py-3 text-base font-bold animate-pulse-gold">ابدأ الآن</a>
          <a href="/login" class="btn-secondary px-8 py-3 text-base">تسجيل دخول</a>
        </div>
      {:else}
        <a href="/trade" class="btn-gold px-8 py-3 text-base font-bold animate-pulse-gold">ابدأ التداول</a>
      {/if}
    </div>

    <div class="relative z-10 w-full max-w-5xl mx-auto mt-16 px-4">
      <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
        <div class="card text-center hover:border-amber-500/50 transition-all duration-300 hover:-translate-y-1">
          <div class="w-12 h-12 rounded-xl bg-emerald-900/30 flex items-center justify-center mx-auto mb-3">
            <svg class="w-6 h-6 text-emerald-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7h8m0 0v8m0-8l-8 8-4-4-6 6"/></svg>
          </div>
          <h3 class="text-sm font-semibold text-gray-200 mb-1">تداول فوري</h3>
          <p class="text-xs text-gray-400">أوامر محددة وسوقية مع دفتر أوامر فوري</p>
        </div>

        <div class="card text-center hover:border-amber-500/50 transition-all duration-300 hover:-translate-y-1">
          <div class="w-12 h-12 rounded-xl bg-red-900/30 flex items-center justify-center mx-auto mb-3">
            <svg class="w-6 h-6 text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 17h8m0 0V9m0 8l-8-8-4 4-6-6"/></svg>
          </div>
          <h3 class="text-sm font-semibold text-gray-200 mb-1">عقود آجلة</h3>
          <p class="text-xs text-gray-400">رافعة مالية تصل إلى 125x مع تصفية آلية</p>
        </div>

        <div class="card text-center hover:border-amber-500/50 transition-all duration-300 hover:-translate-y-1">
          <div class="w-12 h-12 rounded-xl bg-blue-900/30 flex items-center justify-center mx-auto mb-3">
            <svg class="w-6 h-6 text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"/></svg>
          </div>
          <h3 class="text-sm font-semibold text-gray-200 mb-1">سوق P2P</h3>
          <p class="text-xs text-gray-400">تداول مباشر بين المستخدمين مع ضمانات</p>
        </div>

        <div class="card text-center hover:border-amber-500/50 transition-all duration-300 hover:-translate-y-1">
          <div class="w-12 h-12 rounded-xl bg-amber-900/30 flex items-center justify-center mx-auto mb-3">
            <svg class="w-6 h-6 text-amber-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"/></svg>
          </div>
          <h3 class="text-sm font-semibold text-gray-200 mb-1">محفظة آمنة</h3>
          <p class="text-xs text-gray-400">إيداع وسحب آمن بالجنيه المصري والعملات الرقمية</p>
        </div>
      </div>
    </div>
  </div>
</div>
