
import Sidebar from './Sidebar';
import Header from './Header';

interface LayoutProps {
  children?: any;
}

function Layout(props: LayoutProps) {
  return (
    <div class="flex h-screen bg-[#0a0a0f] text-gray-200">
      <Sidebar />
      <div class="flex-1 flex flex-col overflow-hidden">
        <Header />
        <main class="flex-1 overflow-auto p-6">
          {props.children}
        </main>
      </div>
    </div>
  );
}

export default Layout;
