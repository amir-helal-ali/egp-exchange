<script lang="ts">
  import { onMount, onDestroy, afterUpdate } from 'svelte';
  import { orderbook } from '$lib/stores/exchange';

  export let base = 'BTC';
  export let quote = 'EGP';

  let canvasEl: HTMLCanvasElement;

  function draw() {
    const ctx = canvasEl?.getContext('2d');
    if (!ctx) return;

    const dpr = window.devicePixelRatio || 1;
    const rect = canvasEl.getBoundingClientRect();
    canvasEl.width = rect.width * dpr;
    canvasEl.height = rect.height * dpr;
    ctx.scale(dpr, dpr);

    const w = rect.width;
    const h = rect.height;
    const pad = { top: 20, bottom: 24, left: 60, right: 20 };

    const ob = $orderbook;
    if (!ob || !ob.bids || !ob.asks) return;

    const bids = ob.bids.map(b => ({ price: parseFloat(b.price), qty: parseFloat(b.quantity) })).sort((a, b) => a.price - b.price);
    const asks = ob.asks.map(a => ({ price: parseFloat(a.price), qty: parseFloat(a.quantity) })).sort((a, b) => a.price - b.price);

    if (bids.length === 0 && asks.length === 0) return;

    const allPoints = [...bids, ...asks];
    const minPrice = allPoints[0]?.price ?? 0;
    const maxPrice = allPoints[allPoints.length - 1]?.price ?? 1;
    const priceRange = maxPrice - minPrice || 1;

    let cumBids: { price: number; total: number }[] = [];
    let cumTotal = 0;
    for (const b of bids) {
      cumTotal += b.qty;
      cumBids.push({ price: b.price, total: cumTotal });
    }

    let cumAsks: { price: number; total: number }[] = [];
    cumTotal = 0;
    for (const a of asks) {
      cumTotal += a.qty;
      cumAsks.push({ price: a.price, total: cumTotal });
    }

    const maxCum = Math.max(
      cumBids.length > 0 ? cumBids[cumBids.length - 1].total : 0,
      cumAsks.length > 0 ? cumAsks[cumAsks.length - 1].total : 0
    ) * 1.1 || 1;

    ctx.clearRect(0, 0, w, h);

    ctx.strokeStyle = '#334155';
    ctx.lineWidth = 0.5;
    ctx.fillStyle = '#64748b';
    ctx.font = '9px sans-serif';
    ctx.textAlign = 'right';

    const steps = 5;
    for (let i = 0; i <= steps; i++) {
      const y = pad.top + ((h - pad.top - pad.bottom) * i) / steps;
      ctx.beginPath();
      ctx.moveTo(pad.left, y);
      ctx.lineTo(w - pad.right, y);
      ctx.stroke();
      const vol = (maxCum * (steps - i)) / steps;
      ctx.fillText(vol.toFixed(2), pad.left - 4, y + 3);
    }

    function priceToX(price: number) {
      return pad.left + ((price - minPrice) / priceRange) * (w - pad.left - pad.right);
    }

    function volToY(vol: number) {
      return pad.top + (h - pad.top - pad.bottom) * (1 - vol / maxCum);
    }

    function drawStepLine(points: { price: number; total: number }[], color: string, fillColor: string) {
      if (points.length === 0) return;
      ctx.beginPath();
      ctx.strokeStyle = color;
      ctx.lineWidth = 2;
      ctx.moveTo(priceToX(points[0].price), volToY(0));
      for (const p of points) {
        ctx.lineTo(priceToX(p.price), volToY(p.total));
      }
      ctx.stroke();

      ctx.beginPath();
      ctx.moveTo(priceToX(points[0].price), volToY(0));
      for (const p of points) {
        ctx.lineTo(priceToX(p.price), volToY(p.total));
      }
      ctx.lineTo(priceToX(points[points.length - 1].price), volToY(0));
      ctx.closePath();
      ctx.fillStyle = fillColor;
      ctx.fill();
    }

    drawStepLine(cumBids, '#10b981', 'rgba(16, 185, 129, 0.15)');
    drawStepLine(cumAsks, '#ef4444', 'rgba(239, 68, 68, 0.15)');

    ctx.fillStyle = '#94a3b8';
    ctx.font = '9px sans-serif';
    ctx.textAlign = 'center';
    ctx.fillText('العمق: ' + base, w / 2, h - 4);
  }

  $: if ($orderbook) {
    afterUpdate(draw);
  }

  onMount(() => {
    setTimeout(draw, 100);
  });
</script>

<div class="card">
  <h3 class="text-sm font-semibold text-gray-300 mb-2">العمق السوقي</h3>
  <canvas bind:this={canvasEl} class="w-full h-[200px]"></canvas>
</div>
