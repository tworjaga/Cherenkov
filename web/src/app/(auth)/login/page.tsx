'use client';

import { useState } from 'react';
import { useAuthStore } from '@/stores/auth-store';

export default function LoginPage(): JSX.Element {
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const { login, isLoading, error } = useAuthStore();

  const handleSubmit = async (e: React.FormEvent): Promise<void> => {
    e.preventDefault();
    await login(email, password);
  };

  return (
    <div className="min-h-screen flex items-center justify-center bg-bg-primary">
      <div className="w-full max-w-md p-8 bg-bg-secondary border border-border-default rounded-lg">
        <h1 className="text-display-md font-sans text-text-primary mb-6 text-center">
          Cherenkov
        </h1>
        
        {error && (
          <div className="mb-4 p-3 bg-alert-critical/10 border border-alert-critical rounded text-alert-critical text-body-sm">
            {error}
          </div>
        )}

        <form onSubmit={handleSubmit} className="space-y-4">
          <div>
            <label className="block text-heading-xs text-text-secondary mb-2">
              Email
            </label>
            <input
              type="email"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              className="w-full px-3 py-2 bg-bg-primary border border-border-default rounded text-text-primary focus:border-accent-primary focus:outline-none"
              required
            />
          </div>

          <div>
            <label className="block text-heading-xs text-text-secondary mb-2">
              Password
            </label>
            <input
              type="password"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              className="w-full px-3 py-2 bg-bg-primary border border-border-default rounded text-text-primary focus:border-accent-primary focus:outline-none"
              required
            />
          </div>

          <button
            type="submit"
            disabled={isLoading}
            className="w-full py-2 bg-accent-primary text-bg-primary font-medium rounded hover:bg-accent-secondary disabled:opacity-50"
          >
            {isLoading ? 'Signing in...' : 'Sign In'}
          </button>
        </form>
      </div>
    </div>
  );
}
