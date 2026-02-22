import { Header } from '@/components/layout/header';
import { Sidebar } from '@/components/layout/sidebar';
import { RightPanel } from '@/components/layout/right-panel';
import { BottomPanel } from '@/components/layout/bottom-panel';
import { DataProvider, LayoutProvider } from '@/components/providers';


export default function DashboardLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <DataProvider>
      <LayoutProvider>
        <div className="min-h-screen bg-[#050508] flex flex-col">
          <Header />
          <div className="flex-1 flex overflow-hidden">
            <Sidebar />
            <main className="flex-1 relative overflow-hidden">
              {children}
            </main>
            <RightPanel />
          </div>
          <BottomPanel />
        </div>
      </LayoutProvider>
    </DataProvider>
  );

}
