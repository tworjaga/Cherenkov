import { createSignal, onMount } from 'solid-js';

function SensorChart() {
  const [canvasRef, setCanvasRef] = createSignal<HTMLCanvasElement | null>(null);

  onMount(() => {
    const canvas = canvasRef();
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    // Set canvas size
    canvas.width = canvas.offsetWidth;
    canvas.height = canvas.offsetHeight;

    // Draw placeholder chart
    const drawChart = () => {
      ctx.fillStyle = '#0a0a0f';
      ctx.fillRect(0, 0, canvas.width, canvas.height);

      // Draw grid lines
      ctx.strokeStyle = '#2a2a3a';
      ctx.lineWidth = 1;
      for (let i = 0; i < 5; i++) {
        const y = (canvas.height / 4) * i;
        ctx.beginPath();
        ctx.moveTo(0, y);
        ctx.lineTo(canvas.width, y);
        ctx.stroke();
      }

      // Draw sample data line
      ctx.strokeStyle = '#00d4ff';
      ctx.lineWidth = 2;
      ctx.beginPath();
      for (let i = 0; i < canvas.width; i += 5) {
        const y = canvas.height / 2 + Math.sin(i * 0.02) * 50 + Math.random() * 20;
        if (i === 0) {
          ctx.moveTo(i, y);
        } else {
          ctx.lineTo(i, y);
        }
      }
      ctx.stroke();

      // Draw labels
      ctx.fillStyle = '#6b7280';
      ctx.font = '12px sans-serif';
      ctx.fillText('Radiation levels over time', 10, 20);
      ctx.fillText('μSv/h', 10, canvas.height - 10);
    };

    drawChart();

    // Redraw on resize
    const handleResize = () => {
      canvas.width = canvas.offsetWidth;
      canvas.height = canvas.offsetHeight;
      drawChart();
    };

    window.addEventListener('resize', handleResize);
    return () => window.removeEventListener('resize', handleResize);
  });

  return (
    <div class="w-full h-64 bg-[#0a0a0f] rounded-lg border border-[#2a2a3a]">
      <canvas ref={setCanvasRef} class="w-full h-full" />
    </div>
  );
}

export default SensorChart;
