'use client';

import * as React from 'react';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import * as z from 'zod';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Card } from '@/components/ui/card';
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
