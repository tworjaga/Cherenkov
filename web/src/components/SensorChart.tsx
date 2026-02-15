import { onMount } from 'solid-js';
import Chart from 'chart.js/auto';

function SensorChart() {
  let canvasRef: HTMLCanvasElement | undefined;

  onMount(() => {
    if (!canvasRef) return;
    const ctx = canvasRef.getContext('2d');
    if (!ctx) return;

    const chart = new Chart(ctx, {
      type: 'line',
      data: {
        labels: ['00:00', '04:00', '08:00', '12:00', '16:00', '20:00'],
        datasets: [
          {
            label: 'Global Average (μSv/h)',
            data: [0.15, 0.14, 0.16, 0.18, 0.17, 0.15],
            borderColor: '#00d4ff',
            backgroundColor: 'rgba(0, 212, 255, 0.1)',
            tension: 0.4,
            fill: true,
          },
          {
            label: 'Anomaly Threshold',
            data: [0.5, 0.5, 0.5, 0.5, 0.5, 0.5],
            borderColor: '#ff4757',
            borderDash: [5, 5],
            tension: 0,
            pointRadius: 0,
          },
        ],
      },
      options: {
        responsive: true,
        maintainAspectRatio: false,
        plugins: {
          legend: {
            display: true,
            labels: { color: '#a1a1aa' },
          },
        },
        scales: {
          x: {
            grid: { color: 'rgba(42, 42, 58, 0.5)' },
            ticks: { color: '#a1a1aa' },
          },
          y: {
            grid: { color: 'rgba(42, 42, 58, 0.5)' },
            ticks: { color: '#a1a1aa' },
            beginAtZero: true,
          },
        },
      },
    });

    // Cleanup on unmount
    return () => {
      chart.destroy();
    };
  });

  return (
    <div class="h-64">
      <canvas ref={canvasRef} class="w-full h-full"></canvas>
    </div>
  );
}

export default SensorChart;
