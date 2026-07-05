<script lang="ts">
 import { onMount, onDestroy, afterUpdate } from 'svelte';
 import { api } from '$lib/api';
 import type { Trade } from '$lib/api';

 export let base = 'BTC';
 export let quote = 'EGP';

 let canvasEl: HTMLCanvasElement;
 let trades: Trade[] = [];

 function parseTrades() {
   const ctx = canvasEl?.getContext('2d');
   if (!ctx || trades.length === 0) return;

   const dpr = window.devicePixelRatio || 1;
   const rect = canvasEl.getBoundingClientRect();
   canvasEl.width = rect.width * dpr;
   canvasEl.height = rect.height * dpr;
   ctx.scale(dpr, dpr);

   const w = rect.width;
   const h = rect.height;
   const pad = { top: 20, bottom: 20, left: 50, right: 20 };

   const prices = trades.map(t => parseFloat(t.price));
   const minP = Math.min(...prices);
   const maxP = Math.max(...prices);
   const range = maxP - minP || 1;

   ctx.clearRect(0, 0, w, h);
   ctx.strokeStyle = '#334155';
   ctx.lineWidth = 0.5;
   for (let i = 0; i <= 4; i++) {
     const y = pad.top + ((h - pad.top - pad.bottom) * i) / 4;
     ctx.beginPath();
     ctx.moveTo(pad.left, y);
     ctx.lineTo(w - pad.right, y);
     ctx.stroke();
     const val = maxP - (range * i) / 4;
     ctx.fillStyle = '#64748b';
     ctx.font = '10px monospace';
     ctx.textAlign = 'right';
     ctx.fillText(val.toFixed(2), pad.left - 5, y + 3);
   }

   const points = trades.slice(-100).map((t, i) => ({
     x: pad.left + ((w - pad.left - pad.right) * i) / Math.min(trades.length - 1, 99),
     y: pad.top + ((h - pad.top - pad.bottom) * (maxP - parseFloat(t.price))) / range,
     color: t.taker_side === 'buy' ? '#00c853' : '#ff1744',
   }));

   ctx.beginPath();
   ctx.strokeStyle = '#3b82f6';
   ctx.lineWidth = 2;
   points.forEach((p, i) => {
     i === 0 ? ctx.moveTo(p.x, p.y) : ctx.lineTo(p.x, p.y);
   });
   ctx.stroke();

   points.forEach(p => {
     ctx.beginPath();
     ctx.arc(p.x, p.y, 2, 0, Math.PI * 2);
     ctx.fillStyle = p.color;
     ctx.fill();
   });
 }

 function updateTrades() {
   api.get<Trade[]>(`/trades/${base}/${quote}`)
     .then(t => { trades = t; })
     .catch(() => {});
 }

 let interval: ReturnType<typeof setInterval>;

 onMount(() => {
   updateTrades();
   interval = setInterval(updateTrades, 2000);
 });

 onDestroy(() => {
   if (interval) clearInterval(interval);
 });

 afterUpdate(parseTrades);
</script>

<div class="card h-full">
  <h3 class="text-sm font-semibold text-gray-300 mb-2">{base}/{quote} Chart</h3>
  <canvas bind:this={canvasEl} class="w-full h-[250px]"></canvas>
</div>
