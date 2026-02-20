import type { Metadata, Viewport } from 'next';
import { Inter, JetBrains_Mono } from 'next/font/google';
import './globals.css';
import { Header } from '@/components/layout/header';
import { Sidebar } from '@/components/layout/sidebar';
import { RightPanel } from '@/components/layout/right-panel';
import { BottomPanel } from '@/components/layout/bottom-panel';
import { ClientProviders } from '@/components/providers';
import { Toaster } from 'react-hot-toast';
import { SkipLink } from '@/components/ui/skip-link';
import { LayoutProvider } from '@/components/providers/layout-provider';

const inter = Inter({
  subsets: ['latin'],
  variable: '--font-inter',
  display: 'swap',
});

const jetbrainsMono = JetBrains_Mono({
  subsets: ['latin'],
  variable: '--font-jetbrains-mono',
  display: 'swap',
});

export const metadata: Metadata = {
  title: 'Cherenkov - Radiological Intelligence Dashboard',
  description: 'Real-time nuclear radiation monitoring and anomaly detection system',
  keywords: ['radiation', 'monitoring', 'nuclear', 'sensors', 'anomaly detection'],
  authors: [{ name: 'tworjaga' }],
  creator: 'tworjaga',
  metadataBase: new URL('https://cherenkov.app'),
  openGraph: {
    title: 'Cherenkov - Radiological Intelligence Dashboard',
    description: 'Real-time nuclear radiation monitoring and anomaly detection system',
    type: 'website',
  },
};

export const viewport: Viewport = {
  width: 'device-width',
  initialScale: 1,
  maximumScale: 5,
  themeColor: '#12121a',
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html 
      lang="en" 
      className={`${inter.variable} ${jetbrainsMono.variable}`}
      suppressHydrationWarning
    >
      <body className="bg-bg-primary text-text-primary antialiased overflow-hidden touch-manipulation">
        <ClientProviders>
          <LayoutProvider>
            <SkipLink />
            
            <div 
              className="flex flex-col h-screen w-screen"
              role="application"
              aria-label="Cherenkov Radiological Intelligence Dashboard"
            >
              <Header />
              
              <div 
                className="flex flex-1 overflow-hidden relative"
                role="main"
                aria-label="Main content area"
              >
                <Sidebar />
                
                <main 
                  id="main-content"
                  className="flex-1 relative overflow-hidden focus:outline-none focus:ring-2 focus:ring-accent-primary/50"
                  tabIndex={-1}
                >
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
          </LayoutProvider>
        </ClientProviders>
        
        <Toaster 
          position="top-right"
          toastOptions={{
            style: {
              background: '#12121a',
              color: '#ffffff',
              border: '1px solid #2a2a3d',
            },
            ariaProps: {
              role: 'status',
              'aria-live': 'polite',
            },
          }}
        />
      </body>
    </html>
  );
}
