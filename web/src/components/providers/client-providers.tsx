'use client';

import { useKeyboardShortcuts } from '@/hooks/use-keyboard-shortcuts';
import { useWebSocket } from '@/hooks/use-websocket';

export function ClientProviders() {
  useKeyboardShortcuts();
  useWebSocket();
  return null;
}
