<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { createChart, type IChartApi, type ISeriesApi, type LineData, type Time } from 'lightweight-charts';
  import { api, type Trade } from '$lib/api';
  import { getWs } from '$lib/ws';

  export let base = 'BTC';
  export let quote = 'EGP';

  let chartEl: HTMLDivElement;
  let chart: IChartApi | null = null;
  let candleSeries: ISeriesApi<'Candlestick'> | null = null;
  let volumeSeries: ISeriesApi<'Histogram'> | null = null;
  let ma7Series: ISeriesApi<'Line'> | null = null;
  let ma25Series: ISeriesApi<'Line'> | null = null;
  let ma99Series: ISeriesApi<'Line'> | null = null;
  let trades: Trade[] = [];
  let wsChannel = `trades:${base}${quote}`;
  let timeframe = 60;
  let ma7 = true;
  let ma25 = true;
  let ma99 = false;

  const timeframes = [
    { label: '1m', value: 1 },
    { label: '5m', value: 5 },
    { label: '15m', value: 15 },
    { label: '1h', value: 60 },
    { label: '4h', value: 240 },
    { label: '1d', value: 1440 },
  ];

  function buildCandles() {
    if (trades.length === 0) return [];
    const minutes = timeframe;
    const intervalSec = minutes * 60;
    const now = Math.floor(Date.now() / 1000);
    const cutoff = now - minutes * 60 * 200;
    const recent = trades.filter(t => Math.floor(new Date(t.created_at).getTime() / 1000) > cutoff);
    if (recent.length === 0) return [];

    const intervals: { [key: number]: { open: number; high: number; low: number; close: number; volume: number } } = {};

    for (const t of recent) {
      const ts = Math.floor(new Date(t.created_at).getTime() / 1000);
      const interval = Math.floor(ts / intervalSec) * intervalSec;
      const p = parseFloat(t.price);
      const q = parseFloat(t.quantity);
      if (!intervals[interval]) {
        intervals[interval] = { open: p, high: p, low: p, close: p, volume: q };
      } else {
        const c = intervals[interval];
        c.high = Math.max(c.high, p);
        c.low = Math.min(c.low, p);
        c.close = p;
        c.volume += q;
      }
    }

    return Object.entries(intervals)
      .map(([t, c]) => ({ time: Number(t) as Time, ...c }))
      .sort((a, b) => Number(a.time) - Number(b.time));
  }

  function calcMA(candles: { time: Time; close: number }[], period: number): LineData[] {
    if (candles.length < period) return [];
    const result: LineData[] = [];
    for (let i = period - 1; i < candles.length; i++) {
      let sum = 0;
      for (let j = i - period + 1; j <= i; j++) {
        sum += candles[j].close;
      }
      result.push({ time: candles[i].time, value: sum / period });
    }
    return result;
  }

  function updateMA(candles: { time: Time; close: number }[]) {
    if (!chart) return;
    if (ma7Series) { chart.removeSeries(ma7Series); ma7Series = null; }
    if (ma25Series) { chart.removeSeries(ma25Series); ma25Series = null; }
    if (ma99Series) { chart.removeSeries(ma99Series); ma99Series = null; }

    if (ma7) {
      const data = calcMA(candles, 7);
      if (data.length) {
        ma7Series = chart.addLineSeries({ color: '#f59e0b', lineWidth: 1, priceLineVisible: false, lastValueVisible: false });
        ma7Series.setData(data);
      }
    }
    if (ma25) {
      const data = calcMA(candles, 25);
      if (data.length) {
        ma25Series = chart.addLineSeries({ color: '#3b82f6', lineWidth: 1, priceLineVisible: false, lastValueVisible: false });
        ma25Series.setData(data);
      }
    }
    if (ma99) {
      const data = calcMA(candles, 99);
      if (data.length) {
        ma99Series = chart.addLineSeries({ color: '#8b5cf6', lineWidth: 1, priceLineVisible: false, lastValueVisible: false });
        ma99Series.setData(data);
      }
    }
  }

  function updateChart() {
    if (!chart || !candleSeries || !volumeSeries) return;
    const candles = buildCandles();
    if (candles.length === 0) return;
    candleSeries.setData(candles);
    volumeSeries.setData(
      candles.map(c => ({
        time: c.time,
        value: c.volume,
        color: c.close >= c.open ? 'rgba(16, 185, 129, 0.3)' : 'rgba(239, 68, 68, 0.3)',
      }))
    );
    updateMA(candles);
    chart.timeScale().fitContent();
  }

  function handleNewTrade(data: unknown) {
    const trade = data as Trade;
    trades = [trade, ...trades].slice(0, 10000);
    updateChart();
  }

  function setTimeframe(val: number) {
    timeframe = val;
    updateChart();
  }

  onMount(async () => {
    try {
      trades = await api.get<Trade[]>(`/trades/${base}/${quote}`);
    } catch {}

    chart = createChart(chartEl, {
      layout: {
        background: { color: '#1e293b' },
        textColor: '#94a3b8',
      },
      grid: {
        vertLines: { color: '#334155' },
        horzLines: { color: '#334155' },
      },
      crosshair: {
        mode: 0,
      },
      rightPriceScale: {
        borderColor: '#334155',
      },
      timeScale: {
        borderColor: '#334155',
        timeVisible: false,
        secondsVisible: false,
      },
      width: chartEl.clientWidth,
      height: 400,
    });

    candleSeries = chart.addCandlestickSeries({
      upColor: '#10b981',
      downColor: '#ef4444',
      borderDownColor: '#ef4444',
      borderUpColor: '#10b981',
      wickDownColor: '#ef4444',
      wickUpColor: '#10b981',
    });

    volumeSeries = chart.addHistogramSeries({
      priceFormat: { type: 'volume' },
      priceScaleId: '',
    });
    volumeSeries.priceScale().applyOptions({
      scaleMargins: { top: 0.85, bottom: 0 },
    });

    updateChart();

    const ws = getWs();
    ws.subscribe(wsChannel, handleNewTrade);
    ws.connect();

    const handleResize = () => {
      if (chart && chartEl) {
        chart.applyOptions({ width: chartEl.clientWidth });
      }
    };
    window.addEventListener('resize', handleResize);
  });

  onDestroy(() => {
    const ws = getWs();
    ws.unsubscribe(wsChannel, handleNewTrade);
    if (chart) {
      chart.remove();
      chart = null;
    }
  });
