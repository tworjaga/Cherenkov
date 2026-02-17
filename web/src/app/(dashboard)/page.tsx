import { StatusIndicator } from '@/components/dashboard/status-indicator';
import { StatCards } from '@/components/dashboard/stat-cards';
import { AlertSummary } from '@/components/dashboard/alert-summary';
import { SensorOverview } from '@/components/dashboard/sensor-overview';
import { QuickActions } from '@/components/dashboard/quick-actions';

export default function DashboardPage() {
  return (
    <div className="h-full overflow-auto p-6 space-y-6">
      <div className="flex items-center justify-between">
        <h1 className="text-2xl font-bold text-white">Dashboard</h1>
        <StatusIndicator />
      </div>
      
      <StatCards />
      
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <AlertSummary />
        <SensorOverview />
      </div>
      
      <QuickActions />
    </div>
  );
}
