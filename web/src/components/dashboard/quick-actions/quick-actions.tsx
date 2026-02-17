'use client';

import { useRouter } from 'next/navigation';
import { Card } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Globe, AlertTriangle, Activity, Settings } from 'lucide-react';

export const QuickActions = () => {
  const router = useRouter();

  const actions = [
    {
      label: 'View Globe',
      icon: Globe,
      href: '/globe',
      variant: 'default' as const,
    },
    {
      label: 'Check Anomalies',
      icon: AlertTriangle,
      href: '/anomalies',
      variant: 'outline' as const,
    },
    {
      label: 'Sensor Status',
      icon: Activity,
      href: '/sensors',
      variant: 'outline' as const,
    },
    {
      label: 'Settings',
      icon: Settings,
      href: '/settings/general',
      variant: 'ghost' as const,
    },
  ];

  return (
    <Card className="p-4">
      <h2 className="text-heading-sm text-text-primary mb-4">Quick Actions</h2>
      <div className="grid grid-cols-2 gap-3">
        {actions.map((action) => (
          <Button
            key={action.label}
            variant={action.variant}
            className="justify-start gap-2 h-auto py-3"
            onClick={() => router.push(action.href)}
          >
            <action.icon className="w-4 h-4" />
            <span className="text-body-sm">{action.label}</span>
          </Button>
        ))}
      </div>
    </Card>
  );
};
