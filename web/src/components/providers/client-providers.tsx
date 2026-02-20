'use client';

import { ReactNode } from 'react';
import { useKeyboardShortcuts } from '@/hooks/use-keyboard-shortcuts';
import { useWebSocket } from '@/hooks/use-websocket';

interface ClientProvidersProps {
  children: ReactNode;
}

export function ClientProviders({ children }: ClientProvidersProps) {
  useKeyboardShortcuts();
  useWebSocket();
  return <>{children}</>;
}
