'use client';

import * as React from 'react';
import { Button } from '@/components/ui/button';
import { cn } from '@/lib/utils';


interface SettingsFormProps {
  className?: string;
  children: React.ReactNode;
  onSubmit: () => void;
  isLoading?: boolean;
}

export const SettingsForm = ({
  className,
  children,
  onSubmit,
  isLoading,
}: SettingsFormProps) => {
  return (
    <form onSubmit={onSubmit} className={cn('space-y-6', className)}>
      {children}
      <div className="flex justify-end gap-4 pt-4 border-t border-border-subtle">
        <Button type="button" variant="outline">
          Cancel
        </Button>
        <Button type="submit" isLoading={isLoading}>
          Save Changes
        </Button>
      </div>
    </form>
  );
};

export default SettingsForm;
