'use client';

import { Header, Sidebar } from '@/components/layout';
import { useWebSocket, useKeyboardShortcuts } from '@/hooks';

export default function Home() {
  useWebSocket();
  useKeyboardShortcuts();

  return (
    <main className="min-h-screen bg-bg-primary">
      <Header />
      <Sidebar />
      
      <div className="pt-header pl-sidebar">
        <div className="h-[calc(100vh-56px)] flex">
          <div className="flex-1 bg-bg-primary relative">
            {/* Globe view will be rendered here */}
            <div className="absolute inset-0 flex items-center justify-center">
              <div className="text-center">
                <h1 className="text-display-md text-text-primary mb-4">
                  Cherenkov Dashboard
                </h1>
                <p className="text-body-sm text-text-secondary">
                  Real-time radiological intelligence
                </p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </main>
  );
}