</script>

<div class="card h-full">
  <div class="flex items-center justify-between mb-2 gap-2">
    <h3 class="text-sm font-semibold text-gray-300">{base}/{quote}</h3>
    <div class="flex gap-1">
      {#each timeframes as tf}
        <button
          class="text-[10px] px-2 py-0.5 rounded transition-colors {timeframe === tf.value ? 'bg-amber-600/30 text-amber-400' : 'text-gray-400 hover:bg-dark-600'}"
          on:click={() => setTimeframe(tf.value)}>{tf.label}</button>
      {/each}
    </div>
    <div class="flex gap-2 text-[10px]">
      <label class="flex items-center gap-1 text-gray-400 cursor-pointer">
        <input type="checkbox" bind:checked={ma7} on:change={updateChart} class="accent-amber-500" />
        <span class="text-amber-500">MA7</span>
      </label>
      <label class="flex items-center gap-1 text-gray-400 cursor-pointer">
        <input type="checkbox" bind:checked={ma25} on:change={updateChart} class="accent-blue-500" />
        <span class="text-blue-500">MA25</span>
      </label>
      <label class="flex items-center gap-1 text-gray-400 cursor-pointer">
        <input type="checkbox" bind:checked={ma99} on:change={updateChart} class="accent-purple-500" />
        <span class="text-purple-500">MA99</span>
      </label>
    </div>
  </div>
  <div bind:this={chartEl} class="w-full"></div>
</div>
