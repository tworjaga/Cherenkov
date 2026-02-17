import type { Metadata } from 'next';
import { Inter, JetBrains_Mono } from 'next/font/google';
import './globals.css';
import { Header } from '@/components/layout/header';
import { Sidebar } from '@/components/layout/sidebar';
import { RightPanel } from '@/components/layout/right-panel';
import { BottomPanel } from '@/components/layout/bottom-panel';
import { ClientProviders } from '@/components/providers';
import { Toaster } from 'react-hot-toast';


const inter = Inter({
  subsets: ['latin'],
  variable: '--font-inter',
});

const jetbrainsMono = JetBrains_Mono({
  subsets: ['latin'],
  variable: '--font-jetbrains-mono',
});

export const metadata: Metadata = {
  title: 'Cherenkov - Radiological Intelligence Dashboard',
  description: 'Real-time nuclear radiation monitoring and anomaly detection system',
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en" className={`${inter.variable} ${jetbrainsMono.variable}`}>
      <body className="bg-bg-primary text-text-primary antialiased overflow-hidden">
        <ClientProviders />
        <div className="flex flex-col h-screen">

          <Header />
          
          <div className="flex flex-1 overflow-hidden">
            <Sidebar />
            
            <main className="flex-1 relative overflow-hidden">
              {children}
            </main>
            
            <RightPanel />
          </div>
          
          <BottomPanel 
            globalTimeSeries={[]}
            regionalStats={[]}
            recentEvents={[]}
          />
        </div>
        
        <Toaster 
          position="top-right"
          toastOptions={{
            style: {
              background: '#12121a',
              color: '#ffffff',
              border: '1px solid #2a2a3d',
            },
          }}
        />
      </body>
    </html>
  );
}
