interface StatCardProps {
  title: string;
  value: string;
  change: string;
  trend: 'up' | 'down' | 'neutral';
  color: 'blue' | 'green' | 'yellow' | 'red';
}

function StatCard(props: StatCardProps) {
  const colorClasses = {
    blue: 'border-[#00d4ff]/30 bg-[#00d4ff]/5',
    green: 'border-green-500/30 bg-green-500/5',
    yellow: 'border-yellow-500/30 bg-yellow-500/5',
    red: 'border-red-500/30 bg-red-500/5',
  };

  const trendIcon = {
    up: '↑',
    down: '↓',
    neutral: '→',
  };

  const trendColor = {
    up: 'text-green-500',
    down: 'text-red-500',
    neutral: 'text-gray-500',
  };

  return (
    <div class={`p-6 rounded-xl border ${colorClasses[props.color]}`}>
      <p class="text-sm text-gray-400 mb-2">{props.title}</p>
      <div class="flex items-end justify-between">
        <p class="text-3xl font-bold text-white">{props.value}</p>
        <div class={`flex items-center gap-1 text-sm ${trendColor[props.trend]}`}>
          <span>{trendIcon[props.trend]}</span>
          <span>{props.change}</span>
        </div>
      </div>
    </div>
  );
}

export default StatCard;
