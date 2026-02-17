'use client';

import * as React from 'react';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import * as z from 'zod';
import { motion } from 'framer-motion';
import { useAuthStore } from '@/stores/auth-store';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Card } from '@/components/ui/card';
import { cn } from '@/lib/utils';

const loginSchema = z.object({
  email: z.string().email('Invalid email address'),
  password: z.string().min(8, 'Password must be at least 8 characters'),
});

type LoginFormData = z.infer<typeof loginSchema>;

interface LoginFormProps {
  className?: string;
  onSuccess?: () => void;
}

export const LoginForm = ({ className, onSuccess }: LoginFormProps) => {
  const { login, isLoading, error } = useAuthStore();
  const {
    register,
    handleSubmit,
    formState: { errors },
  } = useForm<LoginFormData>({
    resolver: zodResolver(loginSchema),
  });

  const onSubmit = async (data: LoginFormData) => {
    try {
      await login(data.email, data.password);
      onSuccess?.();
    } catch {
      // Error handled by store
    }
  };

  return (
    <motion.div
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ duration: 0.3 }}
    >
      <Card className={cn('p-8 w-full max-w-md', className)}>
        <div className="text-center mb-8">
          <h1 className="text-display-md text-text-primary mb-2">Cherenkov</h1>
          <p className="text-body-sm text-text-secondary">
            Radiological Intelligence Platform
          </p>
        </div>

        <form onSubmit={handleSubmit(onSubmit)} className="space-y-6">
          <div>
            <label className="block text-heading-xs text-text-secondary mb-2">
              Email
            </label>
            <Input
              type="email"
              placeholder="Enter your email"
              {...register('email')}
              error={errors.email?.message}
            />
          </div>

          <div>
            <label className="block text-heading-xs text-text-secondary mb-2">
              Password
            </label>
            <Input
              type="password"
              placeholder="Enter your password"
              {...register('password')}
              error={errors.password?.message}
            />
          </div>

          {error && (
            <div className="p-3 bg-alert-critical/10 border border-alert-critical/30 rounded text-body-sm text-alert-critical">
              {error}
            </div>
          )}

          <Button
            type="submit"
            className="w-full"
            isLoading={isLoading}
          >
            Sign In
          </Button>
        </form>

        <div className="mt-6 text-center">
          <a
            href="/forgot-password"
            className="text-body-xs text-accent-primary hover:text-accent-secondary transition-colors"
          >
            Forgot password?
          </a>
        </div>
      </Card>
    </motion.div>
  );
};

export default LoginForm;
